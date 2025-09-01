
use crate::{common::states::{AppState, DespawnOnStateExit}, stage::stage_builder::stage_creator::TILE_SIZE, stage_editor::enums::EditorItem};

use super::{super::controller::EditorController, editor_renderer::{EditorRenderer, RenderedEditorItem}};
use bevy::prelude::*;


pub fn draw_editor(
    renderer_opt: Option<ResMut<EditorRenderer>>,
    editor_controller_opt: Option<Res<EditorController>>
) {
    if renderer_opt.is_none() || editor_controller_opt.is_none() {
        return;
    }
    let mut renderer = renderer_opt.unwrap();
    let editor_controller = editor_controller_opt.unwrap();

    //nothing to be updated
    if editor_controller.version == renderer.version { return; }
    
    //out of sync, renderer should never be ahead
    if editor_controller.version < renderer.version {
        renderer.full_refresh = true;
        return;
    }

    renderer.full_refresh = true;   //TEMPORARY

}


pub fn refresh_editor_renderer(    
    renderer_opt: Option<ResMut<EditorRenderer>>,
    editor_controller_opt: Option<Res<EditorController>>,
    existing_items: Query<Entity, With<RenderedEditorItem>>,
    mut commands: Commands
) {
    if renderer_opt.is_none() || editor_controller_opt.is_none() {
        return;
    }
    let mut renderer = renderer_opt.unwrap();
    let editor_controller = editor_controller_opt.unwrap();

    if renderer.full_refresh == false { return; }

    for entity in existing_items.iter() {
        commands.entity(entity).despawn();
    }

    // draw editor items
    for grid_pos in editor_controller.stage_grid.keys() {
        let editor_item = editor_controller.stage_grid[grid_pos];

        let atlas = match editor_item {
            EditorItem::Ground => editor_controller.ground_atlas.clone(),
            _ => editor_controller.object_atlas.clone(),
        };

        commands.spawn((
            Sprite {
                image: atlas,
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                rect: Some(EditorRenderer::get_item_icon_atlas_rect(&editor_item)),
                ..default()
            },
            Transform { 
                translation: editor_controller.grid_pos_to_world_grid_pos(*grid_pos), 
                rotation: Quat::from_rotation_z(editor_item.get_rotation()), 
                ..default()
            },
            RenderedEditorItem,
            DespawnOnStateExit::App(AppState::StageEditor)
        ));

    }

    // draw rail grid

    renderer.version = editor_controller.version;
    renderer.full_refresh = false;
}
