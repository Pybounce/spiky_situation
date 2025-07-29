use bevy::prelude::*;
use controller::EditorController;
use item_icon::*;
use renderer::editor_renderer::EditorRenderer;
use crate::{camera::PixelPerfectTranslation, common::{mouse::MouseData, states::{AppState, DespawnOnStateExit, StageEditorState}}, stage::stage_builder::stage_asset::Stage};

mod enums;
mod controller;
mod item_icon;
pub mod renderer;

pub struct StageEditorPlugin;

impl Plugin for StageEditorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::StageEditor), load_stage_editor_assets)
        .add_systems(Update, check_stage_editor_loaded.run_if(in_state(StageEditorState::Loading)).run_if(in_state(AppState::StageEditor)))
        .add_systems(OnEnter(StageEditorState::InEdit), build_stage_editor)
        .add_systems(OnExit(AppState::StageEditor), teardown_stage_editor)
        .add_systems(Update, (
            (handle_current_item_change, add_item_icon, move_item_icon, update_ground_atlas_indices),
            (handle_rotate, handle_placement, handle_grid_object_removals),
            handle_save,
            move_camera
        ).run_if(in_state(StageEditorState::InEdit)));
    }
}

fn load_stage_editor_assets(
    mut stage_editor_load_details: ResMut<StageEditorLoadDetails>,
    asset_server: Res<AssetServer>
) {
    stage_editor_load_details.template_stage_handle = match stage_editor_load_details.template_stage_id {
        Some(template_stage_id) => asset_server.load(format!("stage_{}.stage", template_stage_id)).into(),
        None => None,
    };
}

fn check_stage_editor_loaded(
    stage_editor_load_details: Res<StageEditorLoadDetails>,
    asset_server: Res<AssetServer>,
    mut app_state: ResMut<NextState<AppState>>,
    mut stage_editor_state: ResMut<NextState<StageEditorState>>,
) {
    if let Some(handle) = &stage_editor_load_details.template_stage_handle {
        match asset_server.load_state(handle) {
            bevy::asset::LoadState::NotLoaded => {
                app_state.set(AppState::StageSelect);
                return;
            },
            bevy::asset::LoadState::Loading => { return; },
            bevy::asset::LoadState::Loaded => (),
            bevy::asset::LoadState::Failed(_) => {
                app_state.set(AppState::StageSelect);
                return;
            },
        }
    }
    stage_editor_state.set(StageEditorState::InEdit);
}

fn build_stage_editor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    stage_assets: Res<Assets<Stage>>,
    stage_editor_load_details: Res<StageEditorLoadDetails>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    let object_atlas: Handle<Image> = asset_server.load("object_tilemap.png");
    let ground_atlas: Handle<Image> = asset_server.load("ground_grass.png");
    
    let editor_controller: EditorController;

    if let Some(handle) = &stage_editor_load_details.template_stage_handle {
        match stage_assets.get(handle) {
            Some(stage) => {editor_controller = EditorController::from_stage(stage, stage.id, &object_atlas, &ground_atlas); },
            None => {
                app_state.set(AppState::StageSelect);
                return;
            },
        }
    }
    else {
        editor_controller = EditorController::new(stage_editor_load_details.new_stage_id, &object_atlas, &ground_atlas);
    }

    commands.insert_resource(editor_controller);
    commands.insert_resource(EditorRenderer::new());

    commands.spawn(Text2dBundle {
        text: Text::from_section("Stage Editor", TextStyle::default()),
        ..default()
    })
    .insert(DespawnOnStateExit::App(AppState::StageEditor));

}

fn teardown_stage_editor(
    mut commands: Commands,
    mut editor_state: ResMut<NextState<StageEditorState>>,

) {
    commands.remove_resource::<EditorController>();
    commands.remove_resource::<EditorRenderer>();
    editor_state.set(StageEditorState::Loading);
}

#[derive(Resource)]
pub struct StageEditorLoadDetails {
    pub template_stage_id: Option<usize>,
    pub new_stage_id: usize,
    pub template_stage_handle: Option<Handle<Stage>>
}

fn handle_placement(
    buttons: Res<ButtonInput<MouseButton>>,
    mut editor_con: ResMut<EditorController>,
    mouse_data: Res<MouseData>
) {
    if buttons.just_pressed(MouseButton::Left) {
        let mouse_pos = editor_con.world_to_grid_pos(mouse_data.world_position.extend(0.0));
        if editor_con.try_place(mouse_pos) {

        }
    }
}

fn handle_grid_object_removals(
    buttons: Res<ButtonInput<MouseButton>>,
    mut editor_con: ResMut<EditorController>,
    mouse_data: Res<MouseData>
) {
    if buttons.just_pressed(MouseButton::Right) {
        let mouse_pos = editor_con.world_to_grid_pos(mouse_data.world_position.extend(0.0));
        editor_con.try_remove(mouse_pos);
    }
}

fn handle_save(
    input: Res<ButtonInput<KeyCode>>,
    mut editor_con: ResMut<EditorController>,
) {
    if input.just_pressed(KeyCode::KeyS) {
        editor_con.try_save();
    }
}

