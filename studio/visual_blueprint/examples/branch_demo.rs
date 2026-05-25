//! v3: Branch + `CheckGoldAmount` dataflow.
//!
//! ```sh
//! cargo run -p twelfth_visual_blueprint --example branch_demo
//! ```

use bevy::prelude::*;
use twelfth_visual_blueprint::Gold;
use visual_blueprint_macros::visual_blueprint;

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

fn spawn_gold(mut commands: Commands) {
    commands.spawn((Gold { value: 1500.0 }, Name::new("PlayerGold")));
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, spawn_gold.before(generated_begin_play_system))
        .add_plugins(RichBranchPlugin)
        .run();
}
