//! AST conversion and starter-graph construction.

use bevy::prelude::Vec2;
use twelfth_visual_blueprint::{
    ast::{ExecLink, VisualScriptGraph},
    nodes::VisualNode,
    pins::DataLink,
    validate::validate_graph,
};

use crate::registry::NodeKind;

use super::resource::GraphResource;

impl GraphResource {
    pub fn with_starter_graph() -> Self {
        let mut g = Self {
            nodes: Vec::new(),
            exec_wires: Vec::new(),
            data_wires: Vec::new(),
            next_id: 0,
            by_id: std::collections::HashMap::new(),
        };
        use twelfth_visual_blueprint::nodes::NodeKind as Bk;
        const START_X: f32 = 40.0;
        const ROW_Y:   f32 = 120.0;
        const GAP:     f32 = 90.0;

        let k0 = NodeKind(Bk::EventBeginPlay);
        let k1 = NodeKind(Bk::CheckGoldAmount);
        let k2 = NodeKind(Bk::Branch);
        let k3 = NodeKind(Bk::PrintLog);

        let x0 = START_X;
        let x1 = x0 + k0.visual_width() + GAP;
        let x2 = x1 + k1.visual_width() + GAP;
        let x3 = x2 + k2.visual_width() + GAP;

        let branch_h = k2.visual_height();
        let log_h    = k3.visual_height();
        let y3       = ROW_Y - log_h * 0.5;
        let y4       = ROW_Y + branch_h - log_h * 0.5;

        let e0 = g.push_node(k0, Vec2::new(x0, ROW_Y));
        let e1 = g.push_node(k1, Vec2::new(x1, ROW_Y));
        let e2 = g.push_node(k2, Vec2::new(x2, ROW_Y));
        let e3 = g.push_node(k3, Vec2::new(x3, y3));
        let e4 = g.push_node(k3, Vec2::new(x3, y4));
        if let Some(n) = g.node_mut(e3) {
            n.params = VisualNode::print_log("Rich!");
        }
        if let Some(n) = g.node_mut(e4) {
            n.params = VisualNode::print_log("Need more gold!");
        }
        let _ = g.connect_exec(e0, "exec_out", e1, "exec_in");
        let _ = g.connect_exec(e1, "exec_out", e2, "exec_in");
        let _ = g.connect_exec(e2, "true",     e3, "exec_in");
        let _ = g.connect_exec(e2, "false",    e4, "exec_in");
        let _ = g.connect_data(e1, "result",   e2, "condition");
        g
    }

    /// Build `VisualScriptGraph` AST for the interpreter / codegen pipeline.
    pub fn to_blueprint_graph(&self) -> Result<VisualScriptGraph, String> {
        let nodes = self
            .nodes
            .iter()
            .map(|n| (n.id.0 as usize, n.params.clone()))
            .collect();
        let exec_links = self
            .exec_wires
            .iter()
            .map(|w| ExecLink {
                from_node_id: w.from_node.0 as usize,
                from_pin:     w.from_pin,
                to_node_id:   w.to_node.0 as usize,
                to_pin:       w.to_pin,
            })
            .collect();
        let data_links = self
            .data_wires
            .iter()
            .map(|w| DataLink {
                from_node_id: w.from_node.0 as usize,
                from_pin:     w.from_pin,
                to_node_id:   w.to_node.0 as usize,
                to_pin:       w.to_pin,
            })
            .collect();
        let mut graph = VisualScriptGraph::new("EditorCanvas", nodes, exec_links, data_links);
        validate_graph(&graph).map_err(|e| e.message)?;
        graph.build_indices();
        Ok(graph)
    }
}
