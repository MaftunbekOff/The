//! Tick subgraph extraction from full blueprint graph.

use twelfth_visual_blueprint::ast::VisualScriptGraph;

pub(crate) fn tick_subgraph(full: &VisualScriptGraph) -> Option<VisualScriptGraph> {
    let entry = full.tick_entry().ok()?;
    let mut nodes = vec![(entry, full.node(entry)?.clone())];
    let mut exec_links = Vec::new();
    let mut data_links = Vec::new();
    let mut stack = vec![entry];
    let mut seen = vec![entry];

    while let Some(id) = stack.pop() {
        let node = full.node(id)?;
        for out in node.exec_outputs() {
            for child in full.exec_successors(id, out.name) {
                if !seen.contains(&child) {
                    seen.push(child);
                    nodes.push((child, full.node(child)?.clone()));
                    stack.push(child);
                }
                exec_links.push(twelfth_visual_blueprint::ExecLink {
                    from_node_id: id,
                    from_pin: out.name,
                    to_node_id: child,
                    to_pin: "exec_in",
                });
            }
        }
    }

    for link in &full.data_links {
        if seen.contains(&link.from_node_id) && seen.contains(&link.to_node_id) {
            data_links.push(link.clone());
        }
    }

    Some(VisualScriptGraph::new("PIE_Tick", nodes, exec_links, data_links))
}
