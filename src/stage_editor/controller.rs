
use bevy::{prelude::*, scene::ron, utils::hashbrown::HashMap};

use crate::stage::stage_builder::{stage_asset::{GroundTile, HalfSaw, IntervalBlock, Key, LockBlock, PhantomBlock, SawShooterBlock, Spike, Spring, Stage, TerrainTheme}, stage_creator::TILE_SIZE};

use super::{enums::*, get_ground_atlas_index};

pub const EDITOR_TILEMAP_SIZE: f32 = 16.0;
pub const GROUND_TILEMAP_SIZE: f32 = 7.0;

#[derive(Resource)]
pub struct EditorController {
    pub current_item: EditorItem,
    tile_size: f32,
    /// Tracks whether or not the latest stage updates have been saved
    saved: bool,
    pub object_atlas: Handle<Image>,
    pub ground_atlas: Handle<Image>,
    pub stage_grid: HashMap<IVec2, EditorItem>,
    pub grid_size: IVec2,
    grid_snap_unit: f32,
    pub version: usize,
    new_stage_id: usize
}

///API
impl EditorController {
    pub fn new(new_stage_id: usize, object_atlas: &Handle<Image>, ground_atlas: &Handle<Image>) -> Self {
        
        Self { 
            current_item: EditorItem::default(),
            tile_size: TILE_SIZE,
            saved: false,
            object_atlas: object_atlas.clone(),
            ground_atlas: ground_atlas.clone(),
            grid_size: IVec2::new(20, 50),
            stage_grid: HashMap::new(),
            version: 0,
            new_stage_id,
            grid_snap_unit: 16.0
         }
    }

    pub fn from_stage(stage: &Stage, new_stage_id: usize, object_atlas: &Handle<Image>, ground_atlas: &Handle<Image>) -> Self {
        let mut editor = Self { 
            current_item: EditorItem::default(),
            tile_size: TILE_SIZE,
            saved: false,
            object_atlas: object_atlas.clone(),
            ground_atlas: ground_atlas.clone(),
            grid_size: IVec2::new(stage.grid_width as i32, stage.grid_height as i32),
            stage_grid: HashMap::new(),
            version: 0,
            new_stage_id,
            grid_snap_unit: 16.0
         };
         editor.set_stage_template(stage);
         return editor;
    }

    pub fn cycle_next_item(&mut self) {
        if self.can_cycle_item() {
            self.current_item = self.current_item.cycle_next();
        }
    }
    pub fn cycle_prev_item(&mut self) {
        if self.can_cycle_item() {
            self.current_item = self.current_item.cycle_prev();
        }
    }
    pub fn cycle_next_item_variant(&mut self) {
        if self.can_cycle_item_variant() {
            self.current_item = self.current_item.cycle_next_variant();
        }
    }
    pub fn cycle_prev_item_variant(&mut self) {
        if self.can_cycle_item_variant() {
            self.current_item = self.current_item.cycle_prev_variant();
        }
    }
    
    pub fn should_display_item_icon(&self) -> bool {
        true
    }
    /// Returns the grid position in world space <br/>
    /// In other words, snaps the world pos to the grid and returns that snapped world pos
    pub fn world_to_grid_world_pos(&self, world_pos: Vec3) -> Vec3 {
        let grid_pos = self.world_to_grid_pos(world_pos);
        return self.grid_pos_to_world_grid_pos(grid_pos);
    }
    pub fn grid_pos_to_world_grid_pos(&self, grid_pos: IVec2) -> Vec3 {
        Vec3::new(
            (grid_pos.x as f32 + (grid_pos.x.signum() as f32 * 0.5)) * self.tile_size, 
            (grid_pos.y as f32 + (grid_pos.y.signum() as f32 * 0.5)) * self.tile_size, 
            0.0)
    }
    /// Returns the grid position
    pub fn world_to_grid_pos(&self, world_pos: Vec3) -> IVec2 {
        IVec2::new(
            (world_pos.x / self.tile_size).trunc() as i32, 
            (world_pos.y /self.tile_size).trunc()as i32) 
    }

