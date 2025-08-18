
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
        SplatEntry::new(Rect::new(0.0, 0.0, 64.0, 64.0), SplatType::Radial, Vec2::ZERO),
        SplatEntry::new(Rect::new(192.0, 0.0, 240.0, 96.0), SplatType::Long, Vec2::new(-8.0, -24.0)),
        SplatEntry::new(Rect::new(240.0, 0.0, 272.0, 96.0), SplatType::Long, Vec2::new(0.0, -24.0)),
        SplatEntry::new(Rect::new(272.0, 0.0, 336.0, 80.0), SplatType::Diagonal, Vec2::new(-16.0, -24.0)),
    ];

    let test_entries = vec![
        SplatEntry::new(Rect::new(0.0, 64.0, 64.0, 128.0), SplatType::Radial, Vec2::ZERO),
        SplatEntry::new(Rect::new(64.0, 64.0, 112.0, 160.0), SplatType::Long, Vec2::new(0.0, -24.0)),
        SplatEntry::new(Rect::new(112.0, 64.0, 192.0, 144.0), SplatType::Diagonal, Vec2::new(-16.0, -16.0)),
    ];

    commands.insert_resource(SplatDb {
        atlas: splats,
        splat_entries: entries
    });
}