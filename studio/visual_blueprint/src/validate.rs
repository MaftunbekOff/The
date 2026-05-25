//! Graf qoidalari: qat’iy tiplar, exec DAG, branch shartlari.

use crate::ast::VisualScriptGraph;
use crate::interpreter::flow::validate_exec_dag;
use crate::pins::{DataLink, PinType};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationError {
    pub message: String,
}

impl ValidationError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

pub type ValidationResult<T> = Result<T, ValidationError>;

pub fn validate_data_link(
    from_ty: PinType,
    to_ty: PinType,
    context: &str,
) -> ValidationResult<()> {
    if from_ty == PinType::Exec || to_ty == PinType::Exec {
        return Err(ValidationError::new(format!(
            "{context}: Exec porti faqat `exec_links` orqali ulanadi"
        )));
    }
    if from_ty != to_ty {
        return Err(ValidationError::new(format!(
            "{context}: tip mos emas — chiqish `{}`, kirish `{}` kutilgan",
            from_ty.name(),
            to_ty.name()
        )));
    }
    Ok(())
}

pub fn validate_graph(graph: &VisualScriptGraph) -> ValidationResult<()> {
    for link in &graph.data_links {
        validate_data_link_entry(graph, link)?;
    }
    graph
        .validate_branch_nodes()
        .map_err(ValidationError::new)?;

    if let Ok(entry) = graph.begin_play_entry() {
        validate_exec_dag(graph, entry).map_err(ValidationError::new)?;
    }
    if graph.tick_entry().is_ok() {
        let entry = graph.tick_entry().map_err(ValidationError::new)?;
        validate_exec_dag(graph, entry).map_err(ValidationError::new)?;
    }
    Ok(())
}

fn validate_data_link_entry(graph: &VisualScriptGraph, link: &DataLink) -> ValidationResult<()> {
    let from_node = graph
        .node(link.from_node_id)
        .ok_or_else(|| ValidationError::new(format!("tugun #{} topilmadi", link.from_node_id)))?;
    let to_node = graph
        .node(link.to_node_id)
        .ok_or_else(|| ValidationError::new(format!("tugun #{} topilmadi", link.to_node_id)))?;

    let from_ty = from_node
        .data_output_type(link.from_pin)
        .ok_or_else(|| {
            ValidationError::new(format!(
                "tugun #{} da `{}` data chiqishi yo‘q",
                link.from_node_id, link.from_pin
            ))
        })?;
    let to_ty = to_node
        .data_input_type(link.to_pin)
        .ok_or_else(|| {
            ValidationError::new(format!(
                "tugun #{} da `{}` data kirishi yo‘q",
                link.to_node_id, link.to_pin
            ))
        })?;

    validate_data_link(
        from_ty,
        to_ty,
        &format!(
            "data #{}:{} → #{}:{}",
            link.from_node_id, link.from_pin, link.to_node_id, link.to_pin
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ExecLink, VisualScriptGraph};
    use crate::interpreter::demo::demo_rich_branch_graph;
    use crate::nodes::VisualNode;
    use crate::pins::DataLink;

    #[test]
    fn rejects_string_to_entity() {
        let err = validate_data_link(PinType::String, PinType::Entity, "test").unwrap_err();
        assert!(err.message.contains("tip mos emas"));
    }

    #[test]
    fn rejects_bool_to_string_on_data_link() {
        let err = validate_data_link(PinType::Bool, PinType::String, "test").unwrap_err();
        assert!(err.message.contains("tip mos emas"));
    }

    #[test]
    fn demo_treasury_graph_is_valid() {
        validate_graph(&crate::demo_treasury_graph()).expect("treasury");
    }

    #[test]
    fn rich_branch_graph_is_valid() {
        validate_graph(&demo_rich_branch_graph()).expect("branch");
    }

    #[test]
    fn rejects_bool_to_print_log_message() {
        let mut g = demo_rich_branch_graph();
        g.data_links.push(DataLink {
            from_node_id: 1,
            from_pin: "result",
            to_node_id: 3,
            to_pin: "message",
        });
        assert!(validate_graph(&g).is_err());
    }

    #[test]
    fn rejects_exec_cycle() {
        let graph = VisualScriptGraph::new(
            "cycle",
            vec![
                (0, VisualNode::event_begin_play()),
                (1, VisualNode::print_log("a")),
                (2, VisualNode::print_log("b")),
            ],
            vec![
                ExecLink::exec_out(0, 1),
                ExecLink::exec_out(1, 2),
                ExecLink::exec_out(2, 1),
            ],
            vec![],
        );
        assert!(validate_graph(&graph).is_err());
    }
}
