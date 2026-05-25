//! Selection, terminal, and status sync systems.

use bevy::prelude::*;
use bevy::text::EditableText;

use crate::graph::VmNode;
use crate::interpreter::PieRuntimeError;
use crate::state::{PaletteFilter, PlayState, SelectedNode, TerminalState};
use crate::ui::components::{PaletteSearchInput, StatusLabel, TerminalLabel, TerminalScrollArea};
use crate::ui::nodes::palette::AddNodeButton;
use crate::ui::nodes::theme::{
    BG_NODE, BG_NODE_SELECTED, EVENT_BODY_BG, StyledBlueprintNode,
};

/// Xatoli node ustiga qizil renkli borderColor qo'shadi.
pub fn sync_pie_error_badge(
    pie_error: Res<PieRuntimeError>,
    mut nodes: Query<(&VmNode, &mut BorderColor)>,
) {
    if !pie_error.is_changed() {
        return;
    }
    let error_id = pie_error.failing_node_id;
    for (vm, mut border) in &mut nodes {
        if error_id == Some(vm.id.0) {
            // Qizil ramka — xato chiqqan node
            *border = BorderColor::all(Color::srgb(0.95, 0.22, 0.22));
        }
    }
}

pub fn sync_node_selection_visual(
    selected: Res<SelectedNode>,
    mut nodes: Query<(
        &VmNode,
        &mut BackgroundColor,
        &mut BorderColor,
        &mut Node,
        Option<&StyledBlueprintNode>,
    )>,
) {
    if !selected.is_changed() {
        return;
    }
    for (vm, mut bg, mut border, mut node, styled) in &mut nodes {
        let is_sel = selected.is_selected(vm.id);
        if let Some(StyledBlueprintNode(kind)) = styled {
            *bg = BackgroundColor(EVENT_BODY_BG);
            border.set_all(crate::ui::nodes::styled_border(is_sel, *kind));
        } else {
            *bg = if is_sel {
                BackgroundColor(BG_NODE_SELECTED)
            } else {
                BackgroundColor(BG_NODE)
            };
            border.set_all(if is_sel {
                Color::srgb(0.45, 0.65, 0.95)
            } else {
                Color::srgb(0.32, 0.36, 0.45)
            });
        }
        node.border = UiRect::all(px(if is_sel { 2 } else { 1 }));
    }
}

pub fn sync_terminal_text(
    terminal: Res<TerminalState>,
    mut text_q: Query<&mut Text, With<TerminalLabel>>,
    mut scroll_q: Query<(&mut ScrollPosition, &ComputedNode), With<TerminalScrollArea>>,
) {
    let Ok(mut text) = text_q.single_mut() else {
        return;
    };

    let content_changed = terminal.is_changed();
    **text = terminal.text();

    if !content_changed {
        return;
    }

    let Ok((mut scroll_position, computed)) = scroll_q.single_mut() else {
        return;
    };

    let max_offset = (computed.content_size() - computed.size()).max(Vec2::ZERO)
        * computed.inverse_scale_factor;
    scroll_position.y = max_offset.y;
}

pub fn sync_status_text(play: Res<PlayState>, mut q: Query<&mut Text, With<StatusLabel>>) {
    let Ok(mut text) = q.single_mut() else {
        return;
    };
    **text = match *play {
        PlayState::Idle => "Holat: Tayyor".into(),
        PlayState::Playing => "Holat: PIE ▶".into(),
        PlayState::Exported => "Holat: Eksport qilindi ✓".into(),
    };
}

// ── Palette qidiruv ───────────────────────────────────────────────────────────

fn editable_text_str(editable: &EditableText) -> String {
    editable
        .value()
        .into_iter()
        .fold(String::new(), |mut s, part| {
            s.push_str(part);
            s
        })
}

/// Palette qidiruv maydoni o'zgarganda tugma ko'rinishini filtrlaydi.
pub fn sync_palette_search(
    mut filter: ResMut<PaletteFilter>,
    search_q: Query<&EditableText, (With<PaletteSearchInput>, Changed<EditableText>)>,
    mut buttons: Query<(&AddNodeButton, &mut Node)>,
) {
    for editable in &search_q {
        filter.query = editable_text_str(editable).to_lowercase();
    }
    if !filter.is_changed() {
        return;
    }
    let q = filter.query.clone();
    for (btn, mut node) in &mut buttons {
        let label = btn.kind.label().to_lowercase();
        let desc  = btn.kind.description().to_lowercase();
        node.display = if q.is_empty() || label.contains(&q) || desc.contains(&q) {
            Display::DEFAULT
        } else {
            Display::None
        };
    }
}
