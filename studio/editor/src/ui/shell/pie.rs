//! PIE start/stop toolbar actions.

use bevy::prelude::*;

use crate::graph::GraphResource;
use crate::interpreter::{
    pie_stop, PieEntityHolder, PieGoldHolder, PiePendingGraph, PieSession, PieStartRequested,
};
use crate::interpreter::resources::{BlackboardResource, ScriptActor};
use crate::state::{PlayState, TerminalState};
use crate::ui::components::{RunButton, StopButton};

use super::save_load::do_export;

/// PIE Start/Stop toolbar actions — entity tizimini ham boshqaradi.
pub fn pie_control_buttons(
    mut commands: Commands,
    graph: Res<GraphResource>,
    mut terminal: ResMut<TerminalState>,
    mut play: ResMut<PlayState>,
    mut pie_start: ResMut<PieStartRequested>,
    mut pending: ResMut<PiePendingGraph>,
    mut session: ResMut<PieSession>,
    mut entity_table: ResMut<crate::interpreter::PieEntityTable>,
    mut blackboard: ResMut<BlackboardResource>,
    holders: Query<Entity, With<PieGoldHolder>>,
    entity_holders: Query<Entity, With<PieEntityHolder>>,
    scene_actors: Query<Entity, With<ScriptActor>>,
    run_q:  Query<&Interaction, (Changed<Interaction>, With<RunButton>)>,
    stop_q: Query<&Interaction, (Changed<Interaction>, With<StopButton>)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for interaction in &run_q {
        if *interaction == Interaction::Pressed {
            terminal.log("[start] PIE — eksport + Play-in-Editor…");
            do_export(&graph, &mut terminal, &mut play);
            pie_start.0 = true;
            *play = PlayState::Playing;
        }
    }

    let stop_pressed = stop_q.iter().any(|i| *i == Interaction::Pressed)
        || keys.just_pressed(KeyCode::Escape);
    if stop_pressed && session.active {
        pie_stop(
            &mut session, &mut commands, holders, entity_holders,
            &mut entity_table, &mut blackboard, &mut terminal, scene_actors,
        );
        pending.0 = None;
        *play = PlayState::Idle;
    }
}
