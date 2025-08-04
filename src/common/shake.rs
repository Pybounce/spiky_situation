use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Shake {
    pub current_offset: Vec2,
    pub force: f32,
    pub duration: Option<f64>,
    pub shake_delay: f64,
    pub current_delay: f64
}

pub fn shake(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Shake, &mut Transform)>,
    time: Res<Time>
) {
    for (e, mut s, mut t) in &mut query {


        if let Some(d) = s.duration {
            s.duration = Some(d - time.delta_secs_f64());
            if s.duration.unwrap() <= 0.0 {
                t.translation -= s.current_offset.extend(0.0);
                commands.entity(e).remove::<Shake>();
                continue;
            }
        }

        s.current_delay -= time.delta_secs_f64();
        
        if s.current_delay <= 0.0 {
            s.current_delay = s.shake_delay;

            t.translation -= s.current_offset.extend(0.0);

            let x = rand::thread_rng().gen_range(0.0..s.force);
            let y = rand::thread_rng().gen_range(0.0..s.force);
    
            s.current_offset = Vec2::new(x, y);
            t.translation += s.current_offset.extend(0.0);
        }


    }
}