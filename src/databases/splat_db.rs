
use bevy::{image::{ImageLoaderSettings, ImageSampler}, prelude::*, render::render_resource::{Extent3d, TextureFormat}};


#[derive(Resource)]
pub struct SplatDb {
    atlas: Handle<Image>,
    splat_entries: Vec<SplatEntry>
}

impl SplatDb {
    pub fn random_radial(&self) -> (Handle<Image>, Rect) {
        return (self.atlas.clone(), self.splat_entries[0].rect);
    }
}

struct SplatEntry {
    pub rect: Rect,
    pub splat_type: SplatType,
    pub splat_direction: SplatDirection
}

impl SplatEntry {
    pub fn new(rect: Rect, splat_type: SplatType, direction: SplatDirection) -> Self {
        return Self {
            rect,
            splat_type,
            splat_direction: direction
        };
    }
}

enum SplatType {
    Radial,
    Long,
    Wide
}

enum SplatDirection {
    Up,
    Down,
    DiagonalUp,
    DiagonalDown,
    UpAndDiagonal,
    DownAndDiagonal
}


pub fn init_splat_db(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let splats: Handle<Image> = asset_server.load_with_settings("splats.png", |settings: &mut ImageLoaderSettings| {
        settings.is_srgb = false;
    });

    let entries = vec![
        SplatEntry::new(Rect::new(0.0, 0.0, 64.0  / 1024.0, 64.0 / 1024.0), SplatType::Radial, SplatDirection::UpAndDiagonal)
    ];

    commands.insert_resource(SplatDb {
        atlas: splats,
        splat_entries: entries
    });
}