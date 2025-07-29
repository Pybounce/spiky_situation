
use std::default;

use bevy::{prelude::*, utils::hashbrown::HashMap};

use super::super::{controller::{EditorController, EDITOR_TILEMAP_SIZE}, enums::{EditorItem, IntervalBlockVariant, KeyVariant, LockBlockVariant}};

#[derive(Resource)]
pub struct EditorRenderer {
    pub stage_grid: HashMap<IVec2, Entity>,
    pub version: usize,
    pub full_refresh: bool
}

#[derive(Component)]
pub struct RenderedEditorItem;

impl EditorRenderer {
    pub fn new() -> Self {
        Self {
            stage_grid: HashMap::new(),
            version: 0,
            full_refresh: true
        }
    }
}

//Helper Methods
impl EditorRenderer {
    pub fn get_item_icon_atlas_rect(editor_item: &EditorItem) -> Rect {
        let (index, tile_size) = match editor_item {
            EditorItem::Ground => (15.0, 16.0),
            EditorItem::Spike { .. } => (4.0, 16.0),
            EditorItem::Spawn => (18.0, 16.0),
            EditorItem::Spring { .. } => (5.0, 16.0),
            EditorItem::PhantomBlock => (21.0, 16.0),
            EditorItem::HalfSaw { .. } => (0.0, 16.0),
            EditorItem::Key { variant } => {
                                        match variant {
                                            KeyVariant::One => (255.0, 16.0),
                                            KeyVariant::Two => (239.0, 16.0),
                                            KeyVariant::Three => (223.0, 16.0),
                                        }
                                    },
            EditorItem::LockBlock { variant } => {
                                        match variant {
                                            LockBlockVariant::One => (254.0, 16.0),
                                            LockBlockVariant::Two => (238.0, 16.0),
                                            LockBlockVariant::Three => (222.0, 16.0),
                                        }
                                    },
            EditorItem::IntervalBlock { variant }=> {
                                        match variant {
                                            IntervalBlockVariant::On => (253.0, 16.0),
                                            IntervalBlockVariant::Off => (237.0, 16.0),
                                        }
                                    },
            EditorItem::SawShooter { .. } => (27.0, 16.0),
            EditorItem::Goal => (48.0, 16.0),
            EditorItem::TerrainTheme { variant } => match variant {
                crate::stage_editor::enums::TerrainThemeVarient::Grass => (220.0, 16.0),
                crate::stage_editor::enums::TerrainThemeVarient::Snow => (236.0, 16.0),
                crate::stage_editor::enums::TerrainThemeVarient::Sand => (252.0, 16.0),
            },
        };

        let upper_left = Vec2::new(index % EDITOR_TILEMAP_SIZE, (index / EDITOR_TILEMAP_SIZE).trunc()) * tile_size;
        let lower_right = upper_left + tile_size;
        Rect::new(upper_left.x, upper_left.y, lower_right.x, lower_right.y)
    }
}

