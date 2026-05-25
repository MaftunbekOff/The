//! Graf → ketma-ket bajariladigan zanjir.

use crate::graph::{GraphSnapshot, NodeId, NodeKind};

/// Compile xatosi (terminalga chiqadi).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompileError(pub String);

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Bajariladigan tugunlar zanjiri (`Start` dan sim bo‘ylab).
#[derive(Debug, Clone)]
pub struct CompiledScript {
    pub steps: Vec<CompiledStep>,
}

#[derive(Debug, Clone, Copy)]
pub struct CompiledStep {
    pub id: NodeId,
    pub kind: NodeKind,
}

/// Faqat `Start` dan boshlab ulangan zanjirni oladi.
pub fn compile(snapshot: &GraphSnapshot) -> Result<CompiledScript, CompileError> {
    let Some(entry) = snapshot.entry() else {
        return Err(CompileError("Start tuguni yo‘q".into()));
    };

    if snapshot.next_after(entry).is_none() {
        return Err(CompileError(
            "Start dan sim chiqmaydi — avval «out» → «in» bog‘lang".into(),
        ));
    }

    let chain = snapshot.wired_chain_from(entry);
    let steps = chain
        .into_iter()
        .map(|id| {
            let kind = snapshot
                .kind(id)
                .ok_or_else(|| CompileError(format!("tugun #{} topilmadi", id.0)))?;
            Ok(CompiledStep { id, kind })
        })
        .collect::<Result<Vec<_>, CompileError>>()?;

    if steps.len() < 2 {
        return Err(CompileError("Zanjirda kamida Start va yana bitta tugun kerak".into()));
    }

    Ok(CompiledScript { steps })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::GraphSnapshot;

    #[test]
    fn compiles_start_wait_end() {
        let snap = GraphSnapshot {
            nodes: vec![
                (NodeId(0), NodeKind::Start),
                (NodeId(1), NodeKind::Wait),
                (NodeId(2), NodeKind::End),
            ],
            edges: vec![(NodeId(0), NodeId(1)), (NodeId(1), NodeId(2))],
        };
        let script = compile(&snap).unwrap();
        assert_eq!(script.steps.len(), 3);
        assert_eq!(script.steps[0].kind, NodeKind::Start);
        assert_eq!(script.steps[2].kind, NodeKind::End);
    }

    #[test]
    fn rejects_start_without_wire() {
        let snap = GraphSnapshot {
            nodes: vec![(NodeId(0), NodeKind::Start)],
            edges: vec![],
        };
        assert!(compile(&snap).is_err());
    }
}
