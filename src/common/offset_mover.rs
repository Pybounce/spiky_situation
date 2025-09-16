
use bevy::prelude::*;

#[derive(Component)]
pub struct OffsetMover {
    pub offsets: Vec<Vec3>,
    pub speed: f32,
    pub current_target_index: usize,
    pub current_offset: Vec3
}

impl OffsetMover {
    pub fn new_from_grid(grid_offsets: &Vec<Vec2>, speed: f32) -> Self {
        OffsetMover {
            offsets: grid_offsets.iter().map(|x| x.extend(0.0)).collect(),
            speed,
            current_target_index: 0,
            current_offset: Vec3::ZERO,
        }
    }
    pub fn target_offset(&self) -> Vec3 {
        self.offsets[self.current_target_index]
    }
    pub fn delta(&self) -> Vec3 {
        self.target_offset() - self.current_offset
    }
    pub fn target_next_offset(&mut self) {
        self.current_target_index += 1;
        if self.current_target_index >= self.offsets.len() {
            self.current_target_index = 0;
        }
    }
}

//TODO: Overtime movers may become out of sync
    // Right now if the actual distance to the target is smaller than the next move distance, the saw only moves the smaller distance
    // But really it should start moving the to the next point by the offset
    // Or we do something smart with a Duration and calc the current lerp using a modulo
    // But who the fuck has time for that
pub fn move_offset_movers(
    mut query: Query<(&mut Transform, &mut OffsetMover)>,
    time: Res<Time>
) {
    for (mut t, mut om) in &mut query {

        let target_delta = om.delta();
        let mut delta = target_delta.normalize_or_zero() * om.speed * time.delta_secs();
        if target_delta.length() < delta.length() {
            delta = target_delta;
        }

        t.translation += delta;
        om.current_offset += delta;

        if target_delta.length() <= 0.01 {
            om.target_next_offset();
        }
    }
}