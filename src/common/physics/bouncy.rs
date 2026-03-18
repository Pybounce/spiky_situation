
use bevy::prelude::*;
use avian2d::prelude::*;

use crate::{audio::{PlaySfxEvent, Sfx}, common::physics::avian_ex::ManyCollidingEntities};

#[derive(Component)]
pub struct Bouncy {
    pub force: Vec2
}

pub fn check_bouncy_collisions(
    mut bounceable_query: Query<(&mut LinearVelocity, &ManyCollidingEntities)>,
    bouncy_query: Query<(&Bouncy, &Transform)>,
    mut sfx_writer: EventWriter<PlaySfxEvent>
) {
    for (mut v, colliding_entities) in &mut bounceable_query {

        for colliding_entity in colliding_entities.iter() {
            if let Ok((b, b_t)) = bouncy_query.get(*colliding_entity) {
                if b.force == Vec2::ZERO { continue; }

                let parallel_v = v.0.project_onto(b.force);
                let perpendicular_v = v.0 - parallel_v;
                v.0 = perpendicular_v + b.force;

                sfx_writer.write(PlaySfxEvent {
                    sfx: Sfx::Bounce,
                    translation: b_t.translation,
                });
            }
        }
    }
}
