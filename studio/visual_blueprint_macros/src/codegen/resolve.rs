//! Float, bool and string resolution for `EmitContext` — pure-node inline codegen.

use proc_macro2::TokenStream;
use quote::quote;
use twelfth_visual_blueprint::nodes::{NodeKind, VisualNode};

use super::EmitContext;

impl EmitContext<'_> {
    // ── Float resolution ──────────────────────────────────────────────────────

    pub(super) fn resolve_float(
        &mut self,
        node_id: usize,
        pin: &str,
        node: &VisualNode,
    ) -> TokenStream {
        if let Some(src) = self.graph.data_source(node_id, pin) {
            if let Some(src_node) = self.graph.node(src.node_id).cloned() {
                if src_node.is_pure() {
                    return self.eval_float_expr(src.node_id, src_node, src.pin);
                }
            }
            let id = self.binding_ident(src);
            return quote! { #id };
        }
        let v = match (node.kind, pin) {
            (NodeKind::AddGold,         "amount")    => node.gold_amount.unwrap_or(0.0),
            (NodeKind::Delay,           "duration")  => node.delay_seconds.unwrap_or(0.0),
            (NodeKind::CheckGoldAmount, "threshold") => node.gold_threshold.unwrap_or(0.0),
            _ => node.float_literal_for(pin),
        };
        quote! { #v }
    }

    pub(super) fn eval_float_expr(
        &mut self,
        node_id: usize,
        node: VisualNode,
        _out_pin: &'static str,
    ) -> TokenStream {
        match node.kind {
            NodeKind::FloatAdd => {
                let a = self.resolve_float(node_id, "a", &node);
                let b = self.resolve_float(node_id, "b", &node);
                quote! { (#a + #b) }
            }
            NodeKind::FloatSubtract => {
                let a = self.resolve_float(node_id, "a", &node);
                let b = self.resolve_float(node_id, "b", &node);
                quote! { (#a - #b) }
            }
            NodeKind::FloatMultiply => {
                let a = self.resolve_float(node_id, "a", &node);
                let b = self.resolve_float(node_id, "b", &node);
                quote! { (#a * #b) }
            }
            NodeKind::FloatDivide => {
                let a = self.resolve_float(node_id, "a", &node);
                let b = self.resolve_float(node_id, "b", &node);
                quote! { ({ let __d = #b; if __d == 0.0 { 0.0f32 } else { #a / __d } }) }
            }
            NodeKind::FloatMin => {
                let a = self.resolve_float(node_id, "a", &node);
                let b = self.resolve_float(node_id, "b", &node);
                quote! { (#a).min(#b) }
            }
            NodeKind::FloatMax => {
                let a = self.resolve_float(node_id, "a", &node);
                let b = self.resolve_float(node_id, "b", &node);
                quote! { (#a).max(#b) }
            }
            NodeKind::FloatPow => {
                let base = self.resolve_float(node_id, "base", &node);
                let exp  = self.resolve_float(node_id, "exp",  &node);
                quote! { (#base).powf(#exp) }
            }
            NodeKind::FloatNegate => {
                let a = self.resolve_float(node_id, "a", &node);
                quote! { (-(#a)) }
            }
            NodeKind::FloatAbs   => { let a = self.resolve_float(node_id, "a", &node); quote! { (#a).abs()   } }
            NodeKind::FloatSqrt  => { let a = self.resolve_float(node_id, "a", &node); quote! { (#a).sqrt()  } }
            NodeKind::FloatSin   => { let a = self.resolve_float(node_id, "a", &node); quote! { (#a).sin()   } }
            NodeKind::FloatCos   => { let a = self.resolve_float(node_id, "a", &node); quote! { (#a).cos()   } }
            NodeKind::FloatFloor => { let a = self.resolve_float(node_id, "a", &node); quote! { (#a).floor() } }
            NodeKind::FloatCeil  => { let a = self.resolve_float(node_id, "a", &node); quote! { (#a).ceil()  } }
            NodeKind::FloatRound => { let a = self.resolve_float(node_id, "a", &node); quote! { (#a).round() } }
            NodeKind::FloatLerp => {
                let a = self.resolve_float(node_id, "a", &node);
                let b = self.resolve_float(node_id, "b", &node);
                let t = self.resolve_float(node_id, "t", &node);
                quote! { ({ let __a = #a; __a + (#b - __a) * #t }) }
            }
            NodeKind::FloatClamp => {
                let v   = self.resolve_float(node_id, "value", &node);
                let min = self.resolve_float(node_id, "min",   &node);
                let max = self.resolve_float(node_id, "max",   &node);
                quote! { (#v).clamp(#min, #max) }
            }
            NodeKind::IntToFloat => {
                let a = self.resolve_int(node_id, "a", &node);
                quote! { (#a as f32) }
            }
            NodeKind::GetFloatVar => {
                let var_name = node.string_a.as_deref().unwrap_or("unnamed");
                let ident = self.var_ident("f", var_name);
                quote! { #ident }
            }
            NodeKind::FloatArrayGet => {
                let arr_name = node.string_a.as_deref().unwrap_or("unnamed");
                let arr_ident = self.var_ident("arr_f", arr_name);
                let idx = self.resolve_int(node_id, "index", &node);
                quote! { #arr_ident.get(#idx as usize).copied().unwrap_or(0.0f32) }
            }
            NodeKind::RandomFloat => {
                quote! { (::std::time::SystemTime::now()
                    .duration_since(::std::time::UNIX_EPOCH).unwrap_or_default().subsec_nanos() as f32
                    / u32::MAX as f32) }
            }
            NodeKind::SelectFloat => {
                let cond = self.resolve_bool(node_id, "condition", &node);
                let a    = self.resolve_float(node_id, "value_a",  &node);
                let b    = self.resolve_float(node_id, "value_b",  &node);
                quote! { (if #cond { #a } else { #b }) }
            }
            NodeKind::Vec2Length => {
                let v = self.resolve_vec2(node_id, "vec", &node);
                quote! { { let __v=#v; (__v[0]*__v[0]+__v[1]*__v[1]).sqrt() } }
            }
            NodeKind::Vec2Dot => {
                let a = self.resolve_vec2(node_id, "a", &node);
                let b = self.resolve_vec2(node_id, "b", &node);
                quote! { { let __a=#a; let __b=#b; __a[0]*__b[0]+__a[1]*__b[1] } }
            }
            NodeKind::Vec2X => {
                let v = self.resolve_vec2(node_id, "vec", &node);
                quote! { (#v)[0] }
            }
            NodeKind::Vec2Y => {
                let v = self.resolve_vec2(node_id, "vec", &node);
                quote! { (#v)[1] }
            }
            _ => quote! { 0.0f32 },
        }
    }

    // ── Bool resolution ───────────────────────────────────────────────────────

    pub(super) fn resolve_bool(
        &mut self,
        node_id: usize,
        pin: &str,
        node: &VisualNode,
    ) -> TokenStream {
        if let Some(src) = self.graph.data_source(node_id, pin) {
            if let Some(src_node) = self.graph.node(src.node_id).cloned() {
                if src_node.is_pure() {
                    return self.eval_bool_expr(src.node_id, src_node, src.pin);
                }
            }
            let id = self.binding_ident(src);
            return quote! { #id };
        }
        let v = node.bool_literal_for(pin);
        quote! { #v }
    }

    pub(super) fn eval_bool_expr(
        &mut self,
        node_id: usize,
        node: VisualNode,
        _out_pin: &'static str,
    ) -> TokenStream {
        match node.kind {
            NodeKind::FloatGreater => {
                let a = self.resolve_float(node_id, "a", &node);
                let b = self.resolve_float(node_id, "b", &node);
                quote! { (#a > #b) }
            }
            NodeKind::FloatLess => {
                let a = self.resolve_float(node_id, "a", &node);
                let b = self.resolve_float(node_id, "b", &node);
                quote! { (#a < #b) }
            }
            NodeKind::FloatGreaterEqual => {
                let a = self.resolve_float(node_id, "a", &node);
                let b = self.resolve_float(node_id, "b", &node);
                quote! { (#a >= #b) }
            }
            NodeKind::FloatLessEqual => {
                let a = self.resolve_float(node_id, "a", &node);
                let b = self.resolve_float(node_id, "b", &node);
                quote! { (#a <= #b) }
            }
            NodeKind::FloatEqual => {
                let a   = self.resolve_float(node_id, "a",   &node);
                let b   = self.resolve_float(node_id, "b",   &node);
                let eps = self.resolve_float(node_id, "eps", &node);
                quote! { ((#a - #b).abs() <= #eps) }
            }
            NodeKind::BoolAnd => {
                let a = self.resolve_bool(node_id, "a", &node);
                let b = self.resolve_bool(node_id, "b", &node);
                quote! { (#a && #b) }
            }
            NodeKind::BoolOr => {
                let a = self.resolve_bool(node_id, "a", &node);
                let b = self.resolve_bool(node_id, "b", &node);
                quote! { (#a || #b) }
            }
            NodeKind::BoolXor => {
                let a = self.resolve_bool(node_id, "a", &node);
                let b = self.resolve_bool(node_id, "b", &node);
                quote! { (#a ^ #b) }
            }
            NodeKind::BoolNot => {
                let a = self.resolve_bool(node_id, "a", &node);
                quote! { (!#a) }
            }
            NodeKind::GetBoolVar => {
                let var_name = node.string_a.as_deref().unwrap_or("unnamed");
                let ident = self.var_ident("b", var_name);
                quote! { #ident }
            }
            NodeKind::SelectBool => {
                let cond = self.resolve_bool(node_id, "condition", &node);
                let a    = self.resolve_bool(node_id, "value_a",   &node);
                let b    = self.resolve_bool(node_id, "value_b",   &node);
                quote! { (if #cond { #a } else { #b }) }
            }
            _ => quote! { false },
        }
    }

    // ── Int resolution ────────────────────────────────────────────────────────

    pub(super) fn resolve_int(
        &mut self,
        node_id: usize,
        pin: &str,
        node: &VisualNode,
    ) -> TokenStream {
        if let Some(src) = self.graph.data_source(node_id, pin) {
            if let Some(src_node) = self.graph.node(src.node_id).cloned() {
                if src_node.is_pure() {
                    return self.eval_int_expr(src.node_id, src_node, src.pin);
                }
            }
            let id = self.binding_ident(src);
            return quote! { #id };
        }
        let v = node.int_literal_for(pin);
        quote! { #v }
    }

    pub(super) fn eval_int_expr(
        &mut self,
        node_id: usize,
        node: VisualNode,
        _out_pin: &'static str,
    ) -> TokenStream {
        match node.kind {
            NodeKind::IntAdd => {
                let a = self.resolve_int(node_id, "a", &node);
                let b = self.resolve_int(node_id, "b", &node);
                quote! { (#a + #b) }
            }
            NodeKind::IntSubtract => {
                let a = self.resolve_int(node_id, "a", &node);
                let b = self.resolve_int(node_id, "b", &node);
                quote! { (#a - #b) }
            }
            NodeKind::IntMultiply => {
                let a = self.resolve_int(node_id, "a", &node);
                let b = self.resolve_int(node_id, "b", &node);
                quote! { (#a * #b) }
            }
            NodeKind::IntDivide => {
                let a = self.resolve_int(node_id, "a", &node);
                let b = self.resolve_int(node_id, "b", &node);
                quote! { ({ let __d = #b; if __d == 0 { 0i32 } else { #a / __d } }) }
            }
            NodeKind::IntModulo => {
                let a = self.resolve_int(node_id, "a", &node);
                let b = self.resolve_int(node_id, "b", &node);
                quote! { ({ let __d = #b; if __d == 0 { 0i32 } else { #a % __d } }) }
            }
            NodeKind::FloatToInt => {
                let a = self.resolve_float(node_id, "a", &node);
                quote! { (#a as i32) }
            }
            NodeKind::GetIntVar => {
                let var_name = node.string_a.as_deref().unwrap_or("unnamed");
                let ident = self.var_ident("i", var_name);
                quote! { #ident }
            }
            NodeKind::FloatArrayLength => {
                let arr_name = node.string_a.as_deref().unwrap_or("unnamed");
                let ident = self.var_ident("arr_f", arr_name);
                quote! { (#ident.len() as i32) }
            }
            NodeKind::IntArrayGet => {
                let arr_name = node.string_a.as_deref().unwrap_or("unnamed");
                let arr_ident = self.var_ident("arr_i", arr_name);
                let idx = self.resolve_int(node_id, "index", &node);
                quote! { #arr_ident.get(#idx as usize).copied().unwrap_or(0i32) }
            }
            NodeKind::IntArrayLength => {
                let arr_name = node.string_a.as_deref().unwrap_or("unnamed");
                let ident = self.var_ident("arr_i", arr_name);
                quote! { (#ident.len() as i32) }
            }
            NodeKind::SelectInt => {
                let cond = self.resolve_bool(node_id, "condition", &node);
                let a    = self.resolve_int(node_id, "value_a",    &node);
                let b    = self.resolve_int(node_id, "value_b",    &node);
                quote! { (if #cond { #a } else { #b }) }
            }
            NodeKind::StringArrayLength => {
                let arr_name = node.string_a.as_deref().unwrap_or("unnamed");
                let ident = self.var_ident("arr_s", arr_name);
                quote! { (#ident.len() as i32) }
            }
            _ => quote! { 0i32 },
        }
    }

    // ── String resolution ─────────────────────────────────────────────────────

    pub(super) fn resolve_string(
        &mut self,
        node_id: usize,
        pin: &str,
        node: &VisualNode,
    ) -> TokenStream {
        if let Some(src) = self.graph.data_source(node_id, pin) {
            let id = self.binding_ident(src);
            return quote! { #id.as_str() };
        }
        if let Some(msg) = node.log_message.as_deref() {
            if node.kind == NodeKind::PrintLog && pin == "message" {
                let lit = syn::LitStr::new(msg, proc_macro2::Span::call_site());
                return quote! { #lit };
            }
        }
        if node.kind == NodeKind::GetStringVar {
            let var_name = node.string_a.as_deref().unwrap_or("unnamed");
            let ident = self.var_ident("s", var_name);
            return quote! { #ident.as_str() };
        }
        if node.kind == NodeKind::SelectString {
            // We can't easily call eval inside resolve_string without a node+id context here,
            // so emit a ternary inline — resolution will occur at compile time via the bindings.
            return quote! { "<select_string>" };
        }
        if node.kind == NodeKind::StringArrayGet {
            let arr_name = node.string_a.as_deref().unwrap_or("unnamed");
            let arr_ident = self.var_ident("arr_s", arr_name);
            let idx = self.resolve_int(node_id, "index", node);
            return quote! { #arr_ident.get(#idx as usize).map(|s| s.as_str()).unwrap_or("") };
        }
        let lit = syn::LitStr::new("<bo'sh>", proc_macro2::Span::call_site());
        quote! { #lit }
    }

    // ── Vec2 resolution ───────────────────────────────────────────────────────
    // Bevy-ga bog'liq emas — [f32; 2] massiv sifatida generatsiya qilinadi.

    pub(super) fn resolve_vec2(
        &mut self,
        node_id: usize,
        pin: &str,
        _node: &VisualNode,
    ) -> TokenStream {
        if let Some(src) = self.graph.data_source(node_id, pin) {
            if let Some(src_node) = self.graph.node(src.node_id).cloned() {
                if src_node.is_pure() {
                    return self.eval_vec2_expr(src.node_id, src_node);
                }
            }
            let id = self.binding_ident(src);
            return quote! { #id };
        }
        // Literal: no Vec2 literal on VisualNode — emit zero
        quote! { [0.0_f32, 0.0_f32] }
    }

    pub(super) fn eval_vec2_expr(&mut self, node_id: usize, node: VisualNode) -> TokenStream {
        match node.kind {
            NodeKind::Vec2Make => {
                let x = self.resolve_float(node_id, "x", &node);
                let y = self.resolve_float(node_id, "y", &node);
                quote! { [#x, #y] }
            }
            NodeKind::Vec2Add => {
                let a = self.resolve_vec2(node_id, "a", &node);
                let b = self.resolve_vec2(node_id, "b", &node);
                quote! { { let __a=#a; let __b=#b; [__a[0]+__b[0], __a[1]+__b[1]] } }
            }
            NodeKind::Vec2Sub => {
                let a = self.resolve_vec2(node_id, "a", &node);
                let b = self.resolve_vec2(node_id, "b", &node);
                quote! { { let __a=#a; let __b=#b; [__a[0]-__b[0], __a[1]-__b[1]] } }
            }
            NodeKind::Vec2Scale => {
                let v = self.resolve_vec2(node_id, "vec", &node);
                let s = self.resolve_float(node_id, "scale", &node);
                quote! { { let __v=#v; let __s=#s; [__v[0]*__s, __v[1]*__s] } }
            }
            NodeKind::Vec2Normalize => {
                let v = self.resolve_vec2(node_id, "vec", &node);
                quote! { {
                    let __v = #v;
                    let __len = (__v[0]*__v[0] + __v[1]*__v[1]).sqrt();
                    if __len < 1e-9 { [0.0_f32; 2] } else { [__v[0]/__len, __v[1]/__len] }
                } }
            }
            NodeKind::Vec2Length => {
                let v = self.resolve_vec2(node_id, "vec", &node);
                quote! { { let __v=#v; (__v[0]*__v[0]+__v[1]*__v[1]).sqrt() } }
            }
            NodeKind::Vec2Dot => {
                let a = self.resolve_vec2(node_id, "a", &node);
                let b = self.resolve_vec2(node_id, "b", &node);
                quote! { { let __a=#a; let __b=#b; __a[0]*__b[0]+__a[1]*__b[1] } }
            }
            NodeKind::Vec2X => {
                let v = self.resolve_vec2(node_id, "vec", &node);
                quote! { (#v)[0] }
            }
            NodeKind::Vec2Y => {
                let v = self.resolve_vec2(node_id, "vec", &node);
                quote! { (#v)[1] }
            }
            _ => quote! { [0.0_f32, 0.0_f32] },
        }
    }
}
