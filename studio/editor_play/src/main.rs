//! Kanvasdan eksport qilingan blueprint → Bevy.
//!
//! 1. `cargo run -p twelfth_editor` — kanvasda Run/Save
//! 2. `cargo run -p twelfth_editor_play` — generatsiya qilingan tizim

use bevy::prelude::*;
use twelfth_visual_blueprint::Gold;

#[path = "../../editor/generated/blueprint.rs"]
mod generated;

fn spawn_demo_gold(mut commands: Commands) {
    commands.spawn((Gold { value: 1500.0 }, Name::new("PlayerGold")));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_demo_gold.before(generated::generated_begin_play_system))
        .add_plugins(generated::EditorGeneratedPlugin)
        .run();
}
