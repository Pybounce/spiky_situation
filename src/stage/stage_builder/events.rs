use bevy::prelude::*;

use crate::{common::states::{AppState, GameState}, databases::game_db::GameDb, game::game_over::GameOver, shaders::background_shader::BackgroundMaterial, stage::stage_builder::{stage_creator::{StageCreator, TILE_SIZE}, CurrentStageData, StageAssets}};


// Future TODO fix
// To preload a stage, just load in the Stage Handle and store in a resource so it's not unloaded
//^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//#[derive(Debug, Copy, Clone, PartialEq, Eq, Event)]
//pub struct LoadStageEvent {
//    pub stage_id: usize
//}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Event)]
pub struct BuildStageEvent {
    pub stage_id: usize,
    pub gateway_id_opt: Option<usize>
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Event)]
pub struct StageBuildCompleteEvent {
    pub stage_id: usize
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Event)]
pub struct StageBuildFailedEvent {
    pub stage_id: usize
}


pub fn read_stage_build_complete_events(
    mut event_reader: EventReader<StageBuildCompleteEvent>,
    mut game_state: ResMut<NextState<GameState>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for _ in event_reader.read() {
        println!("stage success");
        game_state.set(GameState::Playing);
        app_state.set(AppState::Game);
    }
}

pub fn read_stage_build_failed_events(
    mut event_reader: EventReader<StageBuildFailedEvent>,
    mut game_over_event_writer: EventWriter<GameOver>,
) {
    for _ in event_reader.read() {
        println!("stage fail");
        game_over_event_writer.write(GameOver);
    }
}


/// REQUIRES STAGE LOAD EVENT RAISED </br>
/// Listens for BuildStageEvent. </br>
/// Sets the StageBuilderState to building.
/// (which in turn begins the building of the stage)
pub fn read_stage_build_events(
    mut event_reader: EventReader<BuildStageEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_db: Res<GameDb>,
    mut complete_event_writer: EventWriter<StageBuildCompleteEvent>,
    mut failed_event_writer: EventWriter<StageBuildFailedEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>
) {
    for build_stage_event in event_reader.read() {

        if let Some(stage) = game_db.load_stage(build_stage_event.stage_id) {
            println!("loaded stage");
            let ground_grass_handle: Handle<Image> = asset_server.load("ground_grass.png");
            let ground_snow_handle: Handle<Image> = asset_server.load("ground_snow.png");
            let object_tilemap_handle: Handle<Image> = asset_server.load("object_tilemap.png");
            
            let ground_tiles_handle = match stage.terrain_theme {
                super::stage_asset::TerrainTheme::Grass => ground_grass_handle,
                super::stage_asset::TerrainTheme::Snow => ground_snow_handle,
                super::stage_asset::TerrainTheme::Sand => ground_grass_handle,
            };
            commands.insert_resource(StageAssets {
                stage_objects_handle: object_tilemap_handle.clone(),
                ground_tiles_handle: ground_tiles_handle.clone()
            });

            let background_mesh = meshes.add(Mesh::from(Rectangle::default()));
            let background_mat = materials.add(BackgroundMaterial {});

            let stage_creator = StageCreator::new(&stage, build_stage_event.gateway_id_opt, &ground_tiles_handle, &object_tilemap_handle, &background_mesh, &background_mat);
            if stage_creator.build(&mut commands) {
                println!("built");
                commands.insert_resource(CurrentStageData {
                    stage_id: stage.id,
                    gateway_id_opt: build_stage_event.gateway_id_opt,
                    bounds: Rect::new(-TILE_SIZE, -TILE_SIZE, stage.grid_width as f32 * TILE_SIZE, stage.grid_height as f32 * TILE_SIZE),
                });
                complete_event_writer.write(StageBuildCompleteEvent { stage_id: build_stage_event.stage_id });
                continue;
            }
        }
        failed_event_writer.write(StageBuildFailedEvent { stage_id: build_stage_event.stage_id });
    }
}
