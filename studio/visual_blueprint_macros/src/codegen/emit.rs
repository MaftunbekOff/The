//! Exec-node recursive emission — traverses the exec graph and emits Rust tokens.

use proc_macro2::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use twelfth_visual_blueprint::{ast::VisualScriptGraph, nodes::NodeKind, DataPort};

use super::EmitContext;

pub(super) fn emit_exec_node(
    ctx: &mut EmitContext<'_>,
    graph: &VisualScriptGraph,
    node_id: usize,
) -> Result<TokenStream, String> {
    let node = graph
        .node(node_id)
        .ok_or_else(|| format!("tugun #{node_id} topilmadi"))?
        .clone();

    match node.kind {
        NodeKind::EventBeginPlay | NodeKind::EventTick => {
            let next = graph.exec_successors(node_id, "exec_out");
            if let Some(&child) = next.first() {
                emit_exec_node(ctx, graph, child)
            } else {
                Ok(quote! {})
            }
        }

        NodeKind::CheckGoldAmount => {
            let threshold = node.gold_threshold.unwrap_or(0.0);
            let out_ident = ctx.binding_ident(DataPort { node_id, pin: "result" });
            let check_stmt = quote! {
                let #out_ident = query_gold.iter().next()
                    .map(|g| g.value > #threshold)
                    .unwrap_or(false);
            };
            let next = exec_chain_from(ctx, graph, node_id, "exec_out")?;
            Ok(quote! { #check_stmt #next })
        }

        NodeKind::Branch => {
            let cond       = ctx.resolve_bool(node_id, "condition", &node);
            let true_body  = exec_branch_arm(ctx, graph, node_id, "true")?;
            let false_body = exec_branch_arm(ctx, graph, node_id, "false")?;
            Ok(quote! {
                if #cond { #true_body } else { #false_body }
            })
        }

        NodeKind::PrintLog => {
            let msg = ctx.resolve_string(node_id, "message", &node);
            let next = exec_chain_from(ctx, graph, node_id, "exec_out")?;
            Ok(quote! { ::bevy::prelude::info!(#msg); #next })
        }

        NodeKind::AddGold => {
            let amt = ctx.resolve_float(node_id, "amount", &node);
            let add_stmt = quote! {
                for mut gold in query_gold.iter_mut() { gold.value += #amt; }
            };
            let next = exec_chain_from(ctx, graph, node_id, "exec_out")?;
            Ok(quote! { #add_stmt #next })
        }

        NodeKind::Delay => Err(format!(
            "Delay (#{node_id}) codegen hali qo'llab-quvvatlanmaydi"
        )),

        // ── Pure nodes can't appear in exec chain ─────────────────────────────
        NodeKind::FloatAdd | NodeKind::FloatSubtract | NodeKind::FloatMultiply
        | NodeKind::FloatDivide | NodeKind::FloatMin | NodeKind::FloatMax
        | NodeKind::FloatPow | NodeKind::FloatNegate | NodeKind::FloatAbs
        | NodeKind::FloatSqrt | NodeKind::FloatSin | NodeKind::FloatCos
        | NodeKind::FloatFloor | NodeKind::FloatCeil | NodeKind::FloatRound
        | NodeKind::FloatLerp | NodeKind::FloatClamp
        | NodeKind::FloatGreater | NodeKind::FloatLess | NodeKind::FloatGreaterEqual
        | NodeKind::FloatLessEqual | NodeKind::FloatEqual
        | NodeKind::BoolAnd | NodeKind::BoolOr | NodeKind::BoolNot | NodeKind::BoolXor
        | NodeKind::IsKeyPressed | NodeKind::IsKeyJustPressed
        | NodeKind::Vec3Make | NodeKind::Vec3Add | NodeKind::Vec3Sub
        | NodeKind::Vec3Scale | NodeKind::Vec3Length | NodeKind::Vec3Normalize
        | NodeKind::Vec3Dot | NodeKind::Vec3Lerp
        | NodeKind::StringConcat | NodeKind::FloatToString
        | NodeKind::BoolToString | NodeKind::IntToString
        | NodeKind::IntAdd | NodeKind::IntSubtract | NodeKind::IntMultiply
        | NodeKind::IntDivide | NodeKind::IntModulo
        | NodeKind::FloatToInt | NodeKind::IntToFloat
        | NodeKind::IntGreater | NodeKind::IntLess | NodeKind::IntEqual
        | NodeKind::GetNamedEntity | NodeKind::GetTranslation
        | NodeKind::GetScale | NodeKind::GetRotationEuler
            => Err(format!(
                "tugun #{node_id} ({:?}) sof-hisob — exec zanjirida bo'lishi mumkin emas",
                node.kind
            )),

        // ── Entity exec nodes (PIE only for now) ──────────────────────────────
        NodeKind::SpawnEntity => Err(format!(
            "SpawnEntity (#{node_id}) codegen hali qo'llab-quvvatlanmaydi — faqat PIE da"
        )),
        NodeKind::DestroyEntity => Err(format!(
            "DestroyEntity (#{node_id}) codegen hali qo'llab-quvvatlanmaydi — faqat PIE da"
        )),
        NodeKind::SetTranslation | NodeKind::Translate => {
            let next = exec_chain_from(ctx, graph, node_id, "exec_out")?;
            Ok(quote! { // Transform codegen placeholder — entity query kerak
                #next })
        }
        NodeKind::SetScale | NodeKind::SetRotationEuler => {
            let next = exec_chain_from(ctx, graph, node_id, "exec_out")?;
            Ok(quote! { #next })
        }

        // ── Custom events ──────────────────────────────────────────────────────
        NodeKind::EventCustomBegin => {
            let next = exec_chain_from(ctx, graph, node_id, "exec_out")?;
            Ok(quote! { #next })
        }
        NodeKind::FireCustomEvent => Err(format!(
            "FireCustomEvent (#{node_id}) codegen hali qo'llab-quvvatlanmaydi — faqat PIE da"
        )),

        // ── Yangi tugunlar: Variables ─────────────────────────────────────────
        NodeKind::SetFloatVar => {
            let var_name = node.string_a.as_deref().unwrap_or("unnamed");
            let ident = ctx.var_ident("f", var_name);
            let value = ctx.resolve_float(node_id, "value", &node);
            let decl = if ctx.declared_vars.insert(format!("f:{var_name}")) {
                quote! { let mut #ident: f32 = #value; }
            } else {
                quote! { #ident = #value; }
            };
            let next = exec_chain_from(ctx, graph, node_id, "exec_out").unwrap_or_default();
            Ok(quote! { #decl #next })
        }
        NodeKind::SetBoolVar => {
            let var_name = node.string_a.as_deref().unwrap_or("unnamed");
            let ident = ctx.var_ident("b", var_name);
            let value = ctx.resolve_bool(node_id, "value", &node);
            let decl = if ctx.declared_vars.insert(format!("b:{var_name}")) {
                quote! { let mut #ident: bool = #value; }
            } else {
                quote! { #ident = #value; }
            };
            let next = exec_chain_from(ctx, graph, node_id, "exec_out").unwrap_or_default();
            Ok(quote! { #decl #next })
        }
        NodeKind::SetIntVar => {
            let var_name = node.string_a.as_deref().unwrap_or("unnamed");
            let ident = ctx.var_ident("i", var_name);
            let value = ctx.resolve_int(node_id, "value", &node);
            let decl = if ctx.declared_vars.insert(format!("i:{var_name}")) {
                quote! { let mut #ident: i32 = #value; }
            } else {
                quote! { #ident = #value; }
            };
            let next = exec_chain_from(ctx, graph, node_id, "exec_out").unwrap_or_default();
            Ok(quote! { #decl #next })
        }
        NodeKind::SetStringVar => {
            let var_name = node.string_a.as_deref().unwrap_or("unnamed");
            let ident = ctx.var_ident("s", var_name);
            let value = ctx.resolve_string(node_id, "value", &node);
            let decl = if ctx.declared_vars.insert(format!("s:{var_name}")) {
                quote! { let mut #ident: String = (#value).to_string(); }
            } else {
                quote! { #ident = (#value).to_string(); }
            };
            let next = exec_chain_from(ctx, graph, node_id, "exec_out").unwrap_or_default();
            Ok(quote! { #decl #next })
        }

        // ── Float Collections ─────────────────────────────────────────────────
        NodeKind::FloatArrayPush => {
            let arr_name = node.string_a.as_deref().unwrap_or("unnamed");
            let ident = ctx.var_ident("arr_f", arr_name);
            let value = ctx.resolve_float(node_id, "value", &node);
            let decl = if ctx.declared_vars.insert(format!("arr_f:{arr_name}")) {
                quote! { let mut #ident: Vec<f32> = Vec::new(); }
            } else {
                quote! {}
            };
            let next = exec_chain_from(ctx, graph, node_id, "exec_out").unwrap_or_default();
            Ok(quote! { #decl #ident.push(#value); #next })
        }
        NodeKind::FloatArrayClear => {
            let arr_name = node.string_a.as_deref().unwrap_or("unnamed");
            let ident = ctx.var_ident("arr_f", arr_name);
            let decl = if ctx.declared_vars.insert(format!("arr_f:{arr_name}")) {
                quote! { let mut #ident: Vec<f32> = Vec::new(); }
            } else {
                quote! {}
            };
            let next = exec_chain_from(ctx, graph, node_id, "exec_out").unwrap_or_default();
            Ok(quote! { #decl #ident.clear(); #next })
        }

        // ── Control flow extensions ───────────────────────────────────────────
        NodeKind::Sequence => {
            let then0 = exec_chain_from(ctx, graph, node_id, "then_0").unwrap_or_default();
            let then1 = exec_chain_from(ctx, graph, node_id, "then_1").unwrap_or_default();
            Ok(quote! { { #then0 } { #then1 } })
        }
        NodeKind::DoOnce => {
            let flag = Ident::new(&format!("__DO_ONCE_{node_id}"), proc_macro2::Span::call_site());
            let body = exec_chain_from(ctx, graph, node_id, "exec_out").unwrap_or_default();
            Ok(quote! {
                {
                    static #flag: ::std::sync::atomic::AtomicBool =
                        ::std::sync::atomic::AtomicBool::new(false);
                    if !#flag.swap(true, ::std::sync::atomic::Ordering::Relaxed) {
                        #body
                    }
                }
            })
        }

        // ── Loops ─────────────────────────────────────────────────────────────
        NodeKind::ForEachFloat => {
            let arr_name = node.string_a.as_deref().unwrap_or("unnamed");
            let arr_ident = ctx.var_ident("arr_f", arr_name);
            let item_ident = Ident::new(&format!("__item_{node_id}"), proc_macro2::Span::call_site());
            let idx_ident  = Ident::new(&format!("__idx_{node_id}"),  proc_macro2::Span::call_site());
            let decl = if ctx.declared_vars.insert(format!("arr_f:{arr_name}")) {
                quote! { let mut #arr_ident: Vec<f32> = Vec::new(); }
            } else {
                quote! {}
            };
            ctx.declared_vars.insert(format!("f:item_{node_id}"));
            ctx.declared_vars.insert(format!("i:idx_{node_id}"));
            let body      = exec_chain_from(ctx, graph, node_id, "loop_body").unwrap_or_default();
            let completed = exec_chain_from(ctx, graph, node_id, "completed").unwrap_or_default();
            Ok(quote! {
                #decl
                for (#idx_ident, #item_ident) in #arr_ident.iter().copied().enumerate() {
                    let _ = (#idx_ident, #item_ident);
                    #body
                }
                #completed
            })
        }
        NodeKind::WhileLoop => {
            let cond = ctx.resolve_bool(node_id, "condition", &node);
            let body      = exec_chain_from(ctx, graph, node_id, "loop_body").unwrap_or_default();
            let completed = exec_chain_from(ctx, graph, node_id, "completed").unwrap_or_default();
            Ok(quote! {
                let mut __while_limit = 100_000usize;
                while (#cond) && { __while_limit = __while_limit.saturating_sub(1); __while_limit > 0 } {
                    #body
                }
                #completed
            })
        }

        // ── Pure: error if in exec chain ─────────────────────────────────────
        NodeKind::GetFloatVar | NodeKind::GetBoolVar | NodeKind::GetIntVar | NodeKind::GetStringVar
        | NodeKind::FloatArrayGet | NodeKind::FloatArrayLength
        | NodeKind::RandomFloat | NodeKind::GetGameTime | NodeKind::IsValidEntity
        | NodeKind::SelectFloat | NodeKind::SelectBool | NodeKind::SelectInt | NodeKind::SelectString
        | NodeKind::IntArrayGet | NodeKind::IntArrayLength
        | NodeKind::StringArrayGet | NodeKind::StringArrayLength
        | NodeKind::Vec2Make | NodeKind::Vec2Add | NodeKind::Vec2Sub | NodeKind::Vec2Scale
        | NodeKind::Vec2Length | NodeKind::Vec2Normalize | NodeKind::Vec2Dot
        | NodeKind::Vec2X | NodeKind::Vec2Y
            => Err(format!(
                "tugun #{node_id} ({:?}) sof-hisob — exec zanjirida bo'lishi mumkin emas",
                node.kind
            )),

        // Comment — exec zanjirida bo'lmasligi kerak, lekin xatosiz skip
        NodeKind::Comment => Ok(quote! {}),
        // GetSelfEntity — pure, exec da yo'q
        NodeKind::GetSelfEntity => Err(format!(
            "tugun #{node_id} (GetSelfEntity) sof-hisob — exec zanjirida bo'lishi mumkin emas"
        )),

        // ── New exec nodes ─────────────────────────────────────────────────────
        NodeKind::ContinueLoop => Ok(quote! { continue; }),

        // ── New exec nodes (basic codegen) ─────────────────────────────────────
        NodeKind::BreakLoop => Ok(quote! { break; }),
        NodeKind::ResetDoOnce => {
            let next = exec_chain_from(ctx, graph, node_id, "exec_out").unwrap_or_default();
            Ok(quote! { #next })
        }
        NodeKind::IntArrayPush => {
            let arr_name = node.string_a.as_deref().unwrap_or("unnamed");
            let ident = ctx.var_ident("arr_i", arr_name);
            let value = ctx.resolve_int(node_id, "int_val", &node);
            let decl = if ctx.declared_vars.insert(format!("arr_i:{arr_name}")) {
                quote! { let mut #ident: Vec<i32> = Vec::new(); }
            } else {
                quote! {}
            };
            let next = exec_chain_from(ctx, graph, node_id, "exec_out").unwrap_or_default();
            Ok(quote! { #decl #ident.push(#value); #next })
        }
        NodeKind::IntArrayClear => {
            let arr_name = node.string_a.as_deref().unwrap_or("unnamed");
            let ident = ctx.var_ident("arr_i", arr_name);
            let decl = if ctx.declared_vars.insert(format!("arr_i:{arr_name}")) {
                quote! { let mut #ident: Vec<i32> = Vec::new(); }
            } else {
                quote! {}
            };
            let next = exec_chain_from(ctx, graph, node_id, "exec_out").unwrap_or_default();
            Ok(quote! { #decl #ident.clear(); #next })
        }

        NodeKind::StringArrayPush => {
            let arr_name = node.string_a.as_deref().unwrap_or("unnamed");
            let ident = ctx.var_ident("arr_s", arr_name);
            let value = ctx.resolve_string(node_id, "text", &node);
            let decl = if ctx.declared_vars.insert(format!("arr_s:{arr_name}")) {
                quote! { let mut #ident: Vec<String> = Vec::new(); }
            } else {
                quote! {}
            };
            let next = exec_chain_from(ctx, graph, node_id, "exec_out").unwrap_or_default();
            Ok(quote! { #decl #ident.push(#value.to_string()); #next })
        }
        NodeKind::StringArrayClear => {
            let arr_name = node.string_a.as_deref().unwrap_or("unnamed");
            let ident = ctx.var_ident("arr_s", arr_name);
            let decl = if ctx.declared_vars.insert(format!("arr_s:{arr_name}")) {
                quote! { let mut #ident: Vec<String> = Vec::new(); }
            } else {
                quote! {}
            };
            let next = exec_chain_from(ctx, graph, node_id, "exec_out").unwrap_or_default();
            Ok(quote! { #decl #ident.clear(); #next })
        }

        // ── ECS exec nodes ─────────────────────────────────────────────────────

        NodeKind::AddTag => {
            let tag = node.string_a.as_deref().unwrap_or("Tag");
            let tag_component = Ident::new(
                &format!("Tag{}", tag.replace(' ', "_")),
                proc_macro2::Span::call_site(),
            );
            let next = exec_chain_from(ctx, graph, node_id, "exec_out").unwrap_or_default();
            // entity_expr: connect from entity data source if wired, else self entity
            Ok(quote! {
                // AddTag: AOT — commands.entity(entity).insert(TagName);
                // Entity bağlantısı PIE da ishlaydi; AOT'da ForEachEntity ichida entity o'zgaruvchi orqali
                let __tag_entity = *entity_ref;
                commands.entity(__tag_entity).insert(#tag_component);
                #next
            })
        }

        NodeKind::RemoveTag => {
            let tag = node.string_a.as_deref().unwrap_or("Tag");
            let tag_component = Ident::new(
                &format!("Tag{}", tag.replace(' ', "_")),
                proc_macro2::Span::call_site(),
            );
            let next = exec_chain_from(ctx, graph, node_id, "exec_out").unwrap_or_default();
            Ok(quote! {
                let __tag_entity = *entity_ref;
                commands.entity(__tag_entity).remove::<#tag_component>();
                #next
            })
        }

        NodeKind::ForEachEntity => {
            let loop_body = exec_chain_from(ctx, graph, node_id, "loop_body").unwrap_or_default();
            let completed = exec_chain_from(ctx, graph, node_id, "completed").unwrap_or_default();
            let index_var = Ident::new(
                &format!("__idx_{node_id}"),
                proc_macro2::Span::call_site(),
            );
            let entity_var = Ident::new(
                &format!("__entity_{node_id}"),
                proc_macro2::Span::call_site(),
            );
            Ok(quote! {
                for (#index_var, #entity_var) in query_all_entities.iter().enumerate() {
                    let _ = #index_var;
                    let _ = #entity_var;
                    #loop_body
                }
                #completed
            })
        }

        // ECS pure nodes used only as data sources — not valid as exec root
        NodeKind::EventFixedTick | NodeKind::EventOnSpawn
            => Ok(quote! {}),

        NodeKind::QueryAllEntities | NodeKind::QueryByTag
        | NodeKind::EntityArrayGet  | NodeKind::EntityArrayLength
        | NodeKind::HasTag          | NodeKind::GetEntityName
            => Err(format!(
                "tugun #{node_id} ({:?}) sof-hisob — exec zanjirida bo'lishi mumkin emas",
                node.kind
            )),
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn exec_chain_from(
    ctx: &mut EmitContext<'_>,
    graph: &VisualScriptGraph,
    from: usize,
    from_pin: &str,
) -> Result<TokenStream, String> {
    let children = graph.exec_successors(from, from_pin);
    if let Some(&child) = children.first() {
        if children.len() > 1 {
            return Err(format!(
                "tugun #{from} `{from_pin}`: bitta dan ortiq exec chiqish qo'llab-quvvatlanmaydi"
            ));
        }
        emit_exec_node(ctx, graph, child)
    } else {
        Ok(quote! {})
    }
}

fn exec_branch_arm(
    ctx: &mut EmitContext<'_>,
    graph: &VisualScriptGraph,
    branch_id: usize,
    arm: &str,
) -> Result<TokenStream, String> {
    let children = graph.exec_successors(branch_id, arm);
    let mut stmts = Vec::new();
    for child in children {
        stmts.push(emit_exec_node(ctx, graph, child)?);
    }
    Ok(quote! { #( #stmts )* })
}
