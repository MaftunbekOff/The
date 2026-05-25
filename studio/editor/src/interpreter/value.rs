//! Runtime data values during PIE execution.

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub(crate) enum PieValue {
    Bool(bool),
    Float(f32),
    Vec3([f32; 3]),
    Str(String),
    Int(i32),
    /// Entity slot ID — `PieEntityTable` da entity ni topish uchun.
    Entity(u32),
    /// Float massivi — FloatArray tugunlari uchun.
    FloatArray(Vec<f32>),
    /// Int massivi — IntArray tugunlari uchun.
    IntArray(Vec<i32>),
    /// String massivi — StringArray tugunlari uchun.
    StringArray(Vec<String>),
    /// 2D vektor — Vec2 tugunlari uchun.
    Vec2([f32; 2]),
}
