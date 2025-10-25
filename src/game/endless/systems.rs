
use bevy::prelude::*;

use crate::{common::death::DeathMarker, databases::save_db::{SaveDb, SaveGame}, game::{endless::components::EndlessRun, game_over::GameOver}, local_player::LocalPlayer, stage::stage_builder::{events::{BuildStageEvent}, CurrentStageData}};


pub fn save_endless_game(
    mut save_game_event_reader: EventReader<SaveGame>,
    endless_game_opt: Option<Res<EndlessRun>>,
    save_db: Res<SaveDb>
) {
    if let Some(endless_run) = endless_game_opt { 
        let mut saved = false;
        for _ in save_game_event_reader.read() {
            if saved { continue; }

            save_db.save_endless(&endless_run);

            saved = true;
        }
    }

}


pub fn check_death_endless_mode(
    query: Query<(), (With<LocalPlayer>, Added<DeathMarker>)>,
    stage_data_opt: Option<Res<CurrentStageData>>,
    mut build_event_writer: EventWriter<BuildStageEvent>,
    mut current_run_opt: Option<ResMut<EndlessRun>>,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut save_event_writer: EventWriter<SaveGame>,
    save_db: Res<SaveDb>
) {
    if let Some(stage_data) = stage_data_opt {
        if let Ok(_) = &query.single()  {
            if let Some(current_run) = current_run_opt.as_mut() {
                if current_run.lives_remaining() == 0 {
                    game_over_event_writer.write(GameOver);
                    save_db.delete_game_save();
                }
                else {
                    save_event_writer.write(SaveGame);
                    current_run.remove_life();
                    build_event_writer.write(BuildStageEvent { stage_id: stage_data.stage_id, gateway_id_opt: stage_data.gateway_id_opt });
                }
            }
        }
    }
}
