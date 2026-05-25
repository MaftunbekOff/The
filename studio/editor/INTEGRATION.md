# Editor ↔ Visual Blueprint integratsiyasi

## Oqim

```text
twelfth_editor (kanvas)
    → GraphResource::to_blueprint_graph()
    → validate_graph + graph_to_dsl()
    → studio/editor/generated/blueprint.rs
    → visual_blueprint! (compile-time)
    → twelfth_editor_play (Bevy)
```

## Tugunlar (palette)

`twelfth_visual_blueprint::NodeKind` — 7 tur: Begin Play, Tick, Print Log, Add Gold, Branch, Check Gold.

- **Exec** portlar (ko‘k): `exec_in`, `exec_out`, `true`, `false`
- **Data** portlar (sariq): `condition`, `result`, `message`, `amount`

## Buyruqlar

| Tugma | Vazifa |
|-------|--------|
| **Save** | `generated/blueprint.rs` yozish |
| **Run** | Xuddi shu eksport + terminalda `cargo run -p twelfth_editor_play` |
| **Stop** | Holatni tozalash |

## `twelfth_vm`

**Deprecated** — editor endi VM interpretatsiyasidan foydalanmaydi. Faqat tarixiy reference.

## Ishga tushirish

```sh
cargo run -p twelfth_editor
# kanvasda o‘zgartiring, Run bosing
cargo run -p twelfth_editor_play
```
