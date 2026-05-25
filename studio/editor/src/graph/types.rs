//! Graph node/port/wire types.

use bevy::prelude::*;
use twelfth_visual_blueprint::nodes::VisualNode;

use crate::registry::{NodeKind, WireCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct NodeId(pub u32);

#[derive(Component, Debug, Clone, Copy)]
pub struct VmNode {
    pub id: NodeId,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct VmNodeTitle;

#[derive(Component, Debug, Clone, Copy)]
pub struct GraphPort {
    pub node: NodeId,
    pub pin: &'static str,
    pub category: WireCategory,
}

#[derive(Debug, Clone)]
pub struct GraphNodeData {
    pub id: NodeId,
    pub kind: NodeKind,
    pub position: Vec2,
    pub params: VisualNode,
}

impl GraphNodeData {
    pub fn title(&self) -> &'static str {
        self.kind.label()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GraphWire {
    pub from_node: NodeId,
    pub from_pin: &'static str,
    pub to_node: NodeId,
    pub to_pin: &'static str,
    pub category: WireCategory,
}
