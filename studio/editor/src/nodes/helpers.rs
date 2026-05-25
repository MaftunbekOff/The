//! Helper nodes — RandomFloat, GetGameTime, IsValidEntity, Select*.

use twelfth_visual_blueprint::nodes::{NodeKind as K, VisualNode};

use crate::nodes::descriptor::{NodeDescriptor, NodeRegistry};

pub(crate) fn register(r: &mut NodeRegistry) {
    r.register(NodeDescriptor {
        kind: K::RandomFloat,
        label: "Random Float",
        description: "Min-max oralig'ida tasodifiy float (sof hisob)",
        category: "Math",
        width: 200.0, height: 72.0,
        default_node: || VisualNode::random_float(0.0, 1.0),
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::GetGameTime,
        label: "Get Game Time",
        description: "O'yin boshlanganidan beri o'tgan vaqt (soniya)",
        category: "Math",
        width: 190.0, height: 56.0,
        default_node: VisualNode::get_game_time,
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::IsValidEntity,
        label: "Is Valid Entity",
        description: "Entity mavjud yoki yo'qligini tekshiradi (sof hisob)",
        category: "Entity",
        width: 200.0, height: 64.0,
        default_node: VisualNode::is_valid_entity,
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::SelectFloat,
        label: "Select Float",
        description: "Shart asosida ikkita float ichidan birini tanlaydi",
        category: "Math",
        width: 200.0, height: 80.0,
        default_node: VisualNode::select_float,
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::SelectBool,
        label: "Select Bool",
        description: "Shart asosida ikkita bool ichidan birini tanlaydi",
        category: "Logic",
        width: 200.0, height: 80.0,
        default_node: VisualNode::select_bool,
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::SelectInt,
        label: "Select Int",
        description: "Shart asosida ikkita int ichidan birini tanlaydi",
        category: "Math",
        width: 200.0, height: 80.0,
        default_node: VisualNode::select_int,
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::SelectString,
        label: "Select String",
        description: "Shart asosida ikkita string ichidan birini tanlaydi",
        category: "String",
        width: 210.0, height: 80.0,
        default_node: VisualNode::select_string,
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::Comment,
        label: "Comment",
        description: "Graf uchun izoh qutisi — exec/data pinlari yo'q",
        category: "Utility",
        width: 240.0, height: 64.0,
        default_node: || VisualNode::comment("// Izoh..."),
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::GetSelfEntity,
        label: "Get Self Entity",
        description: "Joriy ScriptActor entity'sini qaytaradi (per-entity scripting)",
        category: "Entity",
        width: 190.0, height: 56.0,
        default_node: VisualNode::get_self_entity,
        exec: None,
    });
}
