use avian2d::prelude::PhysicsLayer;


#[derive(PhysicsLayer, Default)]
pub enum GamePhysicsLayer {
    #[default]
    Default, 
    Player,  
    Ground,
    StageObject
}