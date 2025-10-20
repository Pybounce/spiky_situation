
use bevy::{color::palettes::css::NAVY, prelude::*};

use crate::rt_lights::components::AreaLight;


pub fn debug_lights(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &AreaLight)>
) {
    for (transform, light) in &query {
        for (pos, intensity) in light.lights_from_area(transform.translation) {
            let size = 8.0 * (intensity / light.intensity);
            gizmos.circle_2d(Isometry2d::from_translation(pos.truncate()), size, NAVY).resolution(64);
        }
    }

}