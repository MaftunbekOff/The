//! Exec grafi yurish — shoxlanish va data bog'lash (mantiqsiz codegen yo'q).

use std::collections::HashSet;

use crate::ast::VisualScriptGraph;
use crate::nodes::NodeKind;

/// Data chiqish manbasi.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DataPort {
    pub node_id: usize,
    pub pin: &'static str,
}

impl VisualScriptGraph {
    pub fn data_source(&self, to_node: usize, to_pin: &str) -> Option<DataPort> {
        self.data_links
            .iter()
            .find(|l| l.to_node_id == to_node && l.to_pin == to_pin)
            .map(|l| DataPort {
                node_id: l.from_node_id,
                pin: l.from_pin,
            })
    }

    pub fn validate_branch_nodes(&self) -> Result<(), String> {
        for (id, node) in &self.nodes {
            if node.kind != NodeKind::Branch {
                continue;
            }
            if self.data_source(*id, "condition").is_none()
                && node.condition_value.is_none()
            {
                return Err(format!(
                    "Branch #{id}: `condition` (bool) simi yoki checkbox qiymati kerak"
                ));
            }
            let has_true = !self.exec_successors(*id, "true").is_empty();
            let has_false = !self.exec_successors(*id, "false").is_empty();
            if !has_true && !has_false {
                return Err(format!("Branch #{id}: `true` yoki `false` shoxi kerak"));
            }
        }
        Ok(())
    }
}

/// Exec DAG bo'yicha DFS (sikllarni ushlash).
/// Cycle detection uchun `HashSet` — O(1) lookup.
pub fn validate_exec_dag(graph: &VisualScriptGraph, entry: usize) -> Result<(), String> {
    fn visit(
        graph: &VisualScriptGraph,
        id: usize,
        stack: &mut Vec<usize>,
        in_stack: &mut HashSet<usize>,
        visited: &mut HashSet<usize>,
    ) -> Result<(), String> {
        if in_stack.contains(&id) {
            return Err(format!(
                "exec sikli: tugun #{id} — faqat komponent orqali bir tomonlama oqim"
            ));
        }
        if visited.contains(&id) {
            return Ok(());
        }
        visited.insert(id);
        stack.push(id);
        in_stack.insert(id);

        let node = graph
            .node(id)
            .ok_or_else(|| format!("tugun #{id} topilmadi"))?;

        for out in node.exec_outputs() {
            for child in graph.exec_successors(id, out.name) {
                visit(graph, child, stack, in_stack, visited)?;
            }
        }

        stack.pop();
        in_stack.remove(&id);
        Ok(())
    }

    let mut stack = Vec::new();
    let mut in_stack = HashSet::new();
    let mut visited = HashSet::new();
    visit(graph, entry, &mut stack, &mut in_stack, &mut visited)
}

#[cfg(test)]
mod tests {
    use crate::interpreter::demo::demo_rich_branch_graph;

    #[test]
    fn exec_successors_branch() {
        let g = demo_rich_branch_graph();
        assert_eq!(g.exec_successors(2, "true"), vec![3]);
        assert_eq!(g.exec_successors(2, "false"), vec![4]);
    }
}
