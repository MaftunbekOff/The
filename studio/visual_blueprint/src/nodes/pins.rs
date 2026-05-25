//! Pin layout specs for every `NodeKind`.

use super::{NodeKind, VisualNode};
use crate::pins::{PinSpec, PinType};

impl VisualNode {
    pub fn exec_inputs(&self) -> &'static [PinSpec] {
        use NodeKind::*;
        match self.kind {
            // Events — no exec input (they ARE the entry point)
            EventBeginPlay | EventTick | EventCustomBegin
            | EventFixedTick | EventOnSpawn => &[],

            // Pure nodes — no exec
            FloatAdd | FloatSubtract | FloatMultiply | FloatDivide | FloatMin | FloatMax | FloatPow
            | FloatNegate | FloatAbs | FloatSqrt | FloatSin | FloatCos
            | FloatFloor | FloatCeil | FloatRound
            | FloatLerp | FloatClamp
            | FloatGreater | FloatLess | FloatGreaterEqual | FloatLessEqual | FloatEqual
            | BoolAnd | BoolOr | BoolNot | BoolXor
            | IsKeyPressed | IsKeyJustPressed
            | Vec3Make | Vec3Add | Vec3Sub | Vec3Scale | Vec3Length | Vec3Normalize | Vec3Dot | Vec3Lerp
            | StringConcat | FloatToString | BoolToString | IntToString
            | IntAdd | IntSubtract | IntMultiply | IntDivide | IntModulo
            | FloatToInt | IntToFloat
            | IntGreater | IntLess | IntEqual
            | GetNamedEntity | GetTranslation | GetScale | GetRotationEuler
            | GetFloatVar | GetBoolVar | GetIntVar | GetStringVar
            | FloatArrayGet | FloatArrayLength
            | RandomFloat | GetGameTime | IsValidEntity
            | SelectFloat | SelectBool | SelectInt | SelectString
            | IntArrayGet | IntArrayLength
            | StringArrayGet | StringArrayLength
            | Vec2Make | Vec2Add | Vec2Sub | Vec2Scale | Vec2Length
            | Vec2Normalize | Vec2Dot | Vec2X | Vec2Y
            | Comment | GetSelfEntity
            // ECS pure nodes
            | QueryAllEntities | QueryByTag | EntityArrayGet | EntityArrayLength
            | HasTag | GetEntityName
                => &[],

            // Exec-chain nodes
            PrintLog | Delay | AddGold | Branch | CheckGoldAmount
            | SpawnEntity | DestroyEntity | SetTranslation | Translate | SetScale | SetRotationEuler
            | FireCustomEvent
            | SetFloatVar | SetBoolVar | SetIntVar | SetStringVar
            | FloatArrayPush | FloatArrayClear
            | Sequence | DoOnce | ForEachFloat | WhileLoop
            | BreakLoop | ResetDoOnce | ContinueLoop
            | IntArrayPush | IntArrayClear
            | StringArrayPush | StringArrayClear
            // ECS exec nodes
            | AddTag | RemoveTag | ForEachEntity
                => &[PinSpec { name: "exec_in", ty: PinType::Exec }],
        }
    }

    pub fn exec_outputs(&self) -> &'static [PinSpec] {
        use NodeKind::*;
        match self.kind {
            // Pure nodes — no exec out
            FloatAdd | FloatSubtract | FloatMultiply | FloatDivide | FloatMin | FloatMax | FloatPow
            | FloatNegate | FloatAbs | FloatSqrt | FloatSin | FloatCos
            | FloatFloor | FloatCeil | FloatRound
            | FloatLerp | FloatClamp
            | FloatGreater | FloatLess | FloatGreaterEqual | FloatLessEqual | FloatEqual
            | BoolAnd | BoolOr | BoolNot | BoolXor
            | IsKeyPressed | IsKeyJustPressed
            | Vec3Make | Vec3Add | Vec3Sub | Vec3Scale | Vec3Length | Vec3Normalize | Vec3Dot | Vec3Lerp
            | StringConcat | FloatToString | BoolToString | IntToString
            | IntAdd | IntSubtract | IntMultiply | IntDivide | IntModulo
            | FloatToInt | IntToFloat
            | IntGreater | IntLess | IntEqual
            | GetNamedEntity | GetTranslation | GetScale | GetRotationEuler
            | GetFloatVar | GetBoolVar | GetIntVar | GetStringVar
            | FloatArrayGet | FloatArrayLength
            | RandomFloat | GetGameTime | IsValidEntity
            | BreakLoop | ContinueLoop
            | SelectFloat | SelectBool | SelectInt | SelectString
            | IntArrayGet | IntArrayLength
            | StringArrayGet | StringArrayLength
            | Vec2Make | Vec2Add | Vec2Sub | Vec2Scale | Vec2Length
            | Vec2Normalize | Vec2Dot | Vec2X | Vec2Y
            | Comment | GetSelfEntity
            // ECS pure nodes
            | QueryAllEntities | QueryByTag | EntityArrayGet | EntityArrayLength
            | HasTag | GetEntityName
                => &[],

            // Branch — two exec outputs
            Branch => &[
                PinSpec { name: "true",  ty: PinType::Exec },
                PinSpec { name: "false", ty: PinType::Exec },
            ],

            // Loops — body + completed
            ForEachFloat | WhileLoop
                => &[
                    PinSpec { name: "loop_body", ty: PinType::Exec },
                    PinSpec { name: "completed", ty: PinType::Exec },
                ],
            // ECS ForEachEntity loop
            ForEachEntity => &[
                PinSpec { name: "loop_body", ty: PinType::Exec },
                PinSpec { name: "completed", ty: PinType::Exec },
            ],

            // Sequence — 2 outputs + fallthrough
            Sequence => &[
                PinSpec { name: "then_0",   ty: PinType::Exec },
                PinSpec { name: "then_1",   ty: PinType::Exec },
                PinSpec { name: "exec_out", ty: PinType::Exec },
            ],

            // Simple exec through
            EventBeginPlay | EventTick | EventCustomBegin
            | EventFixedTick | EventOnSpawn
            | PrintLog | Delay | AddGold | CheckGoldAmount
            | SpawnEntity | DestroyEntity | SetTranslation | Translate | SetScale | SetRotationEuler
            | FireCustomEvent
            | SetFloatVar | SetBoolVar | SetIntVar | SetStringVar
            | FloatArrayPush | FloatArrayClear | DoOnce | ResetDoOnce
            | IntArrayPush | IntArrayClear
            | StringArrayPush | StringArrayClear
            // ECS exec through
            | AddTag | RemoveTag
                => &[PinSpec { name: "exec_out", ty: PinType::Exec }],
        }
    }

    pub fn data_inputs(&self) -> &'static [PinSpec] {
        use NodeKind::*;
        match self.kind {
            EventBeginPlay | EventTick | EventFixedTick | CheckGoldAmount => &[],
            EventCustomBegin | EventOnSpawn => &[],
            PrintLog => &[PinSpec { name: "message",   ty: PinType::String }],
            Delay    => &[PinSpec { name: "duration",  ty: PinType::Float  }],
            AddGold  => &[PinSpec { name: "amount",    ty: PinType::Float  }],
            Branch   => &[PinSpec { name: "condition", ty: PinType::Bool   }],

            FloatAdd | FloatSubtract | FloatMultiply | FloatDivide | FloatMin | FloatMax => &[
                PinSpec { name: "a", ty: PinType::Float },
                PinSpec { name: "b", ty: PinType::Float },
            ],
            FloatPow => &[
                PinSpec { name: "base", ty: PinType::Float },
                PinSpec { name: "exp",  ty: PinType::Float },
            ],
            FloatNegate | FloatAbs | FloatSqrt | FloatSin | FloatCos
            | FloatFloor | FloatCeil | FloatRound => &[
                PinSpec { name: "a", ty: PinType::Float },
            ],
            FloatLerp => &[
                PinSpec { name: "a", ty: PinType::Float },
                PinSpec { name: "b", ty: PinType::Float },
                PinSpec { name: "t", ty: PinType::Float },
            ],
            FloatClamp => &[
                PinSpec { name: "value", ty: PinType::Float },
                PinSpec { name: "min",   ty: PinType::Float },
                PinSpec { name: "max",   ty: PinType::Float },
            ],
            FloatGreater | FloatLess | FloatGreaterEqual | FloatLessEqual => &[
                PinSpec { name: "a", ty: PinType::Float },
                PinSpec { name: "b", ty: PinType::Float },
            ],
            FloatEqual => &[
                PinSpec { name: "a",   ty: PinType::Float },
                PinSpec { name: "b",   ty: PinType::Float },
                PinSpec { name: "eps", ty: PinType::Float },
            ],
            BoolAnd | BoolOr | BoolXor => &[
                PinSpec { name: "a", ty: PinType::Bool },
                PinSpec { name: "b", ty: PinType::Bool },
            ],
            BoolNot => &[PinSpec { name: "a", ty: PinType::Bool }],
            IsKeyPressed | IsKeyJustPressed => &[],
            Vec3Make => &[
                PinSpec { name: "x", ty: PinType::Float },
                PinSpec { name: "y", ty: PinType::Float },
                PinSpec { name: "z", ty: PinType::Float },
            ],
            Vec3Add | Vec3Sub => &[
                PinSpec { name: "a", ty: PinType::Vec3 },
                PinSpec { name: "b", ty: PinType::Vec3 },
            ],
            Vec3Scale => &[
                PinSpec { name: "vec",   ty: PinType::Vec3  },
                PinSpec { name: "scale", ty: PinType::Float },
            ],
            Vec3Length | Vec3Normalize => &[PinSpec { name: "vec", ty: PinType::Vec3 }],
            Vec3Dot => &[
                PinSpec { name: "a", ty: PinType::Vec3 },
                PinSpec { name: "b", ty: PinType::Vec3 },
            ],
            Vec3Lerp => &[
                PinSpec { name: "a", ty: PinType::Vec3  },
                PinSpec { name: "b", ty: PinType::Vec3  },
                PinSpec { name: "t", ty: PinType::Float },
            ],
            StringConcat => &[
                PinSpec { name: "string_a", ty: PinType::String },
                PinSpec { name: "string_b", ty: PinType::String },
            ],
            FloatToString => &[PinSpec { name: "float_val", ty: PinType::Float }],
            BoolToString  => &[PinSpec { name: "a",         ty: PinType::Bool  }],
            IntToString   => &[PinSpec { name: "int_val",   ty: PinType::Int   }],
            IntAdd | IntSubtract | IntMultiply | IntDivide | IntModulo => &[
                PinSpec { name: "int_a", ty: PinType::Int },
                PinSpec { name: "int_b", ty: PinType::Int },
            ],
            FloatToInt => &[PinSpec { name: "float_val", ty: PinType::Float }],
            IntToFloat => &[PinSpec { name: "int_val",   ty: PinType::Int   }],
            IntGreater | IntLess | IntEqual => &[
                PinSpec { name: "int_a", ty: PinType::Int },
                PinSpec { name: "int_b", ty: PinType::Int },
            ],
            GetNamedEntity => &[],
            SpawnEntity    => &[],
            DestroyEntity  => &[PinSpec { name: "entity", ty: PinType::Entity }],
            GetTranslation => &[PinSpec { name: "entity", ty: PinType::Entity }],
            SetTranslation => &[
                PinSpec { name: "entity",   ty: PinType::Entity },
                PinSpec { name: "position", ty: PinType::Vec3   },
            ],
            Translate => &[
                PinSpec { name: "entity", ty: PinType::Entity },
                PinSpec { name: "delta",  ty: PinType::Vec3   },
            ],
            GetScale => &[PinSpec { name: "entity", ty: PinType::Entity }],
            SetScale => &[
                PinSpec { name: "entity", ty: PinType::Entity },
                PinSpec { name: "scale",  ty: PinType::Vec3   },
            ],
            GetRotationEuler => &[PinSpec { name: "entity",   ty: PinType::Entity }],
            SetRotationEuler => &[
                PinSpec { name: "entity",   ty: PinType::Entity },
                PinSpec { name: "rotation", ty: PinType::Vec3   },
            ],
            FireCustomEvent  => &[],

            // Variables — name is param (string_a)
            SetFloatVar  => &[PinSpec { name: "value",   ty: PinType::Float  }],
            SetBoolVar   => &[PinSpec { name: "value",   ty: PinType::Bool   }],
            SetIntVar    => &[PinSpec { name: "int_val", ty: PinType::Int    }],
            SetStringVar => &[PinSpec { name: "text",    ty: PinType::String }],
            GetFloatVar | GetBoolVar | GetIntVar | GetStringVar => &[],

            // Float Collections
            FloatArrayPush  => &[PinSpec { name: "value", ty: PinType::Float }],
            FloatArrayGet   => &[PinSpec { name: "index", ty: PinType::Int   }],
            FloatArrayLength | FloatArrayClear => &[],

            // Loops
            WhileLoop    => &[PinSpec { name: "condition", ty: PinType::Bool }],
            ForEachFloat => &[],
            Sequence | DoOnce => &[],

            // Helpers
            RandomFloat => &[
                PinSpec { name: "min", ty: PinType::Float },
                PinSpec { name: "max", ty: PinType::Float },
            ],
            GetGameTime   => &[],
            IsValidEntity => &[PinSpec { name: "entity", ty: PinType::Entity }],

            // Loop control
            BreakLoop | ResetDoOnce | ContinueLoop => &[],

            // Select ternary
            SelectFloat => &[
                PinSpec { name: "condition", ty: PinType::Bool  },
                PinSpec { name: "value_a",   ty: PinType::Float },
                PinSpec { name: "value_b",   ty: PinType::Float },
            ],
            SelectBool => &[
                PinSpec { name: "condition", ty: PinType::Bool },
                PinSpec { name: "value_a",   ty: PinType::Bool },
                PinSpec { name: "value_b",   ty: PinType::Bool },
            ],
            SelectInt => &[
                PinSpec { name: "condition", ty: PinType::Bool },
                PinSpec { name: "value_a",   ty: PinType::Int  },
                PinSpec { name: "value_b",   ty: PinType::Int  },
            ],
            SelectString => &[
                PinSpec { name: "condition", ty: PinType::Bool   },
                PinSpec { name: "value_a",   ty: PinType::String },
                PinSpec { name: "value_b",   ty: PinType::String },
            ],

            // Int Arrays
            IntArrayPush  => &[PinSpec { name: "int_val", ty: PinType::Int }],
            IntArrayGet   => &[PinSpec { name: "index",   ty: PinType::Int }],
            IntArrayLength | IntArrayClear => &[],

            // String Arrays
            StringArrayPush  => &[PinSpec { name: "text",  ty: PinType::String }],
            StringArrayGet   => &[PinSpec { name: "index", ty: PinType::Int    }],
            StringArrayLength | StringArrayClear => &[],

            // Vec2
            Vec2Make => &[
                PinSpec { name: "x", ty: PinType::Float },
                PinSpec { name: "y", ty: PinType::Float },
            ],
            Vec2Add | Vec2Sub => &[
                PinSpec { name: "a", ty: PinType::Vec2 },
                PinSpec { name: "b", ty: PinType::Vec2 },
            ],
            Vec2Scale => &[
                PinSpec { name: "vec",   ty: PinType::Vec2  },
                PinSpec { name: "scale", ty: PinType::Float },
            ],
            Vec2Length | Vec2Normalize => &[PinSpec { name: "vec", ty: PinType::Vec2 }],
            Vec2Dot => &[
                PinSpec { name: "a", ty: PinType::Vec2 },
                PinSpec { name: "b", ty: PinType::Vec2 },
            ],
            Vec2X | Vec2Y => &[PinSpec { name: "vec", ty: PinType::Vec2 }],

            // Comment + ScriptActor
            Comment       => &[],
            GetSelfEntity => &[],

            // ── ECS nodes ─────────────────────────────────────────────────────
            // Query — pure, no data inputs (use all scene entities)
            QueryAllEntities | EntityArrayLength => &[],
            // QueryByTag: tag name is param (string_a), no pin input
            QueryByTag => &[],
            // EntityArrayGet: index input
            EntityArrayGet => &[PinSpec { name: "index", ty: PinType::Int }],
            // ForEachEntity: no data input (iterates all)
            ForEachEntity => &[],
            // Tag ops: entity + tag name is param (string_a)
            AddTag | RemoveTag
                => &[PinSpec { name: "entity", ty: PinType::Entity }],
            HasTag
                => &[PinSpec { name: "entity", ty: PinType::Entity }],
            // GetEntityName: entity input
            GetEntityName
                => &[PinSpec { name: "entity", ty: PinType::Entity }],
        }
    }

    pub fn data_outputs(&self) -> &'static [PinSpec] {
        use NodeKind::*;
        match self.kind {
            EventTick | EventFixedTick
                => &[PinSpec { name: "delta_time", ty: PinType::Float }],
            EventOnSpawn
                => &[PinSpec { name: "entity", ty: PinType::Entity }],
            CheckGoldAmount
                => &[PinSpec { name: "result", ty: PinType::Bool }],

            FloatAdd | FloatSubtract | FloatMultiply | FloatDivide | FloatMin | FloatMax | FloatPow
            | FloatNegate | FloatAbs | FloatSqrt | FloatSin | FloatCos
            | FloatFloor | FloatCeil | FloatRound | FloatLerp | FloatClamp
                => &[PinSpec { name: "result", ty: PinType::Float }],

            FloatGreater | FloatLess | FloatGreaterEqual | FloatLessEqual | FloatEqual
            | BoolAnd | BoolOr | BoolNot | BoolXor
                => &[PinSpec { name: "result", ty: PinType::Bool }],

            IsKeyPressed | IsKeyJustPressed
                => &[PinSpec { name: "pressed", ty: PinType::Bool }],

            Vec3Make | Vec3Add | Vec3Sub | Vec3Scale | Vec3Normalize | Vec3Lerp
                => &[PinSpec { name: "result", ty: PinType::Vec3 }],
            Vec3Length | Vec3Dot
                => &[PinSpec { name: "result", ty: PinType::Float }],

            StringConcat | FloatToString | BoolToString | IntToString
                => &[PinSpec { name: "text", ty: PinType::String }],

            IntAdd | IntSubtract | IntMultiply | IntDivide | IntModulo
                => &[PinSpec { name: "int_result", ty: PinType::Int }],
            IntGreater | IntLess | IntEqual
                => &[PinSpec { name: "result", ty: PinType::Bool }],
            FloatToInt => &[PinSpec { name: "int_result", ty: PinType::Int   }],
            IntToFloat => &[PinSpec { name: "result",     ty: PinType::Float }],

            SpawnEntity    => &[PinSpec { name: "entity", ty: PinType::Entity }],
            GetNamedEntity => &[PinSpec { name: "entity", ty: PinType::Entity }],
            GetTranslation | GetScale | GetRotationEuler
                => &[PinSpec { name: "result", ty: PinType::Vec3 }],

            // Variables
            GetFloatVar  => &[PinSpec { name: "result",     ty: PinType::Float  }],
            GetBoolVar   => &[PinSpec { name: "result",     ty: PinType::Bool   }],
            GetIntVar    => &[PinSpec { name: "int_result", ty: PinType::Int    }],
            GetStringVar => &[PinSpec { name: "text",       ty: PinType::String }],

            // Collections
            FloatArrayGet    => &[PinSpec { name: "result",     ty: PinType::Float }],
            FloatArrayLength => &[PinSpec { name: "int_result", ty: PinType::Int   }],

            // ForEachFloat iteration outputs
            ForEachFloat => &[
                PinSpec { name: "item",  ty: PinType::Float },
                PinSpec { name: "index", ty: PinType::Int   },
            ],

            // Helpers
            RandomFloat   => &[PinSpec { name: "result", ty: PinType::Float }],
            GetGameTime   => &[PinSpec { name: "result", ty: PinType::Float }],
            IsValidEntity => &[PinSpec { name: "result", ty: PinType::Bool  }],

            // Select
            SelectFloat  => &[PinSpec { name: "result",     ty: PinType::Float  }],
            SelectBool   => &[PinSpec { name: "result",     ty: PinType::Bool   }],
            SelectInt    => &[PinSpec { name: "int_result", ty: PinType::Int    }],
            SelectString => &[PinSpec { name: "text",       ty: PinType::String }],

            IntArrayGet    => &[PinSpec { name: "int_result", ty: PinType::Int }],
            IntArrayLength => &[PinSpec { name: "int_result", ty: PinType::Int }],

            StringArrayGet    => &[PinSpec { name: "text",       ty: PinType::String }],
            StringArrayLength => &[PinSpec { name: "int_result", ty: PinType::Int    }],

            Vec2Make | Vec2Add | Vec2Sub | Vec2Scale | Vec2Normalize
                => &[PinSpec { name: "result", ty: PinType::Vec2  }],
            Vec2Length | Vec2Dot | Vec2X | Vec2Y
                => &[PinSpec { name: "result", ty: PinType::Float }],

            // ── ECS nodes ─────────────────────────────────────────────────────
            // ForEachEntity — entity + index per iteration
            ForEachEntity => &[
                PinSpec { name: "entity", ty: PinType::Entity },
                PinSpec { name: "index",  ty: PinType::Int    },
            ],
            // QueryAllEntities / QueryByTag — return count of matches (entities via ForEachEntity)
            QueryAllEntities | QueryByTag
                => &[PinSpec { name: "count", ty: PinType::Int }],
            // EntityArrayGet — single entity by index
            EntityArrayGet
                => &[PinSpec { name: "entity", ty: PinType::Entity }],
            // EntityArrayLength
            EntityArrayLength
                => &[PinSpec { name: "int_result", ty: PinType::Int }],
            // HasTag → Bool
            HasTag => &[PinSpec { name: "result", ty: PinType::Bool }],
            // GetEntityName → String
            GetEntityName => &[PinSpec { name: "name", ty: PinType::String }],

            // No outputs: events, exec nodes, annotations
            _ => &[],
        }
    }
}
