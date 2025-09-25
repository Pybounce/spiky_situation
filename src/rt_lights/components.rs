
use bevy::prelude::*;


//pub enum LightOccluder {
//    Box(f32, f32)
//}
//
//pub struct PointLight {
//    pub intensity: f32,
//    pub colour: Color
//}

#[derive(Component, Default, Clone, Copy)]
pub struct PointLight {
    pub intensity: u8,
    pub colour: Color
}