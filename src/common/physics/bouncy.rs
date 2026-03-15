
use bevy::prelude::*;
use avian2d::prelude::*;
use bevy_seedling::{prelude::SpatialBasicNode, sample::SamplePlayer, sample_effects};

use crate::common::physics::avian_ex::ManyCollidingEntities;

#[derive(Component)]
pub struct Bouncy {
    pub force: Vec2
}

pub fn check_bouncy_collisions(
    mut bounceable_query: Query<(&mut LinearVelocity, &ManyCollidingEntities)>,
    bouncy_query: Query<(&Bouncy, &Transform)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut last_played: Local<f32>,
    time: Res<Time>
) {
    for (mut v, colliding_entities) in &mut bounceable_query {

        for colliding_entity in colliding_entities.iter() {
            if let Ok((b, b_t)) = bouncy_query.get(*colliding_entity) {
                if b.force == Vec2::ZERO { continue; }

                let parallel_v = v.0.project_onto(b.force);
                let perpendicular_v = v.0 - parallel_v;
                v.0 = perpendicular_v + b.force;

                let cooldown = 0.05;
                if time.elapsed_secs() - *last_played >= cooldown {
                    *last_played = time.elapsed_secs();
                    commands.spawn((
                        SamplePlayer::new(asset_server.load("audio/sfx/bounce.wav")),
                        sample_effects![SpatialBasicNode::default()],
                        Transform::from_translation(b_t.translation)
                    ));
                }
            }
        }
    }
}
