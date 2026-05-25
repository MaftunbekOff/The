//! Twelfth Hybrid Engine dev editor.

use bevy::prelude::*;
use twelfth_editor::TwelfthEditorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TwelfthEditorPlugin)
        .run();
}
