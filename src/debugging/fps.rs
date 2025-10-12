use bevy::prelude::*;

#[derive(Component)]
pub struct FPSUI;

pub fn setup_fps_stuff(mut commands: Commands) {
    commands.insert_resource(FrameRate::new(1000));
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            ..default()
        },
        Text::default(),
        FPSUI
    ));

}

pub fn update_fps_ui(
    mut query: Query<&mut Text, With<FPSUI>>,
    time: Res<Time>,
    mut frame_rate: ResMut<FrameRate>
) {
    let mut text = query.single_mut().unwrap();

    frame_rate.update(time.delta_secs_f64());
    let fps_text = format!(
        "{:<10} {:>6.2}\n{:<10} {:>6.2}\n{:<10} {:>6.2}",
        "FPS:", frame_rate.average_fps(),
        "1% Low:", frame_rate.one_percent_low_fps(),
        "0.1% Low:", frame_rate.point_one_percent_low_fps()
    );

    text.0 = fps_text;
}

#[derive(Resource, Default)]
pub struct FrameRate {
    frame_times: std::collections::VecDeque<f64>,
    max_frames: usize,
}

impl FrameRate {
    pub fn new(max_frames: usize) -> Self {
        FrameRate {
            frame_times: std::collections::VecDeque::with_capacity(max_frames),
            max_frames,
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        if self.frame_times.len() == self.max_frames {
            self.frame_times.pop_front();
        }
        self.frame_times.push_back(delta_time);
        
    }

    pub fn average_fps(&self) -> f64 {
        let sum: f64 = self.frame_times.iter().sum();
        (self.frame_times.len() as f64) / sum
    }

    pub fn one_percent_low_fps(&self) -> f64 {
        self.percentile_low_fps(0.01)
    }

    pub fn point_one_percent_low_fps(&self) -> f64 {
        self.percentile_low_fps(0.001)
    }

    fn percentile_low_fps(&self, percentile: f64) -> f64 {
        let mut times: Vec<f64> = self
            .frame_times
            .iter()
            .copied()
            .filter(|x| x.is_finite() && *x > 0.0)
            .collect();

        if times.is_empty() {
            return 0.0;
        }

        times.sort_by(|a, b| b.partial_cmp(a).unwrap());
        let index = ((times.len() as f64) * percentile).ceil() as usize;
        let index = index.min(times.len() - 1);

        return 1.0 / times[index];
    }
}


