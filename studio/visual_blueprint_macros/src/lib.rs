//! `visual_blueprint!` — graf DSL → compile-time Bevy tizimlari.

mod codegen;

use proc_macro::TokenStream;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    Ident, LitFloat, LitStr, Token,
};

use twelfth_visual_blueprint::{
    ast::{ExecLink, VisualScriptGraph},
    nodes::VisualNode,
    pins::{DataLink, PinType},
    validate::validate_data_link,
};

use crate::codegen::{emit_plugin, ScriptChains};

/// ```ignore
/// visual_blueprint! {
///     script MyPlugin;
///     startup {
///         let rich = check_gold 1000.0;
///         branch rich {
///             arm_true { log "Rich!"; }
///             arm_false { log "Need more gold!"; }
///         }
///     }
/// }
/// ```
#[proc_macro]
pub fn visual_blueprint(input: TokenStream) -> TokenStream {
    match parse_and_emit(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn parse_and_emit(input: TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let spec = syn::parse::<ScriptSpec>(input)?;
    let chains = spec.into_chains()?;
    emit_plugin(&chains)
        .map_err(|msg| syn::Error::new(proc_macro2::Span::call_site(), msg))
}

struct ScriptSpec {
    plugin_name: Ident,
    startup: DslBlock,
    update: DslBlock,
}

struct DslBlock {
    stmts: Vec<Stmt>,
}

#[derive(Clone)]
struct DataRef {
    node_id: usize,
    pin: &'static str,
}

enum BranchCondition {
    DataRef(Ident),
    Inline(bool),
}

enum Stmt {
    LetCheckGold {
        name: Ident,
        threshold: LitFloat,
    },
    Branch {
        condition: BranchCondition,
        true_arm: DslBlock,
        false_arm: DslBlock,
    },
    Log(LitStr),
    AddGold(LitFloat),
}

struct GraphBuilder {
    graph: VisualScriptGraph,
    next_id: usize,
    last_exec: Option<usize>,
    bindings: std::collections::HashMap<String, DataRef>,
}

impl GraphBuilder {
    fn new(event: VisualNode) -> Self {
        let entry = 0usize;
        Self {
            graph: VisualScriptGraph::new(String::new(), vec![(entry, event)], Vec::new(), Vec::new()),
            next_id: 1,
            last_exec: Some(entry),
            bindings: std::collections::HashMap::new(),
        }
    }

    fn alloc(&mut self, node: VisualNode) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.graph.nodes.push((id, node));
        id
    }

    fn link_exec(&mut self, from: usize, from_pin: &'static str, to: usize) {
        self.graph.exec_links.push(ExecLink {
            from_node_id: from,
            from_pin,
            to_node_id: to,
            to_pin: "exec_in",
        });
        self.last_exec = Some(to);
    }

    fn append_exec_linear(&mut self, node: VisualNode) -> usize {
        let id = self.alloc(node);
        if let Some(from) = self.last_exec {
            self.link_exec(from, "exec_out", id);
        }
        self.last_exec = Some(id);
        id
    }

    fn append_exec_from(&mut self, from: usize, from_pin: &'static str, node: VisualNode) -> usize {
        let id = self.alloc(node);
        self.link_exec(from, from_pin, id);
        id
    }

    fn link_data(&mut self, from: DataRef, to_node: usize, to_pin: &'static str) -> syn::Result<()> {
        let from_node = self
            .graph
            .node(from.node_id)
            .ok_or_else(|| syn::Error::new(proc_macro2::Span::call_site(), "ichki xato"))?;
        let to_n = self
            .graph
            .node(to_node)
            .ok_or_else(|| syn::Error::new(proc_macro2::Span::call_site(), "ichki xato"))?;
        let from_ty = from_node
            .data_output_type(from.pin)
            .ok_or_else(|| syn::Error::new(proc_macro2::Span::call_site(), "chiqish yo‘q"))?;
        let to_ty = to_n
            .data_input_type(to_pin)
            .ok_or_else(|| syn::Error::new(proc_macro2::Span::call_site(), "kirish yo‘q"))?;
        validate_data_link(from_ty, to_ty, "DSL data simi")
            .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e.message))?;
        self.graph.data_links.push(DataLink {
            from_node_id: from.node_id,
            from_pin: from.pin,
            to_node_id: to_node,
            to_pin,
        });
        Ok(())
    }

    fn build_block(
        &mut self,
        block: &DslBlock,
        fork: Option<(usize, &'static str)>,
    ) -> syn::Result<()> {
        let mut fork = fork;
        for stmt in &block.stmts {
            let attach = fork.take();
            match stmt {
                Stmt::LetCheckGold { name, threshold } => {
                    let amount: f32 = parse_float(threshold)?;
                    validate_data_link(PinType::Float, PinType::Float, "check_gold threshold")
                        .map_err(|e| syn::Error::new(threshold.span(), e.message))?;
                    let id = match attach {
                        Some((from, pin)) => {
                            self.append_exec_from(from, pin, VisualNode::check_gold_amount(amount))
                        }
                        None => self.append_exec_linear(VisualNode::check_gold_amount(amount)),
                    };
                    self.bindings.insert(
                        name.to_string(),
                        DataRef {
                            node_id: id,
                            pin: "result",
                        },
                    );
                }
                Stmt::Branch {
                    condition,
                    true_arm,
                    false_arm,
                } => {
                    let branch_id = match attach {
                        Some((from, pin)) => {
                            self.append_exec_from(from, pin, VisualNode::branch())
                        }
                        None => self.append_exec_linear(VisualNode::branch()),
                    };
                    match condition {
                        BranchCondition::DataRef(condition) => {
                            let cond_ref = self
                                .bindings
                                .get(&condition.to_string())
                                .cloned()
                                .ok_or_else(|| {
                                    syn::Error::new(
                                        condition.span(),
                                        format!(
                                            "`{}` topilmadi — avval `let {0} = check_gold ...` deb eʼlon qiling",
                                            condition
                                        ),
                                    )
                                })?;
                            validate_data_link(PinType::Bool, PinType::Bool, "branch sharti")
                                .map_err(|e| syn::Error::new(condition.span(), e.message))?;
                            self.link_data(cond_ref, branch_id, "condition")?;
                        }
                        BranchCondition::Inline(value) => {
                            if let Some((_, node)) = self
                                .graph
                                .nodes
                                .iter_mut()
                                .find(|(id, _)| *id == branch_id)
                            {
                                node.condition_value = Some(*value);
                            }
                        }
                    }
                    self.build_block(true_arm, Some((branch_id, "true")))?;
                    self.build_block(false_arm, Some((branch_id, "false")))?;
                    // Branch shoxlari tugagach, asosiy exec zanjir tugaydi (merge yo‘q).
                    self.last_exec = None;
                }
                Stmt::Log(lit) => {
                    validate_data_link(PinType::String, PinType::String, "log")
                        .map_err(|e| syn::Error::new(lit.span(), e.message))?;
                    match attach {
                        Some((from, pin)) => {
                            self.append_exec_from(from, pin, VisualNode::print_log(lit.value()));
                        }
                        None => {
                            self.append_exec_linear(VisualNode::print_log(lit.value()));
                        }
                    }
                }
                Stmt::AddGold(lit) => {
                    let amount = parse_float(lit)?;
                    validate_data_link(PinType::Float, PinType::Float, "add_gold")
                        .map_err(|e| syn::Error::new(lit.span(), e.message))?;
                    match attach {
                        Some((from, pin)) => {
                            self.append_exec_from(from, pin, VisualNode::add_gold(amount));
                        }
                        None => {
                            self.append_exec_linear(VisualNode::add_gold(amount));
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

fn parse_float(lit: &LitFloat) -> syn::Result<f32> {
    lit.base10_parse()
        .map_err(|_| syn::Error::new(lit.span(), "kutilgan `f32` litreal"))
}

impl Parse for ScriptSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let kw: Ident = input.parse()?;
        if kw != "script" {
            return Err(syn::Error::new(kw.span(), "kutilgan `script`"));
        }
        let plugin_name: Ident = input.parse()?;
        if input.peek(Token![;]) {
            input.parse::<Token![;]>()?;
        }
        let mut startup = DslBlock { stmts: vec![] };
        let mut update = DslBlock { stmts: vec![] };
        while !input.is_empty() {
            let branch: Ident = input.parse()?;
            let content;
            braced!(content in input);
            let block = parse_block(&content)?;
            if branch == "startup" {
                startup = block;
            } else if branch == "update" {
                update = block;
            } else {
                return Err(syn::Error::new(branch.span(), "`startup` yoki `update`"));
            }
        }
        Ok(Self {
            plugin_name,
            startup,
            update,
        })
    }
}

fn parse_block(input: ParseStream) -> syn::Result<DslBlock> {
    let mut stmts = Vec::new();
    while !input.is_empty() {
        if input.peek(Token![let]) {
            input.parse::<Token![let]>()?;
            let name: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let op: Ident = input.parse()?;
            if op == "check_gold" {
                let threshold: LitFloat = input.parse()?;
                stmts.push(Stmt::LetCheckGold { name, threshold });
            } else {
                return Err(syn::Error::new(op.span(), "kutilgan `check_gold`"));
            }
        } else if input.peek(Ident) {
            let head: Ident = input.parse()?;
            if head == "branch" {
                // `true`/`false` kalit so'z — LitBool, boshqasi Ident
                let condition = if input.peek(syn::LitBool) {
                    let lit: syn::LitBool = input.parse()?;
                    BranchCondition::Inline(lit.value)
                } else {
                    let cond_kw: Ident = input.parse()?;
                    BranchCondition::DataRef(cond_kw)
                };
                let branch_body;
                syn::braced!(branch_body in input);
                let mut true_stmts = DslBlock { stmts: vec![] };
                let mut false_stmts = DslBlock { stmts: vec![] };
                while !branch_body.is_empty() {
                    let arm_kw: Ident = branch_body.parse()?;
                    let arm_body;
                    syn::braced!(arm_body in branch_body);
                    let block = parse_block(&arm_body)?;
                    if arm_kw == "arm_true" {
                        true_stmts = block;
                    } else if arm_kw == "arm_false" {
                        false_stmts = block;
                    } else {
                        return Err(syn::Error::new(
                            arm_kw.span(),
                            "kutilgan `arm_true` yoki `arm_false`",
                        ));
                    }
                }
                stmts.push(Stmt::Branch {
                    condition,
                    true_arm: true_stmts,
                    false_arm: false_stmts,
                });
            } else if head == "log" {
                let msg: LitStr = input.parse()?;
                stmts.push(Stmt::Log(msg));
            } else if head == "add_gold" {
                let amount: LitFloat = input.parse()?;
                stmts.push(Stmt::AddGold(amount));
            } else {
                return Err(syn::Error::new(
                    head.span(),
                    "noma’lum: log | add_gold | branch | let ... = check_gold",
                ));
            }
        }
        if input.peek(Token![;]) {
            input.parse::<Token![;]>()?;
        }
    }
    Ok(DslBlock { stmts })
}

impl ScriptSpec {
    fn into_chains(self) -> syn::Result<ScriptChains> {
        let mut startup_builder = GraphBuilder::new(VisualNode::event_begin_play());
        startup_builder.build_block(&self.startup, None)?;

        let mut update_graph = VisualScriptGraph::default();
        if !self.update.stmts.is_empty() {
            let mut update_builder = GraphBuilder::new(VisualNode::event_tick());
            update_builder.build_block(&self.update, None)?;
            update_graph = update_builder.graph;
        }

        Ok(ScriptChains {
            plugin_name: self.plugin_name.to_string(),
            startup: startup_builder.graph,
            update: update_graph,
        })
    }
}
