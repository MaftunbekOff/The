//! Exec node dispatch during PIE.
//!
//! Bu fayl faqat markaziy dispatcher — hech qanday node mantig'i bu yerda emas.
//! Har bir node o'z `ExecBehavior` implementatsiyasida `editor/src/nodes/` da.

use crate::interpreter::context::PieExecContext;
use crate::interpreter::error::{PieError, PieErrorKind};
use crate::nodes::descriptor::{ExecFlow, NodeRegistry};

/// Maksimal exec stack chuqurligi — bu chegaradan oshsa xato qaytariladi.
const MAX_EXEC_DEPTH: usize = 512;

pub(crate) fn execute_exec_node(
    ctx: &mut PieExecContext<'_>,
    node_id: usize,
) -> Result<(), String> {
    if ctx.depth >= MAX_EXEC_DEPTH {
        return Err(PieError::stack_overflow(node_id, ctx.depth).into());
    }
    ctx.depth += 1;
    let result = dispatch(ctx, node_id);
    ctx.depth -= 1;
    result
}

fn dispatch(ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<(), String> {
    let node = ctx
        .graph
        .node(node_id)
        .ok_or_else(|| String::from(PieError::node_not_found(node_id)))?
        .clone();

    let registry = NodeRegistry::global();
    let desc = registry
        .get(node.kind)
        .ok_or_else(|| String::from(PieError::unregistered(node_id, &format!("{:?}", node.kind))))?;

    let exec = desc.exec.ok_or_else(|| {
        String::from(PieError::pure_in_exec(node_id, &format!("{:?}", node.kind)))
    })?;

    let flow = exec.run(ctx, node_id)?;

    match flow {
        ExecFlow::ExecOut => {
            for child in ctx.graph.exec_successors(node_id, "exec_out") {
                execute_exec_node(ctx, child)?;
            }
        }
        ExecFlow::Pin(pin) => {
            for child in ctx.graph.exec_successors(node_id, pin) {
                execute_exec_node(ctx, child)?;
            }
        }
        ExecFlow::ThenExecOut(targets) => {
            for target in targets {
                execute_exec_node(ctx, target)?;
            }
            for child in ctx.graph.exec_successors(node_id, "exec_out") {
                execute_exec_node(ctx, child)?;
            }
        }
        ExecFlow::Deferred => {
            // Delay ro'yxatga qo'shildi (exec_fn ichida), zanjir to'xtaydi.
        }
        ExecFlow::Break => {
            // Break signali — caller (ForEachFloat/WhileLoop) uni tutib oladi.
            return Err("__break__".into());
        }
        ExecFlow::Continue => {
            // Continue signali — caller loop uni tutib oladi.
            return Err("__continue__".into());
        }
        ExecFlow::ResetDone => {
            for child in ctx.graph.exec_successors(node_id, "exec_out") {
                execute_exec_node(ctx, child)?;
            }
        }
    }

    Ok(())
}

/// Xatoni `PieError` ga aylantiradi (log uchun).
/// `__break__` kabi internal signallar saqlanadi.
#[allow(dead_code)]
pub(crate) fn to_pie_error(raw: String) -> PieError {
    if raw == "__break__" {
        return PieError { kind: PieErrorKind::General, node_id: None, message: raw };
    }
    PieError { kind: PieErrorKind::General, node_id: None, message: raw }
}
