# Twelfth Hybrid Engine — `studio/`

**The** repozi = **Twelfth Hybrid Engine** (Bevy fork + studiya qatlami).

| Papka | Maqsad |
|-------|--------|
| `src/` | `The` crate — `ThePlugin`, umumiy studiya yadrosi |
| `editor/` | Twelfth Editor (noldan) |
| `vm/` | Visual Script / VM (editor runtime) |
| `visual_blueprint/` | Rust AST → compile-time Bevy systems |
| `../crates/bevy_*` | Bevy upstream — **tegmang** |

## Qoida

> Bevy rivojlantirayotgan joylarga tegmaslik; community qilmayotgan joylarda o‘z qo‘shimchalaringiz.

## Buyruqlar

```sh
cd The
cargo run -p The --bin hello_The
cargo run -p twelfth_editor --bin dev_editor
cargo test -p twelfth_vm
cargo test -p twelfth_visual_blueprint
cargo run -p twelfth_visual_blueprint --example treasury_demo
cargo run --example breakout
```

## Upstream Bevy

```sh
git remote add upstream https://github.com/bevyengine/bevy.git
git fetch upstream && git merge upstream/main
```