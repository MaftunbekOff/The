//! Code generation core: structs, plugin scaffolding, system emission.

mod emit;
mod resolve;

use std::collections::{HashMap, HashSet};

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;
use twelfth_visual_blueprint::{
    ast::VisualScriptGraph,
    validate::validate_graph,
    DataPort,
};

// ── Public types ──────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Default)]
pub struct ScriptChains {
    pub plugin_name: String,
    pub startup: VisualScriptGraph,
    pub update: VisualScriptGraph,
}

// ── EmitContext ───────────────────────────────────────────────────────────────

pub(self) struct EmitContext<'a> {
    pub(self) graph: &'a VisualScriptGraph,
    /// `(node_id, pin)` → ident for exec-node data bindings.
    pub(self) bindings: HashMap<(usize, &'static str), Ident>,
    /// Qaysi lokal o'zgaruvchilar e'lon qilinganini kuzatadi (var_name → true).
    pub(self) declared_vars: HashSet<String>,
}

impl<'a> EmitContext<'a> {
    pub(self) fn new(graph: &'a VisualScriptGraph) -> Self {
        Self {
            graph,
            bindings: HashMap::new(),
            declared_vars: HashSet::new(),
        }
    }

    pub(self) fn binding_ident(&mut self, port: DataPort) -> Ident {
        let key = (port.node_id, port.pin);
        if let Some(id) = self.bindings.get(&key) {
            return id.clone();
        }
        let name  = format!("node_{}_{}", port.node_id, port.pin);
        let ident = Ident::new(&name, proc_macro2::Span::call_site());
        self.bindings.insert(key, ident.clone());
        ident
    }

    /// O'zgaruvchi nomi uchun Rust identifikatori: `__var_{prefix}_{sanitized_name}`.
    pub(self) fn var_ident(&self, prefix: &str, name: &str) -> Ident {
        let sanitized: String = name.chars()
            .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
            .collect();
        let raw = format!("__var_{}_{}", prefix, sanitized);
        let key = if raw.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
            format!("_{raw}")
        } else {
            raw
        };
        Ident::new(&key, proc_macro2::Span::call_site())
    }
}

// ── Public API ────────────────────────────────────────────────────────────────

pub fn emit_plugin(chains: &ScriptChains) -> Result<TokenStream, String> {
    validate_graph(&chains.startup).map_err(|e| e.message)?;
    if !chains.update.nodes.is_empty() {
        validate_graph(&chains.update).map_err(|e| e.message)?;
    }

    let plugin_ident = Ident::new(&chains.plugin_name, proc_macro2::Span::call_site());
    let startup_entry = chains.startup.begin_play_entry()?;
    let startup_system =
        emit_schedule_system("generated_begin_play_system", &chains.startup, startup_entry)?;

    let update_system = if chains.update.nodes.is_empty() {
        None
    } else {
        let entry = chains.update.tick_entry()?;
        Some(emit_schedule_system("generated_tick_system", &chains.update, entry)?)
    };

    let tokens = if let Some(update_fn) = update_system {
        quote! {
            #startup_system
            #update_fn
            pub struct #plugin_ident;
            impl ::bevy::prelude::Plugin for #plugin_ident {
                fn build(&self, app: &mut ::bevy::prelude::App) {
                    app.add_systems(::bevy::prelude::Startup, generated_begin_play_system);
                    app.add_systems(::bevy::prelude::Update, generated_tick_system);
                }
            }
        }
    } else {
        quote! {
            #startup_system
            pub struct #plugin_ident;
            impl ::bevy::prelude::Plugin for #plugin_ident {
                fn build(&self, app: &mut ::bevy::prelude::App) {
                    app.add_systems(::bevy::prelude::Startup, generated_begin_play_system);
                }
            }
        }
    };

    Ok(tokens)
}

#[cfg(test)]
pub fn emit_plugin_from_graph(graph: &VisualScriptGraph) -> Result<TokenStream, String> {
    validate_graph(graph).map_err(|e| e.message)?;
    let plugin_name = if graph.name.is_empty() {
        "GeneratedVisualScriptPlugin".to_string()
    } else {
        format!("{}Plugin", to_pascal_case(&graph.name))
    };
    let chains = ScriptChains {
        plugin_name,
        startup: graph.clone(),
        update: VisualScriptGraph::default(),
    };
    emit_plugin(&chains)
}

fn emit_schedule_system(
    fn_name: &str,
    graph: &VisualScriptGraph,
    entry: usize,
) -> Result<TokenStream, String> {
    let fn_ident = Ident::new(fn_name, proc_macro2::Span::call_site());
    let mut ctx = EmitContext::new(graph);
    let body = emit::emit_exec_node(&mut ctx, graph, entry)?;

    if graph.needs_gold_query() {
        Ok(quote! {
            pub fn #fn_ident(mut query_gold: ::bevy::prelude::Query<&mut ::twelfth_visual_blueprint::Gold>) {
                #body
            }
        })
    } else {
        Ok(quote! {
            pub fn #fn_ident() {
                #body
            }
        })
    }
}

#[cfg(test)]
fn to_pascal_case(s: &str) -> String {
    s.split(|c: char| !c.is_alphanumeric())
        .filter(|p| !p.is_empty())
        .map(|p| {
            let mut c = p.chars();
            match c.next() {
                None    => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn branch_graph_emits_if_and_single_query() {
        let graph = twelfth_visual_blueprint::demo_rich_branch_graph();
        let tokens = emit_plugin_from_graph(&graph).expect("emit");
        let code = tokens.to_string();
        assert!(code.contains("if"));
        assert!(code.contains("Rich"));
        assert!(code.contains("query_gold"));
        assert_eq!(code.matches("Query").count(), 1);
    }

    #[test]
    fn float_add_emits_inline_expr() {
        use twelfth_visual_blueprint::{
            ast::{ExecLink, VisualScriptGraph},
            nodes::VisualNode,
            pins::DataLink,
        };
        let graph = VisualScriptGraph::new(
            "MathTest",
            vec![
                (0, VisualNode::event_begin_play()),
                (1, VisualNode::add_gold(0.0)),
                (2, VisualNode::float_add(1.0, 2.0)),
            ],
            vec![ExecLink::exec_out(0, 1)],
            vec![DataLink {
                from_node_id: 2,
                from_pin: "result",
                to_node_id: 1,
                to_pin: "amount",
            }],
        );
        let tokens = emit_plugin_from_graph(&graph).expect("emit");
        let code = tokens.to_string();
        assert!(code.contains("+"), "inline add missing: {code}");
    }
}
