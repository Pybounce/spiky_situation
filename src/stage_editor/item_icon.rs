use bevy::prelude::*;

use crate::{common::{mouse::MouseData, states::{AppState, DespawnOnStateExit}}, stage::stage_builder::stage_creator::TILE_SIZE};

use super::{controller::EditorController, renderer::editor_renderer::EditorRenderer};

#[derive(Component)]
pub struct ItemIcon;

pub fn add_item_icon(
    mut commands: Commands,
    query: Query<Entity, With<ItemIcon>>,
    editor_con: Res<EditorController>,
    
) {
    let mut first_item = true;
    for e in &query {
        if first_item {
            first_item = false;
            continue;
        }

        commands.entity(e).despawn();
    }

    if first_item == true {
        //no item exists
        let atlas = match editor_con.current_item {
            super::enums::EditorItem::Ground => editor_con.ground_atlas.clone(),
            _ => editor_con.object_atlas.clone(),
        };
        commands.spawn((
            Sprite {
                image: atlas,
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                rect: Some(EditorRenderer::get_item_icon_atlas_rect(&editor_con.current_item)),
            ..default()
            },
            ItemIcon,
            DespawnOnStateExit::App(AppState::StageEditor)
        ));
    }
}

pub fn move_item_icon(
    mut item_icon_query: Query<&mut Transform, With<ItemIcon>>,
    mouse_data: Res<MouseData>,
    editor_con: Res<EditorController>,

) {
    if let Ok(mut t) = item_icon_query.single_mut() {
        t.translation = editor_con.world_to_grid_world_pos(mouse_data.world_position.extend(t.translation.z));
        t.rotation = Quat::from_rotation_z(editor_con.current_item.get_rotation());
    }
}

pub fn handle_current_item_change(
    mut editor_con: ResMut<EditorController>,
    input: Res<ButtonInput<KeyCode>>,
    mouse_data: Res<MouseData>,
    mut current_item_q: Query<&mut Sprite, With<ItemIcon>>
) {
    if input.just_pressed(KeyCode::KeyD) {
        editor_con.cycle_next_item()
    }
    else if input.just_pressed(KeyCode::KeyA) {
        editor_con.cycle_prev_item()
    }
    else if input.just_pressed(KeyCode::KeyW) {
        editor_con.cycle_next_item_variant()
    }
    else if input.just_pressed(KeyCode::KeyS) {
        editor_con.cycle_prev_item_variant()
    }
    else if input.just_pressed(KeyCode::KeyQ) {
        let current_grid_pos = editor_con.world_to_grid_pos(mouse_data.world_position.extend(0.0));
        if let Some(editor_item) = editor_con.stage_grid.get(&current_grid_pos) {
            editor_con.current_item = *editor_item;
            
        }
    }
    else {
        return;
    }
    if let Ok(mut sprite) = current_item_q.single_mut() {
        match editor_con.current_item {
            super::enums::EditorItem::Ground => sprite.image = editor_con.ground_atlas.clone(),
            _ => sprite.image = editor_con.object_atlas.clone(),
        }
        sprite.rect = Some(EditorRenderer::get_item_icon_atlas_rect(&editor_con.current_item));
    }
}