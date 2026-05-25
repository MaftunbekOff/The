//! Compile-time visual blueprint → Bevy `Startup` (zero runtime graph overhead).
//!
//! ```sh
//! cargo run -p twelfth_visual_blueprint --example treasury_demo
//! ```

use bevy::prelude::*;
use twelfth_visual_blueprint::Gold;
use visual_blueprint_macros::visual_blueprint;

// Makro: `Query<&mut Gold>` — komponent alohida, mantiq faqat generatsiya qilingan tizimda.

visual_blueprint! {
    script GeneratedVisualScriptPlugin;
    startup {
        log "Code Log: Tizim muvaffaqiyatli yuklandi!";
        add_gold 500.0;
    }
}

fn spawn_gold_holder(mut commands: Commands) {
    commands.spawn((Gold::default(), Name::new("GoldHolder")));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_gold_holder.before(generated_begin_play_system))
        .add_plugins(GeneratedVisualScriptPlugin)
        .run();
}
