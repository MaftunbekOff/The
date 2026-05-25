//! PIE gold entity spawn.

use bevy::prelude::*;
use twelfth_visual_blueprint::{ast::VisualScriptGraph, Gold};

use super::resources::PieGoldHolder;

pub(crate) fn ensure_pie_gold(
    commands: &mut Commands,
    graph: &VisualScriptGraph,
    existing: &Query<Entity, With<PieGoldHolder>>,
) {
    if !graph.needs_gold_query() || !existing.is_empty() {
        return;
    }
    commands.spawn((
        Gold { value: 1500.0 },
        PieGoldHolder,
        Name::new("PIE_PlayerGold"),
    ));
}
