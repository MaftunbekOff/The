//! String kategoriyasi: StringConcat, FloatToString, BoolToString, IntToString.
//! Barcha sof-hisob tugunlar — exec = None.

use twelfth_visual_blueprint::nodes::{NodeKind as K, VisualNode};

use crate::nodes::descriptor::{NodeDescriptor, NodeRegistry};

pub(crate) fn register(r: &mut NodeRegistry) {
    r.register(NodeDescriptor {
        kind: K::StringConcat,
        label: "String Concat",
        description: "a + b → string",
        category: "String",
        width: 192.0, height: 104.0,
        default_node: || VisualNode::string_concat("", ""),
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::FloatToString,
        label: "Float→String",
        description: "f32 → string",
        category: "String",
        width: 160.0, height: 88.0,
        default_node: VisualNode::float_to_string,
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::BoolToString,
        label: "Bool→String",
        description: "bool → string",
        category: "String",
        width: 160.0, height: 88.0,
        default_node: VisualNode::bool_to_string,
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::IntToString,
        label: "Int→String",
        description: "i32 → string",
        category: "String",
        width: 160.0, height: 88.0,
        default_node: VisualNode::int_to_string,
        exec: None,
    });
}
