
use bevy_rapier2d::prelude::*;
use bevy::prelude::*;

pub trait RapierRaycastExt {
    fn raycast_group(
        &self,
        origin: Vec2,
        dir: Vec2,
        max_distance: f32,
        group: Group,
    ) -> Option<(Entity, f32)>;
}

impl RapierRaycastExt for RapierContext<'_> {
    fn raycast_group(
        &self,
        origin: Vec2,
        dir: Vec2,
        max_distance: f32,
        group: Group,
    ) -> Option<(Entity, f32)> {
        let filter = QueryFilter::default()
            .groups(CollisionGroups::new(group, group));

        self.cast_ray(origin, dir.normalize(), max_distance, true, filter)
    }
}