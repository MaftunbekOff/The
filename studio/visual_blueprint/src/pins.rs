//! Qat’iy port tiplari — `Variant` / `Dynamic` yo‘q.

/// Bevy `Entity` o‘rniga yengil ID (blueprint crate Bevy-siz kompilyatsiya qilinadi).
pub type VisualEntityId = u64;

/// Portning Rust-darajasidagi tipi (har bir pin aynan bittasi).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PinType {
    /// Bajarish oqimi (faqat `exec_links`).
    Exec,
    Bool,
    Float,
    String,
    /// 3 o'lchovli vektor — `[f32; 3]` (x, y, z).
    Vec3,
    Entity,
    /// 32-bit butun son.
    Int,
    /// 2 o'lchovli vektor — `[f32; 2]` (x, y).
    Vec2,
}

impl PinType {
    pub fn name(self) -> &'static str {
        match self {
            PinType::Exec   => "Exec",
            PinType::Bool   => "bool",
            PinType::Float  => "f32",
            PinType::String => "String",
            PinType::Vec3   => "Vec3",
            PinType::Entity => "Entity",
            PinType::Int    => "i32",
            PinType::Vec2   => "Vec2",
        }
    }
}

/// Ma’lumot pinidagi compile-time konstanta.
#[derive(Clone, Debug, PartialEq)]
pub enum TypedLiteral {
    Bool(bool),
    Float(f32),
    String(String),
    Entity(VisualEntityId),
}

impl TypedLiteral {
    pub fn pin_type(&self) -> PinType {
        match self {
            TypedLiteral::Bool(_) => PinType::Bool,
            TypedLiteral::Float(_) => PinType::Float,
            TypedLiteral::String(_) => PinType::String,
            TypedLiteral::Entity(_) => PinType::Entity,
        }
    }
}

/// Ma’lumot simi: chiqish → kirish (tiplar mos).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataLink {
    pub from_node_id: usize,
    pub from_pin: &'static str,
    pub to_node_id: usize,
    pub to_pin: &'static str,
}

/// Port tavsifi.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PinSpec {
    pub name: &'static str,
    pub ty: PinType,
}
