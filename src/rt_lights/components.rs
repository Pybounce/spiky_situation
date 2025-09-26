
use std::default;

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

#[derive(Component, Clone, Copy)]
pub enum LightOccluder {
    Square(f32)
}

/*
#[derive(Component, Default, Clone, Copy)]
pub enum LightEmitter{
    #[default]
    Square((f32, Color))
}
*/



// The Occluder Plan!
// 
// Have LightOccluder components on entities
// Every frame load up a buffer of occluders (max maybe 100x100?)
// Have a shader that draws shapes to a grid 
//      This can be used for occlusion
//      This can be used for emission
// Then occlusion can continue as usual with less data being sent over.
// Only issue is when the occluders are very large.


// The Emission Plan!
// 
// Have LightEmitter components on entities
// Every frame load up a buffer with the emitter data
// Have the same shape shader draw to the light map
// Do many passes of cellular autonomer to the light map


// Currently the light map doesn't allow for atomic changes which is a big issue but more on that later...