    pub fn try_place(&mut self, grid_pos: IVec2) -> bool {
        if !self.can_place(grid_pos) { return false; }
        self.stage_grid.insert(grid_pos, self.current_item);
        self.saved = false;
        self.version += 1;
        return true;
    }
    
    pub fn can_place(&self, grid_pos: IVec2) -> bool {
        grid_pos.x >= 0 && 
        grid_pos.x <= self.grid_size.x as i32 - 1 &&
        grid_pos.y >= 0 && 
        grid_pos.y <= self.grid_size.y as i32 - 1 &&
        !self.stage_grid.contains_key(&grid_pos)
    }

    pub fn try_remove(&mut self, grid_pos: IVec2) -> bool{
        if !self.can_remove(grid_pos) { return false; }

        if let Some(_) = self.stage_grid.remove_entry(&grid_pos) {
            self.version += 1;
            //TODO: raise delete event!
        }
        return true;
    }
    
    pub fn try_save(&mut self) -> bool {

        if !self.can_save() { return false; }

        let stage = self.build_stage();

        let mut bytes: Vec<u8> = vec![];
        ron::ser::to_writer(&mut bytes, &stage).unwrap();
        let name = String::from("assets/stage_".to_owned() + &stage.id.to_string() + ".stage");
        let path = std::path::Path::new(&name);     
        let mut file = std::fs::File::create(&path).expect("yeet1");       
        
        use std::io::Write;
        file.write_all(&bytes).expect("yeet2");
        self.saved = true;
        return true;
    }
    pub fn try_rotate(&mut self) -> bool {
        return self.current_item.try_rotate();
    }
    pub fn can_remove(&self, grid_pos: IVec2) -> bool {
        grid_pos.x >= 0 && 
        grid_pos.x <= self.grid_size.x as i32 - 1 &&
        grid_pos.y >= 0 && 
        grid_pos.y <= self.grid_size.y as i32 - 1 &&
        self.stage_grid.contains_key(&grid_pos)
    }

}

/// Helper Functions
impl EditorController {
    fn can_cycle_item(&self) -> bool {
        true
    }
    fn can_cycle_item_variant(&self) -> bool {
        true
    }
    fn can_save(&self) -> bool {
        true
    }

    fn set_stage_template(&mut self, stage: &Stage) {

        self.stage_grid.insert(stage.spawn_grid_pos.as_ivec2(), EditorItem::Spawn);
        self.version += 1;
        self.stage_grid.insert(stage.goal_grid_pos.as_ivec2(), EditorItem::Goal);
        
        for ground in &stage.ground_tiles {
            self.stage_grid.insert(ground.grid_pos.as_ivec2(), EditorItem::Ground);
        }
        for spike in &stage.spikes {
            self.stage_grid.insert(spike.grid_pos.as_ivec2(), EditorItem::Spike { rotation: spike.rotation });
        }
        for half_saw in &stage.half_saws {
            self.stage_grid.insert(half_saw.grid_pos.as_ivec2(), EditorItem::HalfSaw { rotation: half_saw.rotation });
        }
        for spring in &stage.springs {
            self.stage_grid.insert(spring.grid_pos.as_ivec2(), EditorItem::Spring { rotation: spring.rotation });
        }
        for lock_block in &stage.lock_blocks {
            self.stage_grid.insert(lock_block.grid_pos.as_ivec2(), EditorItem::LockBlock { variant: match lock_block.trigger_id {
                1 => LockBlockVariant::One,
                2 => LockBlockVariant::Two,
                _ => LockBlockVariant::Three,
            }});
        }
        for key in &stage.keys {
            self.stage_grid.insert(key.grid_pos.as_ivec2(), EditorItem::Key { variant: match key.trigger_id {
                1 => KeyVariant::One,
                2 => KeyVariant::Two,
                _ => KeyVariant::Three,
            }});
        }
        for interval_block in &stage.interval_blocks {
            self.stage_grid.insert(interval_block.grid_pos.as_ivec2(), EditorItem::IntervalBlock { variant: match interval_block.is_active {
                true => IntervalBlockVariant::On,
                false => IntervalBlockVariant::Off
            }});
        }
        for saw_shooter in &stage.saw_shooter_blocks {
            self.stage_grid.insert(saw_shooter.grid_pos.as_ivec2(), EditorItem::SawShooter { rotation: saw_shooter.rotation });
        }
        for phantom_block in &stage.phantom_blocks {
            self.stage_grid.insert(phantom_block.grid_pos.as_ivec2(), EditorItem::PhantomBlock);
        }
        
    }

