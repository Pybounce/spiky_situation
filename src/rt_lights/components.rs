
use bevy::prelude::*;


#[derive(Component, Default, Clone, Copy)]
pub struct PointLight {
    pub intensity: f32,
    pub colour: Color
}

#[derive(Component, Default, Clone, Copy)]
pub struct AreaLight {
    pub intensity: f32,
    pub colour: Color,
    pub rect: Rect
}

#[derive(Component, Clone, Copy)]
pub enum LightOccluder {
    Rect(f32, f32),
    Circle(f32)
}


#[derive(Component)]
pub struct StaticLightOccluder;





impl AreaLight {
    pub fn lights_from_area(&self, pos: Vec3) -> Vec<(Vec3, f32)> {
        let mut lights = Vec::new();
        let grid_size = 16.0;

        let Rect { min, max } = self.rect;
        let width = max.x - min.x;
        let height = max.y - min.y;

        let x_count = ((width + grid_size - 1.0) / grid_size).floor() as usize;
        let y_count = ((height + grid_size - 1.0) / grid_size).floor() as usize;

        for xi in 0..x_count {
            for yi in 0..y_count {
                let cell_min_x = min.x + xi as f32 * grid_size;
                let cell_max_x = (cell_min_x + grid_size).min(max.x);
                let cell_min_y = min.y + yi as f32 * grid_size;
                let cell_max_y = (cell_min_y + grid_size).min(max.y);

                let x = (cell_min_x + cell_max_x) / 2.0;
                let y = (cell_min_y + cell_max_y) / 2.0;

                let cell_coverage_x = (cell_max_x - cell_min_x) / grid_size;
                let cell_coverage_y = (cell_max_y - cell_min_y) / grid_size;
                let coverage = cell_coverage_x * cell_coverage_y;

                let intensity = self.intensity * coverage;

                if intensity > 0.0 {
                    lights.push((pos + (Vec2::new(x, y) - self.rect.half_size()).extend(0.0), intensity));
                }
            }
        }

        lights
    }
}