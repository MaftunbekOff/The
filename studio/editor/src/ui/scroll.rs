//! Mouse-wheel scrolling for overflow UI nodes (terminal, etc.).

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::hover::HoverMap,
    prelude::*,
};

const LINE_HEIGHT: f32 = 18.0;

/// Injects scroll events into the UI hierarchy under the cursor.
pub fn send_scroll_events(
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    for mouse_wheel in mouse_wheel_reader.read() {
        let mut delta = -Vec2::new(mouse_wheel.x, mouse_wheel.y);

        if mouse_wheel.unit == MouseScrollUnit::Line {
            delta *= LINE_HEIGHT;
        }

        if keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
            core::mem::swap(&mut delta.x, &mut delta.y);
        }

        for pointer_map in hover_map.values() {
            for entity in pointer_map.keys().copied() {
                commands.trigger(Scroll { entity, delta });
            }
        }
    }
}

/// UI scrolling event.
#[derive(EntityEvent, Debug)]
#[entity_event(propagate, auto_propagate)]
pub(crate) struct Scroll {
    entity: Entity,
    delta: Vec2,
}

pub fn on_scroll_handler(
    mut scroll: On<Scroll>,
    mut query: Query<(&mut ScrollPosition, &Node, &ComputedNode)>,
) {
    let Ok((mut scroll_position, node, computed)) = query.get_mut(scroll.entity) else {
        return;
    };

    let max_offset =
        (computed.content_size() - computed.size()).max(Vec2::ZERO) * computed.inverse_scale_factor;

    let delta = &mut scroll.delta;
    if node.overflow.y == OverflowAxis::Scroll && delta.y != 0. {
        let at_end = if delta.y > 0. {
            scroll_position.y >= max_offset.y
        } else {
            scroll_position.y <= 0.
        };

        if !at_end {
            scroll_position.y = (scroll_position.y + delta.y).clamp(0., max_offset.y);
            delta.y = 0.;
        }
    }

    if node.overflow.x == OverflowAxis::Scroll && delta.x != 0. {
        let at_end = if delta.x > 0. {
            scroll_position.x >= max_offset.x
        } else {
            scroll_position.x <= 0.
        };

        if !at_end {
            scroll_position.x = (scroll_position.x + delta.x).clamp(0., max_offset.x);
            delta.x = 0.;
        }
    }

    if *delta == Vec2::ZERO {
        scroll.propagate(false);
    }
}
