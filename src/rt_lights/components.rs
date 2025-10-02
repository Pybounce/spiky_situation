
use bevy::prelude::*;


#[derive(Component, Default, Clone, Copy)]
pub struct PointLight {
    pub intensity: u8,
    pub colour: Color
}

#[derive(Component, Clone, Copy)]
pub enum LightOccluder {
    Square(f32),
    Circle(f32)
}

