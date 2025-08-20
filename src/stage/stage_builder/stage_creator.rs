use crate::{common::checkpoint::CheckpointBundle, player::spawner::LocalPlayerSpawner, shaders::background_shader::BackgroundMaterial, stage::stage_objects::{goal::GoalFactory, half_saw::SawFactory, interval_block::IntervalBlockFactory, key::KeyFactory, lock_block::LockBlockFactory, phantom_block::PhantomBlockFactory, saw_shooter::SawShooterFactory, spike::SpikeFactory, spring::SpringFactory, tiles::{GroundTileBundle, TileBundle}, StageObject}, stage_editor::map_surrounding_ground_bitmask_to_atlas_index};

use super::stage_asset::Stage;
use bevy::prelude::*;

pub const TILE_SIZE: f32 = 16.0;
pub const TILE_SIZE_HALF: f32 = TILE_SIZE / 2.0;
const TILEMAP_SIZE: usize = 7;
const TILEMAP_TILE_SIZE: f32 = 16.0;
const OBJECT_TILEMAP_SIZE: usize = 16;
const OBJECT_TILE_TILEMAP_SIZE: f32 = 16.0;

pub struct StageCreator<'a> {
    pub stage: &'a Stage, 
    pub tilemap: &'a Handle<Image>,
    pub object_tilemap: &'a Handle<Image>,

    pub background_quad_mesh: &'a Handle<Mesh>,
    pub background_material: &'a Handle<BackgroundMaterial>
}

pub enum ObjectAtlasIndices {
    HalfSaw0 = 0,
    HalfSaw1 = 1,
    HalfSaw2 = 2,
    HalfSaw3 = 3,
    Spike = 4,
    Spring0 = 5,
    Spring1 = 6,
    Spring2 = 7,
    Spring3 = 8,
    Spring4 = 9,
    Player = 18,
    Key = 10,
    LockBlock = 11,
    IntervalBlock0 = 12,
    IntervalBlock1 = 13,
    IntervalBlock2 = 14,
    PhantomBlock0 = 21,
    PhantomBlock1 = 22,
    PhantomBlock2 = 23,
    PhantomBlock3 = 24,
    PhantomBlock4 = 25,
    SawProjectile0 = 32,
    SawProjectile1 = 33,
    SawProjectile2 = 34,
    SawShooter = 27,
}



impl<'a> StageCreator<'a> {

    pub fn new(stage: &'a Stage, tilemap: &'a Handle<Image>, object_tilemap: &'a Handle<Image>, background_quad_mesh: &'a Handle<Mesh>, background_material: &'a Handle<BackgroundMaterial>) -> Self {
        StageCreator {
            stage,
            tilemap,
            object_tilemap,
            background_material,
            background_quad_mesh
        }
    }

    pub fn build(&self, commands: &mut Commands) -> bool {
        build_ground(self, commands)
        && build_goal(self, commands)
        && build_background(self, commands)
        && build_spikes(self, commands)
        && build_borders(self, commands)
        && build_player_spawner(self, commands)
        && build_checkpoints(self, commands)
        && build_half_saws(self, commands)
        && build_springs(self, commands)
        && build_lock_blocks(self, commands)
        && build_keys(self, commands)
        && build_interval_blocks(self, commands)
        && build_phantom_blocks(self, commands)
        && build_saw_shooters(self, commands)
    }


}

fn build_player_spawner(stage_creator: &StageCreator, commands: &mut Commands) -> bool {
    commands.spawn(LocalPlayerSpawner {
        spawn_time: 0.0,
        translation: ((stage_creator.stage.spawn_grid_pos * TILE_SIZE) + TILE_SIZE_HALF).extend(0.0),
    });
    return true;
}


fn build_ground(stage_creator: &StageCreator, commands: &mut Commands) -> bool {
    for tile in &stage_creator.stage.ground_tiles {
        build_ground_tile(commands, stage_creator, tile.grid_pos.x, tile.grid_pos.y, tile.tilemap_index);
    }
    return true;
}

fn build_background(stage_creator: &StageCreator, commands: &mut Commands) -> bool {

    let grid_pos = Vec2::new(stage_creator.stage.grid_width as f32 / 2.0, 
    stage_creator.stage.grid_height as f32 / 2.0);
    
    commands.spawn((
        Mesh2d(stage_creator.background_quad_mesh.clone()),
        MeshMaterial2d(stage_creator.background_material.clone()),
        Transform { 
            translation: Vec3::new(grid_pos.x * TILE_SIZE, grid_pos.y * TILE_SIZE, -10.0), 
            scale: Vec3::new(TILE_SIZE * stage_creator.stage.grid_width as f32, TILE_SIZE * stage_creator.stage.grid_height as f32, 1.0),
            ..default()
        },
        StageObject::Volatile
    ));
    return true;
}

