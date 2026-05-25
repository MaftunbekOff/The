//! Graf RON saqlash / yuklash qatlami.
//!
//! Nega RON:
//!  - Bevy o'zi RON ishlatadi (assets/, scenes)
//!  - Rust enum nomlari to'g'ridan-to'g'ri: `Vec3Make` vs `{"type":"Vec3Make"}`
//!  - Kommentlar yozish mumkin (`//`)
//!  - Nol qo'shimcha konversiya serde orqali

use std::collections::HashSet;
use std::path::Path;
use std::sync::{Mutex, OnceLock};

use bevy::prelude::Vec2;
use serde::{Deserialize, Serialize};
use twelfth_visual_blueprint::nodes::{NodeKind as BpNodeKind, VisualNode};

use crate::graph::{GraphNodeData, GraphResource};
use crate::registry::{NodeKind, WireCategory};

/// Joriy saqlash format versiyasi.
const CURRENT_VERSION: u32 = 2;

// ── Save-format structs ───────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphSaveData {
    /// Format versiyasi — eski fayllar uchun serde(default) = 0.
    #[serde(default)]
    pub version: u32,
    pub next_id: u32,
    pub nodes: Vec<SaveNode>,
    pub exec_wires: Vec<SaveWire>,
    pub data_wires: Vec<SaveWire>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveNode {
    pub id: u32,
    pub kind: BpNodeKind,
    pub x: f32,
    pub y: f32,
    pub params: VisualNode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveWire {
    pub from: u32,
    pub from_pin: String,
    pub to: u32,
    pub to_pin: String,
}

// ── Save ──────────────────────────────────────────────────────────────────────

pub fn save_graph_ron(graph: &GraphResource) -> Result<String, String> {
    let data = GraphSaveData {
        version: CURRENT_VERSION,
        next_id: graph.next_id,
        nodes: graph
            .nodes
            .iter()
            .map(|n| SaveNode {
                id: n.id.0,
                kind: n.kind.0,
                x: n.position.x,
                y: n.position.y,
                params: n.params.clone(),
            })
            .collect(),
        exec_wires: graph
            .exec_wires
            .iter()
            .map(|w| SaveWire {
                from: w.from_node.0,
                from_pin: w.from_pin.to_string(),
                to: w.to_node.0,
                to_pin: w.to_pin.to_string(),
            })
            .collect(),
        data_wires: graph
            .data_wires
            .iter()
            .map(|w| SaveWire {
                from: w.from_node.0,
                from_pin: w.from_pin.to_string(),
                to: w.to_node.0,
                to_pin: w.to_pin.to_string(),
            })
            .collect(),
    };

    ron::ser::to_string_pretty(&data, ron::ser::PrettyConfig::default())
        .map_err(|e| format!("RON serialize: {e}"))
}

pub fn write_graph_ron(graph: &GraphResource, path: &Path) -> Result<(), String> {
    let text = save_graph_ron(graph)?;
    std::fs::write(path, text).map_err(|e| format!("Fayl yozish: {e}"))
}

// ── Load ──────────────────────────────────────────────────────────────────────

pub fn read_graph_ron(path: &Path) -> Result<GraphResource, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("Fayl o'qish: {e}"))?;
    load_graph_ron(&text)
}

pub fn load_graph_ron(ron_str: &str) -> Result<GraphResource, String> {
    let data: GraphSaveData =
        ron::de::from_str(ron_str).map_err(|e| format!("RON parse: {e}"))?;
    graph_from_save(data)
}

fn graph_from_save(data: GraphSaveData) -> Result<GraphResource, String> {
    use crate::graph::GraphWire;
    use crate::graph::NodeId;

    let nodes = data
        .nodes
        .into_iter()
        .map(|n| GraphNodeData {
            id: NodeId(n.id),
            kind: NodeKind(n.kind),
            position: Vec2::new(n.x, n.y),
            params: n.params,
        })
        .collect();

    let exec_wires = data
        .exec_wires
        .into_iter()
        .map(|w| -> Result<GraphWire, String> {
            Ok(GraphWire {
                from_node: NodeId(w.from),
                from_pin: intern_pin(&w.from_pin),
                to_node: NodeId(w.to),
                to_pin: intern_pin(&w.to_pin),
                category: WireCategory::Exec,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let data_wires = data
        .data_wires
        .into_iter()
        .map(|w| -> Result<GraphWire, String> {
            Ok(GraphWire {
                from_node: NodeId(w.from),
                from_pin: intern_pin(&w.from_pin),
                to_node: NodeId(w.to),
                to_pin: intern_pin(&w.to_pin),
                category: WireCategory::Data,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut graph = GraphResource {
        nodes,
        exec_wires,
        data_wires,
        next_id: data.next_id,
        by_id: std::collections::HashMap::new(),
    };
    graph.rebuild_index();
    Ok(graph)
}

// ── Pin name interning ────────────────────────────────────────────────────────

/// Dinamik intern pool — noma'lum pin nomlarini `&'static str` ga aylantiradi.
/// Har bir noyob nom faqat bir marta leak qilinadi (bounded leak).
static DYNAMIC_PINS: OnceLock<Mutex<HashSet<&'static str>>> = OnceLock::new();

/// String pin nomini `&'static str` ga aylantiradi.
///
/// Taniqli nomlar fast-path orqali qaytariladi.
/// Noma'lum nomlar global intern poolga qo'shiladi (leak, lekin bounded).
pub fn intern_pin(s: &str) -> &'static str {
    // ── Fast path: barcha taniqli pin nomlari ──────────────────────────────────
    match s {
        // Exec
        "exec_in"    => return "exec_in",
        "exec_out"   => return "exec_out",
        "true"       => return "true",
        "false"      => return "false",
        // Loop pins
        "loop_body"  => return "loop_body",
        "completed"  => return "completed",
        "then_0"     => return "then_0",
        "then_1"     => return "then_1",
        // Umumiy data
        "a"          => return "a",
        "b"          => return "b",
        "t"          => return "t",
        "x"          => return "x",
        "y"          => return "y",
        "z"          => return "z",
        "result"     => return "result",
        // Float
        "value"      => return "value",
        "min"        => return "min",
        "max"        => return "max",
        "base"       => return "base",
        "exp"        => return "exp",
        "eps"        => return "eps",
        "scale"      => return "scale",
        "vec"        => return "vec",
        // Exec params
        "message"    => return "message",
        "duration"   => return "duration",
        "amount"     => return "amount",
        "condition"  => return "condition",
        "threshold"  => return "threshold",
        "delta_time" => return "delta_time",
        // Input
        "pressed"    => return "pressed",
        // String nodes
        "string_a"   => return "string_a",
        "string_b"   => return "string_b",
        "text"       => return "text",
        // Int nodes
        "int_a"      => return "int_a",
        "int_b"      => return "int_b",
        "int_result" => return "int_result",
        "float_val"  => return "float_val",
        "int_val"    => return "int_val",
        // Entity / Transform pins
        "entity"     => return "entity",
        "position"   => return "position",
        "delta"      => return "delta",
        "rotation"   => return "rotation",
        // Loop data
        "item"       => return "item",
        "index"      => return "index",
        _ => {}
    }

    // ── Slow path: dinamik interning ───────────────────────────────────────────
    let pool = DYNAMIC_PINS.get_or_init(|| Mutex::new(HashSet::new()));
    let mut lock = pool.lock().unwrap();
    if let Some(&existing) = lock.get(s) {
        return existing;
    }
    let leaked: &'static str = Box::leak(s.to_string().into_boxed_str());
    lock.insert(leaked);
    leaked
}
