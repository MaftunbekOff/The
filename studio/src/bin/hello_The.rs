//! Minimal entry point for Twelfth Hybrid Engine studio work.

use bevy::prelude::*;
use The::ThePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ThePlugin)
        .run();
}
