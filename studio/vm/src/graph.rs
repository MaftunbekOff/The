//! Editor-agnostic graph snapshot (faqat `out → in` simlar).

/// Tugun identifikatori.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u32);

/// Tugun turlari (editor `NodeKind` bilan mos).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeKind {
    Start,
    Log,
    Wait,
    End,
}

impl NodeKind {
    pub fn label(self) -> &'static str {
        match self {
            NodeKind::Start => "Start",
            NodeKind::Log => "Log",
            NodeKind::Wait => "Wait",
            NodeKind::End => "End",
        }
    }
}

/// Kanvasdan nusxa — VM faqat shu bilan ishlaydi.
#[derive(Debug, Clone, Default)]
pub struct GraphSnapshot {
    pub nodes: Vec<(NodeId, NodeKind)>,
    /// `(chiqish tuguni, kirish tuguni)`
    pub edges: Vec<(NodeId, NodeId)>,
}

impl GraphSnapshot {
    pub fn kind(&self, id: NodeId) -> Option<NodeKind> {
        self.nodes
            .iter()
            .find(|(nid, _)| *nid == id)
            .map(|(_, k)| *k)
    }

    pub fn entry(&self) -> Option<NodeId> {
        self.nodes
            .iter()
            .find(|(_, k)| *k == NodeKind::Start)
            .map(|(id, _)| *id)
    }

    pub fn next_after(&self, from: NodeId) -> Option<NodeId> {
        self.edges
            .iter()
            .find(|(output, _)| *output == from)
            .map(|(_, input)| *input)
    }

    pub fn wired_chain_from(&self, start: NodeId) -> Vec<NodeId> {
        let mut chain = vec![start];
        let mut current = start;
        while let Some(next) = self.next_after(current) {
            if chain.contains(&next) {
                break;
            }
            chain.push(next);
            current = next;
        }
        chain
    }
}
