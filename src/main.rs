

use bevy::{
    asset::AssetMetaCheck, prelude::*, winit::{ UpdateMode, WinitSettings }
};

mod local_player;
use bevy_rapier2d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use camera::{handle_zoom_change, move_camera, move_pixel_perfect_translations, spawn_camera};
use common::{animated_sprite::{animate_sprites, check_animate_on_touch}, checkpoint::check_checkpoint_reached, death::{check_touched_by_death, despawn_death_marked, delay_death_marked}, mouse::{update_mouse_data, MouseData}, offset_mover::move_offset_movers, physics::{bouncy::check_bouncy_collisions, fragile::break_fragiles, gravity::simulate_gravity}, shake::shake, states::StatesPlugin, triggers::{trigger_on_touch, TriggerEvent}};
use debugging::DebugPlugin;
use game::GamePlugin;

mod networking;
use local_player::{load_player_sprite, update_player_look_direction};
use networking::{networked_players::{remove_disconnected_players, spawn_new_players}, GameNetworkingPlugin};

mod stage_select;
use obstacles::check_insta_kill_collisions;
use player::{common::check_player_out_of_bounds, dash_controller::{apply_dashing, start_dashing}, death::trigger_dead_local_player_respawn, horizontal_movement_controller::{move_airbourne_horizontal_controller, move_ground_horizontal_controller}, jump_controller::{apply_wall_friction, begin_player_jump, check_jump_fall_states, is_coyote_grounded, maintain_player_jump, update_last_grounded}, look_state::{update_player_airborn_look_state, update_player_grounded_look_state}, physics_controller::apply_physics_controller_limits, spawner::spawn_local_players, wall_jump_controller::{add_wall_stuck, begin_player_wall_jump, remove_wall_stuck, update_wall_stuck, update_wall_stuck_time}};
use ground::check_grounded;
use stage::{stage_builder::StageBuilderPlugin, stage_objects::{interval_block::{stop_interval_block_crush, tick_interval_blocks}, lock_block::read_lock_block_triggers, phantom_block::{check_phantom_block_touched, tick_phantom_block}, saw_shooter::tick_saw_shooters}};
use stage_editor::{renderer::systems::{draw_editor, refresh_editor_renderer}, StageEditorPlugin};
use stage_select::StageSelectPlugin;
use wall::check_touching_wall;

use crate::{builders::player_builders::init_player_builder, player::death::spawn_player_corpse};

mod common;

mod game;
mod player;
mod stage;
mod obstacles;
mod camera;
pub mod ground;
pub mod wall;
pub mod stage_editor;
mod debugging;
pub mod builders;

fn main() {
    let winit_settings = WinitSettings {
        focused_mode: UpdateMode::Continuous,
        unfocused_mode: UpdateMode::Continuous,
    };
    let window_settings = WindowPlugin {
        primary_window: Some(Window {
            title: "Legend of the Octo-Parakeet".into(),
            name: Some("bevy_quickstart".into()),
            canvas: Some("#bevy".to_string()),
            fit_canvas_to_parent: true,
            prevent_default_event_handling: true,
            //resolution: (1600.0, 900.0).into(),
            //present_mode: PresentMode::Immediate,
            ..default()
        }),
        ..default()
    };

    
    App::new()
        .insert_resource(winit_settings)
        .insert_resource(Msaa::Off)
        .add_plugins(DefaultPlugins.set(window_settings).set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_plugins(StatesPlugin)
        .add_plugins(StageBuilderPlugin)
        .add_plugins(StageSelectPlugin)
        .add_plugins(StageEditorPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(GameNetworkingPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugins(DebugPlugin)
        .init_resource::<MouseData>()
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (spawn_camera, init_player_builder))
        .add_systems(Update, (handle_zoom_change, move_camera, spawn_new_players, remove_disconnected_players))
        .add_systems(Update, (check_touching_wall, update_wall_stuck_time, apply_wall_friction, begin_player_wall_jump, shake, check_insta_kill_collisions, trigger_dead_local_player_respawn, spawn_local_players, check_grounded, check_player_out_of_bounds, update_last_grounded, maintain_player_jump, begin_player_jump, is_coyote_grounded, check_jump_fall_states, despawn_death_marked, delay_death_marked))
        .add_systems(Update, (apply_physics_controller_limits, add_wall_stuck, update_wall_stuck, remove_wall_stuck))
        .add_systems(Update, (update_player_look_direction, load_player_sprite, simulate_gravity, check_checkpoint_reached, animate_sprites, move_pixel_perfect_translations))
        .add_systems(Update, (start_dashing, break_fragiles, tick_saw_shooters, move_offset_movers, tick_phantom_block, check_phantom_block_touched, stop_interval_block_crush, tick_interval_blocks, check_touched_by_death, read_lock_block_triggers, trigger_on_touch, check_bouncy_collisions, check_animate_on_touch, update_player_airborn_look_state, update_player_grounded_look_state, update_player_look_direction))
        .add_systems(Update, (refresh_editor_renderer, draw_editor, update_mouse_data))
        .add_systems(Update, (move_airbourne_horizontal_controller, move_ground_horizontal_controller, apply_dashing).chain())
        .add_systems(Update, spawn_player_corpse)
        .add_event::<TriggerEvent>()
        .run();
}