fn build_borders(stage_creator: &StageCreator, commands: &mut Commands) -> bool {

    let index = map_surrounding_ground_bitmask_to_atlas_index(u8::MAX);
    let upper_left = Vec2::new((index as f32 % TILEMAP_SIZE as f32) as f32 * TILEMAP_TILE_SIZE, (index / TILEMAP_SIZE) as f32 * TILEMAP_TILE_SIZE);
    let lower_right = Vec2::new(upper_left.x + TILEMAP_TILE_SIZE , upper_left.y + TILEMAP_TILE_SIZE);
    let sprite_rect = Rect::new(upper_left.x, upper_left.y, lower_right.x, lower_right.y);

    let bground_grid_width: f32 = stage_creator.stage.grid_width as f32 * 20.0;
    let bground_grid_height: f32 = stage_creator.stage.grid_height as f32 * 20.0;
    let stage_grid_size = Vec2::new(stage_creator.stage.grid_width as f32, stage_creator.stage.grid_height as f32);

    let mut left = TileBundle::new(
        stage_creator, 
        Vec2::new(-bground_grid_width / 2.0, stage_grid_size.y / 2.0), 
        sprite_rect, 0.0, stage_creator.tilemap
    );
    left.transform.translation.z = 20.0;
    left.transform.scale = Vec3::new(bground_grid_width, bground_grid_height, 1.0);
    commands.spawn(left);

    let mut right = TileBundle::new(
        stage_creator, 
        Vec2::new((bground_grid_width / 2.0) + stage_grid_size.x - 1.0, stage_grid_size.y / 2.0), 
        sprite_rect, 0.0, stage_creator.tilemap
    );
    right.transform.translation.z = 20.0;
    right.transform.scale = Vec3::new(bground_grid_width, bground_grid_height, 1.0);
    commands.spawn(right);

    let mut top = TileBundle::new(
        stage_creator, 
        Vec2::new(stage_grid_size.x / 2.0, (bground_grid_height / 2.0) + stage_grid_size.y - 1.0), 
        sprite_rect, 0.0, stage_creator.tilemap
    );
    top.transform.translation.z = 20.0;
    top.transform.scale = Vec3::new(bground_grid_width, bground_grid_height, 1.0);
    commands.spawn(top);

    let mut bottom = TileBundle::new(
        stage_creator, 
        Vec2::new(stage_grid_size.x / 2.0, -bground_grid_height / 2.0), 
        sprite_rect, 0.0, stage_creator.tilemap
    );
    bottom.transform.translation.z = 20.0;
    bottom.transform.scale = Vec3::new(bground_grid_width, bground_grid_height, 1.0);
    commands.spawn(bottom);


    return true;
}

fn build_goal(stage_creator: &StageCreator, commands: &mut Commands) -> bool {

    let sprite_rect = get_object_tilemap_rect_from_index(ObjectAtlasIndices::Player);
    
    GoalFactory::spawn(
        commands,
        &stage_creator, 
        stage_creator.stage.goal_grid_pos, 
        sprite_rect);
        
    return true;
}

fn build_spikes(stage_creator: &StageCreator, commands: &mut Commands) -> bool {

    let sprite_rect = get_object_tilemap_rect_from_index(ObjectAtlasIndices::Spike);

    for spike in &stage_creator.stage.spikes {

        SpikeFactory::spawn(commands, stage_creator, spike.grid_pos, sprite_rect, spike.rotation);
    }

    return true;
}

fn build_half_saws(stage_creator: &StageCreator, commands: &mut Commands) -> bool {

    let atlas_rects = vec![
        get_object_tilemap_rect_from_index(ObjectAtlasIndices::HalfSaw0),
        get_object_tilemap_rect_from_index(ObjectAtlasIndices::HalfSaw1),
        get_object_tilemap_rect_from_index(ObjectAtlasIndices::HalfSaw2),
        get_object_tilemap_rect_from_index(ObjectAtlasIndices::HalfSaw3),
    ];

    for half_saw in &stage_creator.stage.half_saws {
        SawFactory::spawn_half(commands, stage_creator, atlas_rects.clone(), half_saw);
    }

    return true;
}

fn build_springs(stage_creator: &StageCreator, commands: &mut Commands) -> bool {
    let atlas_rects = vec![
        get_object_tilemap_rect_from_index(ObjectAtlasIndices::Spring0),
        get_object_tilemap_rect_from_index(ObjectAtlasIndices::Spring1),
        get_object_tilemap_rect_from_index(ObjectAtlasIndices::Spring2),
        get_object_tilemap_rect_from_index(ObjectAtlasIndices::Spring3),
        get_object_tilemap_rect_from_index(ObjectAtlasIndices::Spring4),
    ];

    for spring in &stage_creator.stage.springs {
        SpringFactory::spawn(commands, stage_creator, spring.grid_pos, atlas_rects.clone(), spring.rotation);
    }

    return true;
}

