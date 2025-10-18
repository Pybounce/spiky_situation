
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