use bevy::prelude::*;

use crate::stage::{stage_builder::events::BuildStageEvent, stage_objects::StageObject};

use super::CurrentStageData;


pub fn unload_old_stage(
    stage_object_query: Query<(Entity, &StageObject)>,
    mut commands: Commands,
    current_stage_opt: Option<Res<CurrentStageData>>,
    mut event_reader: EventReader<BuildStageEvent>,
) {
    let Some(current_stage) = current_stage_opt else { 
        let _ = event_reader.read().count();
        return; 
    };

    for build_stage_event in event_reader.read() {
        println!("teardown stage");
        for (e, so) in &stage_object_query {
            let should_remove = StageObject::StagePersistent != *so || build_stage_event.stage_id != current_stage.stage_id;
            if should_remove { commands.entity(e).despawn(); }
        }
        commands.remove_resource::<CurrentStageData>();
    }

}

pub fn cleanup_old_stage(
    stage_object_query: Query<Entity, With<StageObject>>,
    mut commands: Commands,
) {

    for e in &stage_object_query {
        commands.entity(e).despawn(); 
    }
    commands.remove_resource::<CurrentStageData>();

}