fn build_lock_blocks(stage_creator: &StageCreator, commands: &mut Commands) -> bool {
    let atlas_rect = get_object_tilemap_rect_from_index(ObjectAtlasIndices::LockBlock);
    for lock_block in &stage_creator.stage.lock_blocks {
        LockBlockFactory::spawn(commands, stage_creator, atlas_rect, lock_block);
    }

    return true;
}

fn build_keys(stage_creator: &StageCreator, commands: &mut Commands) -> bool {
    let atlas_rect = get_object_tilemap_rect_from_index(ObjectAtlasIndices::Key);
    for key in &stage_creator.stage.keys {
        KeyFactory::spawn(commands, stage_creator, atlas_rect, key);
    }

    return true;
}

fn build_interval_blocks(stage_creator: &StageCreator, commands: &mut Commands) -> bool {
    let atlas_rects = vec![
        get_object_tilemap_rect_from_index(ObjectAtlasIndices::IntervalBlock0),
        get_object_tilemap_rect_from_index(ObjectAtlasIndices::IntervalBlock1),
        get_object_tilemap_rect_from_index(ObjectAtlasIndices::IntervalBlock2)
    ];
    for interval_block in &stage_creator.stage.interval_blocks {
        IntervalBlockFactory::spawn(commands, stage_creator, atlas_rects.clone(), interval_block);
    }
    return true;
}

fn build_saw_shooters(stage_creator: &StageCreator, commands: &mut Commands) -> bool {
    let atlas_rects = vec![get_object_tilemap_rect_from_index(ObjectAtlasIndices::SawShooter)];
    for saw_shooter_block in &stage_creator.stage.saw_shooter_blocks {
        SawShooterFactory::spawn(commands, stage_creator, atlas_rects.clone(), saw_shooter_block);
    }
    return true;
}

fn build_phantom_blocks(stage_creator: &StageCreator, commands: &mut Commands) -> bool {
    let atlas_rects = vec![
        get_object_tilemap_rect_from_index(ObjectAtlasIndices::PhantomBlock0),
        get_object_tilemap_rect_from_index(ObjectAtlasIndices::PhantomBlock1),
        get_object_tilemap_rect_from_index(ObjectAtlasIndices::PhantomBlock2),
        get_object_tilemap_rect_from_index(ObjectAtlasIndices::PhantomBlock3),
        get_object_tilemap_rect_from_index(ObjectAtlasIndices::PhantomBlock4)
    ];
    for phantom_block in &stage_creator.stage.phantom_blocks {
        PhantomBlockFactory::spawn(commands, stage_creator, atlas_rects.clone(), phantom_block);
    }
    return true;
}

fn build_checkpoints(stage_creator: &StageCreator, commands: &mut Commands) -> bool {

    let sprite_rect = get_object_tilemap_rect_from_index(ObjectAtlasIndices::Player);

    for checkpoint in &stage_creator.stage.checkpoints {
        commands.spawn(CheckpointBundle::new(
            stage_creator, 
            checkpoint.grid_pos, 
            sprite_rect));
    }

    return true;
}

fn build_ground_tile(commands: &mut Commands, stage_creator: &StageCreator, grid_x: f32, grid_y: f32, atlas_index: usize) {

    let upper_left = Vec2::new((atlas_index as f32 % TILEMAP_SIZE as f32) as f32 * TILEMAP_TILE_SIZE, (atlas_index / TILEMAP_SIZE) as f32 * TILEMAP_TILE_SIZE);
    let lower_right = Vec2::new(upper_left.x + TILEMAP_TILE_SIZE , upper_left.y + TILEMAP_TILE_SIZE);
    let sprite_rect = Rect::new(upper_left.x, upper_left.y, lower_right.x, lower_right.y);

    commands.spawn(GroundTileBundle::new(
        stage_creator, 
        Vec2::new(grid_x, grid_y), 
        sprite_rect));


}

pub fn get_object_tilemap_rect_from_index(atlas_index: ObjectAtlasIndices) -> Rect {
    let index = atlas_index as usize;
    let upper_left = Vec2::new((index as f32 % OBJECT_TILEMAP_SIZE as f32) as f32 * OBJECT_TILE_TILEMAP_SIZE, (index / OBJECT_TILEMAP_SIZE) as f32 * OBJECT_TILE_TILEMAP_SIZE);
    let lower_right = Vec2::new(upper_left.x + OBJECT_TILE_TILEMAP_SIZE, upper_left.y + OBJECT_TILE_TILEMAP_SIZE);
    Rect::new(upper_left.x, upper_left.y, lower_right.x, lower_right.y)
}




