
use bevy::{image::ImageLoaderSettings, prelude::*};
use rand::{seq::SliceRandom, thread_rng, Rng};


#[derive(Resource)]
pub struct SplatDb {
    atlas: Handle<Image>,
    splat_entries: Vec<SplatEntry>,
}

impl SplatDb {
    pub fn random_radial(&self) -> (Handle<Image>, Rect, Vec2) {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.splat_entries.len());
        return (self.atlas.clone(), self.splat_entries[index].rect, self.splat_entries[index].origin_offset);
    }
    pub fn random_of_type(&self, splat_type: SplatType) -> Option<(Handle<Image>, Rect, Vec2)> {
        let mut rng = thread_rng();
        let filtered: Vec<&SplatEntry> = self.splat_entries
            .iter()
            .filter(|entry| entry.splat_type == splat_type)
            .collect();
        return match filtered.choose(&mut rng) {
            Some(splat_entry) => Some((self.atlas.clone(), splat_entry.rect, splat_entry.origin_offset)),
            None => None,
        };
    }
}

struct SplatEntry {
    pub rect: Rect,
    pub splat_type: SplatType,
    pub origin_offset: Vec2
}

impl SplatEntry {
    pub fn new(rect: Rect, splat_type: SplatType, origin_offset: Vec2) -> Self {
        return Self {
            rect,
            splat_type,
            origin_offset,
        };
    }
}
#[derive(PartialEq)]
pub enum SplatType {
    Radial,
    Long,
    Diagonal
}


pub fn init_splat_db(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let splats: Handle<Image> = asset_server.load_with_settings("splats.png", |settings: &mut ImageLoaderSettings| {
        settings.is_srgb = false;
    });

    let entries = vec![
        SplatEntry::new(grid_to_rect(0, 0, 4, 4), SplatType::Radial, Vec2::ZERO),
        SplatEntry::new(grid_to_rect(12, 0, 3, 6), SplatType::Long, Vec2::new(-8.0, -24.0)),
        SplatEntry::new(grid_to_rect(15, 0, 2, 6), SplatType::Long, Vec2::new(0.0, -24.0)),
        SplatEntry::new(grid_to_rect(17, 0, 4, 5), SplatType::Diagonal, Vec2::new(-16.0, -24.0)),
        SplatEntry::new(grid_to_rect(21, 0, 2, 6), SplatType::Long, Vec2::new(0.0, -24.0)),
        SplatEntry::new(grid_to_rect(23, 0, 4, 6), SplatType::Diagonal, Vec2::new(-16.0, -32.0)),
        SplatEntry::new(grid_to_rect(27, 0, 4, 5), SplatType::Diagonal, Vec2::new(-12.0, -16.0)),
        SplatEntry::new(grid_to_rect(31, 0, 3, 6), SplatType::Long, Vec2::new(0.0, -30.0)),
    ];

    let test_entries = vec![
        SplatEntry::new(grid_to_rect(0, 4, 4, 4), SplatType::Radial, Vec2::ZERO),
        SplatEntry::new(grid_to_rect(4, 4, 3, 6), SplatType::Long, Vec2::new(0.0, -24.0)),
        SplatEntry::new(grid_to_rect(7, 4, 5, 5), SplatType::Diagonal, Vec2::new(-16.0, -16.0)),
    ];

    commands.insert_resource(SplatDb {
        atlas: splats,
        splat_entries: entries
    });
}


fn grid_to_rect(min_grid_x: u32, min_grid_y: u32, grid_width: u32, grid_height: u32) -> Rect {
    let min_x = min_grid_x as f32 * 16.0;
    let min_y = min_grid_y as f32 * 16.0;
    let max_x = min_x + (grid_width as f32 * 16.0);
    let max_y = min_y + (grid_height as f32 * 16.0);
    return Rect::new(min_x, min_y, max_x, max_y);
}