# twelfth_visual_blueprint

Pure Rust vizual skript **ma’lumot qatlami** (v3: shox + dataflow).

Kod generatsiyasi: [`visual_blueprint_macros`](../visual_blueprint_macros/).

## v3 yangiliklari

| Xususiyat | Tavsif |
|-----------|--------|
| **Branch** | `exec_in` + `condition: bool` → `true` / `false` exec chiqishlari → `if/else` |
| **CheckGoldAmount** | `Gold::value` bilan solishtiradi → `result: bool` data chiqishi |
| **data_links** | `let node_N_pin` orqali keyingi tugun kirishiga uzatiladi |
| **Bitta Query** | Bir tizimda `Query<&mut Gold>` bir marta |

## Makro DSL (v3)

```rust
visual_blueprint! {
    script RichBranchPlugin;
    startup {
        let rich = check_gold 1000.0;
        branch rich {
            arm_true { log "Rich!"; }
            arm_false { log "Need more gold!"; }
        }
    }
}
```

`rich` → `CheckGoldAmount.result` → `Branch.condition` (compile-time `bool` tekshiruvi).

## AST graf (dasturiy)

```rust
let graph = twelfth_visual_blueprint::demo_rich_branch_graph();
twelfth_visual_blueprint::validate_graph(&graph)?;
```

## Demo

```sh
cargo run -p twelfth_visual_blueprint --example branch_demo
cargo run -p twelfth_visual_blueprint --example treasury_demo
```

## Testlar

```sh
cargo test -p twelfth_visual_blueprint --no-default-features
cargo test -p visual_blueprint_macros --no-default-features
```

## Tugunlar

| Tugun | Data | Exec |
|-------|------|------|
| `EventBeginPlay` | — | `exec_out` |
| `CheckGoldAmount` | `result: bool` | `exec_in` / `exec_out` |
| `Branch` | `condition: bool` | `true` / `false` |
| `PrintLog` | `message: String` | `exec_in` / `exec_out` |
| `AddGold` | `amount: f32` | `exec_in` / `exec_out` |

Noto‘g‘ri data simi (masalan `bool` → `String`) `validate_graph` da rad etiladi.