fn handle_rotate(
    input: Res<ButtonInput<KeyCode>>,
    mut editor_con: ResMut<EditorController>
) {
    if input.just_pressed(KeyCode::KeyR) {
        editor_con.try_rotate();
    }
}

//TODO: Potentially move to moving the cam via clicking mouse3
fn move_camera(
    mut query: Query<&mut PixelPerfectTranslation, With<Camera>>,
    mouse_data: Res<MouseData>,
    time: Res<Time>
) {
    const CAMERA_MOVE_DEADZONE: f32 = 0.1;
    const CAMERA_MOVE_SPEED: f32 = 64.0;

    let mut direction = Vec3::ZERO;    
    if mouse_data.window_position_normalised.x >= 1.0 - CAMERA_MOVE_DEADZONE {
        direction += Vec3::X;
    }
    else if mouse_data.window_position_normalised.x <= CAMERA_MOVE_DEADZONE {
        direction -= Vec3::X;
    }
    if mouse_data.window_position_normalised.y <= CAMERA_MOVE_DEADZONE {
        direction += Vec3::Y;
    }
    else if mouse_data.window_position_normalised.y >= 1.0 - CAMERA_MOVE_DEADZONE {
        direction -= Vec3::Y;
    }

    for mut ppt in &mut query {
        ppt.translation += direction * CAMERA_MOVE_SPEED * time.delta_seconds();
    }
}


fn update_ground_atlas_indices(
    mut stage_entities_q: Query<&mut Sprite, Without<ItemIcon>>, 
    editor_con: Res<EditorController>,
    mouse_data: Res<MouseData>,
    mut item_icon_query: Query<&mut Sprite, With<ItemIcon>>,

) {
    //let current_grid_pos = editor_con.world_to_grid_pos(mouse_data.world_position.extend(0.0));
    //let adjacent_grid_positions = get_not_clockwise_adjacent_grid_positions_but_2_layers_hardcoded_because_thats_the_neutron_style(current_grid_pos);
//
    //let mut ground_icon_grid_pos_opt: Option<IVec2> = None;
//
    //match editor_con.current_item {
    //    enums::EditorItem::Ground => {
    //        if let Ok(mut s) = item_icon_query.get_single_mut() {
    //            let atlas_index = get_ground_atlas_index(&editor_con, current_grid_pos, None) as f32;
    //            let upper_left = Vec2::new(atlas_index % GROUND_TILEMAP_SIZE, (atlas_index / GROUND_TILEMAP_SIZE).trunc()) * TILE_SIZE;
    //            let lower_right = upper_left + TILE_SIZE;
    //            let atlas_rect = Rect::new(upper_left.x, upper_left.y, lower_right.x, lower_right.y);
    //            s.rect = Some(atlas_rect);
    //            ground_icon_grid_pos_opt = Some(current_grid_pos);
    //        }
    //    },
    //    _ => (),
    //}
//
    //for adjacent_grid_pos in &adjacent_grid_positions {
    //    if let Some(stage_object) = editor_con.stage_grid.get(adjacent_grid_pos) {
    //        match stage_object {
    //            enums::EditorStageObject::Ground { entity } => {
    //                if let Ok(mut s) = stage_entities_q.get_mut(*entity) {
    //                    let atlas_index = get_ground_atlas_index(&editor_con, *adjacent_grid_pos, ground_icon_grid_pos_opt) as f32;
    //                    let upper_left = Vec2::new(atlas_index % GROUND_TILEMAP_SIZE, (atlas_index / GROUND_TILEMAP_SIZE).trunc()) * TILE_SIZE;
    //                    let lower_right = upper_left + TILE_SIZE;
    //                    let atlas_rect = Rect::new(upper_left.x, upper_left.y, lower_right.x, lower_right.y);
    //                    s.rect = Some(atlas_rect);
    //                }
    //            }
    //            _ => (),
    //        }
    //    }
    //}
    // commented out whilst working on the editor refactor

}

pub fn get_ground_atlas_index(
    editor_con: &EditorController,
    grid_pos: IVec2,
    ground_icon_grid_pos_opt: Option<IVec2>
) -> usize {
    let adjacent_grid_positions = get_clockwise_adjacent_grid_positions(grid_pos);

    let mut bitmask: u8 = 0;
    let mut current_bit: u8 = 1;

    for adjacent_grid_pos in &adjacent_grid_positions {
        if adjacent_grid_pos.x == -1 || adjacent_grid_pos.x == editor_con.grid_size.x || adjacent_grid_pos.y == -1 || adjacent_grid_pos.y == editor_con.grid_size.y {
            bitmask |= current_bit
        }
        if let Some(ground_icon_grid_pos) = ground_icon_grid_pos_opt {
            if ground_icon_grid_pos == *adjacent_grid_pos {
                bitmask |= current_bit;
            }
        }
        if let Some(stage_object) = editor_con.stage_grid.get(adjacent_grid_pos) {
            match stage_object {
                enums::EditorItem::Ground => bitmask |= current_bit,
                _ => (),
            }
        };

        current_bit <<= 1;
    }

    return map_surrounding_ground_bitmask_to_atlas_index(bitmask);

}

