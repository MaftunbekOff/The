//! Vec2 math nodes — 2D vektor operatsiyalari (faqat sof hisob).

use twelfth_visual_blueprint::nodes::{NodeKind as K, VisualNode};

use crate::nodes::descriptor::{NodeDescriptor, NodeRegistry};

pub(crate) fn register(r: &mut NodeRegistry) {
    r.register(NodeDescriptor {
        kind: K::Vec2Make,
        label: "Vec2 Make",
        description: "X, Y komponentlaridan Vec2 yasaydi",
        category: "Math",
        width: 180.0, height: 72.0,
        default_node: || VisualNode::vec2_make(0.0, 0.0),
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::Vec2Add,
        label: "Vec2 Add",
        description: "Ikkita Vec2 qo'shadi",
        category: "Math",
        width: 180.0, height: 64.0,
        default_node: VisualNode::vec2_add,
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::Vec2Sub,
        label: "Vec2 Subtract",
        description: "Vec2 ayiradi",
        category: "Math",
        width: 180.0, height: 64.0,
        default_node: VisualNode::vec2_sub,
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::Vec2Scale,
        label: "Vec2 Scale",
        description: "Vec2 ni skalar bilan ko'paytiradi",
        category: "Math",
        width: 180.0, height: 64.0,
        default_node: VisualNode::vec2_scale,
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::Vec2Length,
        label: "Vec2 Length",
        description: "Vec2 uzunligini float sifatida qaytaradi",
        category: "Math",
        width: 180.0, height: 56.0,
        default_node: VisualNode::vec2_length,
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::Vec2Normalize,
        label: "Vec2 Normalize",
        description: "Vec2 ni birlik vektorga keltiradi",
        category: "Math",
        width: 190.0, height: 56.0,
        default_node: VisualNode::vec2_normalize,
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::Vec2Dot,
        label: "Vec2 Dot",
        description: "Ikkita Vec2 skalyar ko'paytmasi",
        category: "Math",
        width: 180.0, height: 64.0,
        default_node: VisualNode::vec2_dot,
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::Vec2X,
        label: "Vec2 X",
        description: "Vec2 ning X komponentini chiqaradi",
        category: "Math",
        width: 160.0, height: 56.0,
        default_node: VisualNode::vec2_x,
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::Vec2Y,
        label: "Vec2 Y",
        description: "Vec2 ning Y komponentini chiqaradi",
        category: "Math",
        width: 160.0, height: 56.0,
        default_node: VisualNode::vec2_y,
        exec: None,
    });
}
