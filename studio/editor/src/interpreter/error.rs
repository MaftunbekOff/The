//! Structured PIE runtime errors — `String` o'rniga aniq kategoriyalar.
//!
//! `PieError` type tizimga quyidagi ustunliklarni beradi:
//!
//! * **User-facing diagnostics**: qaysi node, qaysi kategoria, qanday xato.
//! * **Machine-readable**: editor kelajakda xato badge yoki highlight ko'rsatishi mumkin.
//! * **Backward compatible**: `Display` impl `String` error-ga mos keladi.

use std::fmt;

// ── PieErrorKind ──────────────────────────────────────────────────────────────

/// PIE runtime xatosining kategoriyasi.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PieErrorKind {
    /// Exec stack chegarasidan oshdi (cheksiz rekursiya).
    StackOverflow,
    /// Noma'lum node ID — graph ichida topilmadi.
    NodeNotFound,
    /// Sof-hisob (pure) node exec zanjirida ishlatildi.
    PureNodeInExec,
    /// Ro'yxatga olinmagan NodeKind — registry da yo'q.
    UnregisteredNode,
    /// Entity operatsiya xatosi (slot to'liq, entity topilmadi va h.k.).
    EntityError,
    /// Umumiy xato — boshqa kategorialarga tushmagan.
    General,
}

impl fmt::Display for PieErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PieErrorKind::StackOverflow     => write!(f, "StackOverflow"),
            PieErrorKind::NodeNotFound      => write!(f, "NodeNotFound"),
            PieErrorKind::PureNodeInExec    => write!(f, "PureNodeInExec"),
            PieErrorKind::UnregisteredNode  => write!(f, "UnregisteredNode"),
            PieErrorKind::EntityError       => write!(f, "EntityError"),
            PieErrorKind::General           => write!(f, "General"),
        }
    }
}

// ── PieError ──────────────────────────────────────────────────────────────────

/// PIE execution paytida yuzaga kelgan xato.
#[derive(Debug, Clone)]
pub struct PieError {
    /// Xato kategoriyasi.
    pub kind: PieErrorKind,
    /// Xato yuzaga kelgan node ID (agar ma'lum bo'lsa).
    pub node_id: Option<usize>,
    /// Inson o'qiy oladigan xabar.
    pub message: String,
}

impl PieError {
    pub fn new(kind: PieErrorKind, message: impl Into<String>) -> Self {
        Self { kind, node_id: None, message: message.into() }
    }

    pub fn at(mut self, node_id: usize) -> Self {
        self.node_id = Some(node_id);
        self
    }

    pub fn stack_overflow(node_id: usize, depth: usize) -> Self {
        Self {
            kind: PieErrorKind::StackOverflow,
            node_id: Some(node_id),
            message: format!(
                "PIE: exec stack overflow (chuqurlik={depth}, node #{node_id}) — \
                 cheksiz rekursiya yoki juda uzun zanjir?"
            ),
        }
    }

    pub fn node_not_found(node_id: usize) -> Self {
        Self {
            kind: PieErrorKind::NodeNotFound,
            node_id: Some(node_id),
            message: format!("PIE: tugun #{node_id} topilmadi"),
        }
    }

    pub fn pure_in_exec(node_id: usize, kind_name: &str) -> Self {
        Self {
            kind: PieErrorKind::PureNodeInExec,
            node_id: Some(node_id),
            message: format!(
                "PIE: #{node_id} ({kind_name}) sof-hisob tugun exec zanjirida bo'lishi mumkin emas"
            ),
        }
    }

    pub fn unregistered(node_id: usize, kind_name: &str) -> Self {
        Self {
            kind: PieErrorKind::UnregisteredNode,
            node_id: Some(node_id),
            message: format!(
                "PIE: #{node_id} ({kind_name}) ro'yxatga olinmagan node turi"
            ),
        }
    }

    pub fn entity(node_id: usize, detail: impl Into<String>) -> Self {
        Self {
            kind: PieErrorKind::EntityError,
            node_id: Some(node_id),
            message: format!("PIE entity #{node_id}: {}", detail.into()),
        }
    }
}

impl fmt::Display for PieError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(id) = self.node_id {
            write!(f, "[{}@node#{}] {}", self.kind, id, self.message)
        } else {
            write!(f, "[{}] {}", self.kind, self.message)
        }
    }
}

/// `String` ga aylantirish — mavjud `Result<(), String>` API bilan mos.
impl From<PieError> for String {
    fn from(e: PieError) -> Self {
        e.to_string()
    }
}