fn get_clockwise_adjacent_grid_positions(grid_pos: IVec2) -> Vec<IVec2> {
    return vec![
        IVec2::new(grid_pos.x, grid_pos.y + 1),
        IVec2::new(grid_pos.x + 1, grid_pos.y + 1),
        IVec2::new(grid_pos.x + 1, grid_pos.y),
        IVec2::new(grid_pos.x + 1, grid_pos.y - 1),
        IVec2::new(grid_pos.x, grid_pos.y - 1),
        IVec2::new(grid_pos.x - 1, grid_pos.y - 1),
        IVec2::new(grid_pos.x - 1, grid_pos.y),
        IVec2::new(grid_pos.x - 1, grid_pos.y + 1),
    ];
}
fn get_not_clockwise_adjacent_grid_positions_but_2_layers_hardcoded_because_thats_the_neutron_style(grid_pos: IVec2) -> Vec<IVec2> {
    return vec![
        IVec2::new(grid_pos.x, grid_pos.y + 1),
        IVec2::new(grid_pos.x + 1, grid_pos.y + 1),
        IVec2::new(grid_pos.x + 1, grid_pos.y),
        IVec2::new(grid_pos.x + 1, grid_pos.y - 1),
        IVec2::new(grid_pos.x, grid_pos.y - 1),
        IVec2::new(grid_pos.x - 1, grid_pos.y - 1),
        IVec2::new(grid_pos.x - 1, grid_pos.y),
        IVec2::new(grid_pos.x - 1, grid_pos.y + 1),

        IVec2::new(grid_pos.x, grid_pos.y + 2),
        IVec2::new(grid_pos.x + 1, grid_pos.y + 2),
        IVec2::new(grid_pos.x + 2, grid_pos.y + 2),
        IVec2::new(grid_pos.x - 1, grid_pos.y + 2),
        IVec2::new(grid_pos.x - 2, grid_pos.y) + 2,
        IVec2::new(grid_pos.x - 2, grid_pos.y + 1),
        IVec2::new(grid_pos.x + 2, grid_pos.y + 1),
        IVec2::new(grid_pos.x + 2, grid_pos.y),
        IVec2::new(grid_pos.x - 2, grid_pos.y),
        IVec2::new(grid_pos.x + 2, grid_pos.y - 1),
        IVec2::new(grid_pos.x - 2, grid_pos.y - 1),
        IVec2::new(grid_pos.x, grid_pos.y - 2),
        IVec2::new(grid_pos.x + 1, grid_pos.y - 2),
        IVec2::new(grid_pos.x + 2, grid_pos.y - 2),
        IVec2::new(grid_pos.x - 1, grid_pos.y - 2),
        IVec2::new(grid_pos.x - 2, grid_pos.y) - 2,
    ];
}

pub fn map_surrounding_ground_bitmask_to_atlas_index(bitmask: u8) -> usize {
    let indices = [
        0, 1, 0, 1, 2, 3, 49, 4, 0, 1, 
        0, 1, 2, 3, 49, 4, 5, 6, 5, 6, 
        7, 8, 50, 9, 5, 6, 5, 6, 10, 11, 
        51, 12, 0, 1, 0, 1, 2, 3, 49, 4, 
        0, 1, 0, 1, 2, 3, 49, 4, 5, 6, /* 50 */
        5, 6, 7, 8, 50, 9, 5, 6, 5, 6, 
        10, 11, 51, 12, 13, 14, 13, 14, 15, 16, 
        56, 17, 13, 14, 13, 14, 15, 16, 56, 17, 
        18, 19, 18, 19, 20, 21, 59, 22, 18, 19, 
        18, 19, 23, 24, 62, 25, 13, 14, 13, 14, /* 100 */
        15, 16, 56, 17, 13, 14, 13, 14, 15, 16, 
        56, 17, 26, 27, 26, 27, 28, 29, 65, 30, 
        26, 27, 26, 27, 31, 32, 68, 33, 0, 1, 
        0, 1, 2, 3, 49, 4, 0, 1, 0, 1, 
        2, 3, 49, 4, 5, 6, 5, 6, 7, 8, /* 150 */
        50, 9, 5, 6, 5, 6, 10, 11, 10, 12, 
        0, 1, 0, 1, 2, 3, 2, 4, 0, 1, 
        0, 1, 2, 3, 49, 4, 5, 6, 5, 6, 
        7, 8, 50, 9, 5, 6, 5, 6, 10, 11, 
        51, 12, 52, 34, 52, 34, 55, 35, 57, 36, /* 200 */ 
        52, 34, 52, 34, 55, 35, 57, 36, 53, 37, 
        53, 37, 58, 38, 60, 39, 53, 37, 53, 37, 
        61, 40, 63, 41, 52, 34, 52, 34, 55, 35, 
        57, 36, 52, 34, 52, 34, 55, 35, 57, 36, 
        54, 42, 54, 42, 64, 43, 66, 44, 54, 42, 
        54, 42, 67, 45, 69, 46];
    return indices[bitmask as usize];
}