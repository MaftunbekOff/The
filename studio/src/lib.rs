//! Twelfth Hybrid Engine — studio plugins. Keep `crates/bevy_*` unchanged.

#![forbid(unsafe_code)]
#![allow(non_snake_case)] // crate name `The` is intentional

use bevy::prelude::*;

/// Root plugin for Twelfth Hybrid Engine extensions.
pub struct ThePlugin;

impl Plugin for ThePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, log_the_ready);
    }
}

fn log_the_ready() {
    info!("Twelfth Hybrid Engine: studio ready (`studio/`, not `crates/bevy_*`)");
}