    fn build_stage(&self) -> Stage {
        let mut stage: Stage = Stage::new(self.new_stage_id, self.grid_size);
        for (grid_pos, stage_editor_obj) in &self.stage_grid {
            match stage_editor_obj {
                EditorItem::Spike { rotation } => {
                                            stage.spikes.push(Spike {
                                                grid_pos: grid_pos.as_vec2(),
                                                rotation: *rotation,
                                            });
                                        },
                EditorItem::Ground => {
                                            stage.ground_tiles.push(GroundTile {
                                                grid_pos: grid_pos.as_vec2(),
                                                tilemap_index: get_ground_atlas_index(self, *grid_pos, None),
                                            });
                                        },
                EditorItem::Spawn => stage.spawn_grid_pos = grid_pos.as_vec2(),
                EditorItem::Spring { rotation } => {
                                            stage.springs.push(Spring {
                                                grid_pos: grid_pos.as_vec2(),
                                                rotation: *rotation,
                                            });
                                        }
                EditorItem::PhantomBlock => {
                                            stage.phantom_blocks.push(PhantomBlock {
                                                grid_pos: grid_pos.as_vec2(),
                                            });
                                        },
                EditorItem::HalfSaw { rotation } => {
                                            stage.half_saws.push(HalfSaw {
                                                grid_pos: grid_pos.as_vec2(),
                                                rotation: *rotation,
                                                movement_path_opt: None
                                            });
                                        },
                EditorItem::Key { variant } => {
                                            stage.keys.push(Key {
                                                grid_pos: grid_pos.as_vec2(),
                                                trigger_id: match variant {
                                                    KeyVariant::One => 1,
                                                    KeyVariant::Two => 2,
                                                    KeyVariant::Three => 3,
                                                },
                                            });
                                        },
                EditorItem::LockBlock { variant } => {
                                            stage.lock_blocks.push(LockBlock {
                                                grid_pos: grid_pos.as_vec2(),
                                                trigger_id: match variant {
                                                    LockBlockVariant::One => 1,
                                                    LockBlockVariant::Two => 2,
                                                    LockBlockVariant::Three => 3,
                                                },
                                            });
                                        },
                EditorItem::IntervalBlock { variant } => {
                                            stage.interval_blocks.push(IntervalBlock {
                                                grid_pos: grid_pos.as_vec2(),
                                                is_active: match variant {
                                                    IntervalBlockVariant::On => true,
                                                    IntervalBlockVariant::Off => false,
                                                }
                                            });
                                        },
                EditorItem::SawShooter { rotation } => {
                                            stage.saw_shooter_blocks.push(SawShooterBlock {
                                                grid_pos: grid_pos.as_vec2(),
                                                rotation: *rotation,
                                            });
                                        },
                EditorItem::Goal => stage.goal_grid_pos = grid_pos.as_vec2(),
                EditorItem::TerrainTheme { variant } => stage.terrain_theme = match variant {
                    TerrainThemeVarient::Grass => TerrainTheme::Grass,
                    TerrainThemeVarient::Snow => TerrainTheme::Snow,
                    TerrainThemeVarient::Sand => TerrainTheme::Sand,
                },
            }
        }
        return stage;
    }
}
