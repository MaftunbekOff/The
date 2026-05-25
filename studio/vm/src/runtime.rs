//! Ketma-ket bajarish (editor `visual_script_runner` o‘rniga).

use crate::compile::CompiledScript;
use crate::graph::{NodeId, NodeKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmEvent {
    /// Tugun bajarilmoqda.
    Execute { id: NodeId, kind: NodeKind },
    /// Keyingi tugunga o‘tish (sim mavjud).
    Wire { from: NodeId, to: NodeId },
    /// Skript tugadi.
    Finished,
    /// Zanjir oxirida sim yo‘q.
    StalledAt(NodeId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Phase {
    Running,
    Waiting,
    Finished,
}

/// Visual script runtime — `tick(dt)` chaqiriladi.
#[derive(Debug)]
pub struct VmRuntime {
    script: CompiledScript,
    pc: usize,
    wait_remaining: f32,
    phase: Phase,
}

impl VmRuntime {
    pub fn new(script: CompiledScript) -> Self {
        Self {
            script,
            pc: 0,
            wait_remaining: 0.0,
            phase: Phase::Running,
        }
    }

    /// Birinchi tugun darhol (Run bosilganda).
    pub fn start(&mut self) -> Vec<VmEvent> {
        self.pc = 0;
        self.wait_remaining = 0.0;
        self.phase = Phase::Running;
        self.run_step()
    }

    pub fn is_finished(&self) -> bool {
        self.phase == Phase::Finished
    }

    pub fn tick(&mut self, dt: f32) -> Vec<VmEvent> {
        if self.phase == Phase::Finished {
            return Vec::new();
        }

        if self.phase == Phase::Waiting {
            self.wait_remaining -= dt;
            if self.wait_remaining > 0.0 {
                return Vec::new();
            }
            self.phase = Phase::Running;
        }

        if self.pc >= self.script.steps.len() {
            self.phase = Phase::Finished;
            return vec![VmEvent::Finished];
        }

        self.run_step()
    }

    fn run_step(&mut self) -> Vec<VmEvent> {
        let step = self.script.steps[self.pc];
        let mut events = vec![VmEvent::Execute {
            id: step.id,
            kind: step.kind,
        }];

        if step.kind == NodeKind::End {
            self.pc += 1;
            self.phase = Phase::Finished;
            events.push(VmEvent::Finished);
            return events;
        }

        let next_pc = self.pc + 1;
        if next_pc < self.script.steps.len() {
            let next = self.script.steps[next_pc];
            events.push(VmEvent::Wire {
                from: step.id,
                to: next.id,
            });
            self.wait_remaining = match step.kind {
                NodeKind::Wait => 0.5,
                _ => 0.35,
            };
            self.phase = Phase::Waiting;
            self.pc = next_pc;
        } else {
            events.push(VmEvent::StalledAt(step.id));
            self.phase = Phase::Finished;
            self.pc = next_pc;
        }

        events
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compile::{compile, CompiledScript};
    use crate::graph::{GraphSnapshot, NodeId, NodeKind};

    fn sample_script() -> CompiledScript {
        compile(&GraphSnapshot {
            nodes: vec![
                (NodeId(0), NodeKind::Start),
                (NodeId(1), NodeKind::End),
            ],
            edges: vec![(NodeId(0), NodeId(1))],
        })
        .unwrap()
    }

    #[test]
    fn start_then_finish_after_wait() {
        let mut rt = VmRuntime::new(sample_script());
        let ev = rt.start();
        assert!(matches!(ev.first(), Some(VmEvent::Execute { kind: NodeKind::Start, .. })));
        assert!(!rt.is_finished());
        let done = rt.tick(1.0);
        assert!(done.iter().any(|e| matches!(e, VmEvent::Execute { kind: NodeKind::End, .. })));
        assert!(rt.is_finished());
    }
}
