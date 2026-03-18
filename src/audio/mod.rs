
use bevy::{platform::collections::HashMap, prelude::*};
use bevy_seedling::{SeedlingPlugin, prelude::SpatialBasicNode, sample::{AudioSample, SamplePlayer}, sample_effects};

pub struct AudioPlugin;


impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SeedlingPlugin::default())
            .add_event::<PlaySfxEvent>()
            .add_systems(Startup, init_sfx_db)
            .add_systems(Last, handle_sfx_events);
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Sfx {
    PlayerJump,
    PlayerSurfaceHit,
    Bounce,
}

#[derive(Resource)]
struct SfxDb {
    pub entries: HashMap<Sfx, SfxEntry>
}

struct SfxEntry {
    pub handle: Handle<AudioSample>,
    pub last_played: f32,
    pub cooldown: f32
}

#[derive(Event)]
pub struct PlaySfxEvent {
    pub sfx: Sfx,
    pub translation: Vec3
}


fn init_sfx_db(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let mut entries = HashMap::<Sfx, SfxEntry>::new();
    entries.insert(Sfx::PlayerJump, SfxEntry { handle: asset_server.load("audio/sfx/player_jump.wav"), last_played: 0.0, cooldown: 0.1 });
    entries.insert(Sfx::Bounce, SfxEntry { handle: asset_server.load("audio/sfx/bounce.wav"), last_played: 0.0, cooldown: 0.1 });
    entries.insert(Sfx::PlayerSurfaceHit, SfxEntry { handle: asset_server.load("audio/sfx/surface_impact.wav"), last_played: 0.0, cooldown: 0.1 });
    commands.insert_resource(SfxDb {
        entries,
    });
}

fn handle_sfx_events(
    mut events: EventReader<PlaySfxEvent>,
    mut sfxdb: ResMut<SfxDb>,
    mut commands: Commands,
    time: Res<Time>
) {
    for sfx_event in events.read() {
        let Some(entry) = sfxdb.entries.get_mut(&sfx_event.sfx) else { continue; };
        if time.elapsed_secs() - entry.last_played >= entry.cooldown {
            entry.last_played = time.elapsed_secs();
            commands.spawn((
                SamplePlayer::new(entry.handle.clone()),
                sample_effects![SpatialBasicNode::default()],
                Transform::from_translation(sfx_event.translation)
            ));
        }

    }
}