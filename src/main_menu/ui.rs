
use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::{common::states::{AppState, DespawnOnStateExit}, databases::{game_db::GameDb, save_db::{GameSave, SaveDb}}, game::story::StorySave, main_menu::LoadSave};

#[derive(Component)]
pub struct ContinueGamesaveButton(usize);

#[derive(Component)]
pub struct NewGameButton(usize);

pub fn check_new_game_interaction_TEMP_GAMEPAD_SUPPORT(
    gamepad_query: Query<&Gamepad>,
    mut start_game_writer: EventWriter<LoadSave>,
    save_db: Res<SaveDb>
) {
    for gamepad in gamepad_query {
        if gamepad.just_pressed(GamepadButton::South) {
            //let stage_ids = get_stage_ids();
            //let new_run = EndlessRun::new(stage_ids, 100);
            //start_game_writer.write(LoadSave::Endless(new_run));
            //save_db.delete_game_save();
            //return;
        }
    }
}

pub fn check_new_game_interaction(
    mut interaction_query: Query<(&NewGameButton, &Interaction), (Changed<Interaction>, With<Button>)>,
    mut start_game_writer: EventWriter<LoadSave>,
    game_db: Res<GameDb>
) {
    for (new_game_btn, interaction) in &mut interaction_query {

        match interaction {
            Interaction::Pressed => {
                let new_save = StorySave::new(new_game_btn.0);
                if game_db.save_story(&new_save) {
                    start_game_writer.write(LoadSave::Story(new_save));
                }
            }
            _ => ()
        }
    }
}

pub fn check_continue_game_interaction(
    mut interaction_query: Query<(&ContinueGamesaveButton, &Interaction), (Changed<Interaction>, With<Button>)>,
    mut start_game_writer: EventWriter<LoadSave>,
    game_db: Res<GameDb>
) {
    for (story_button, interaction) in &mut interaction_query {
        match interaction {
            Interaction::Pressed => {
                if let Some(existing_save) = game_db.load_gamesave(story_button.0) {
                    match existing_save {
                        GameSave::Story(story_save) => start_game_writer.write(LoadSave::Story(story_save)),
                    };
                };
                
            }
            _ => ()
        }
    }
}



pub fn build_main_menu_ui(
    mut commands: Commands,
    game_db: Res<GameDb>
) {
    commands.spawn(Node {
            justify_content: JustifyContent::Center, 
            align_items: AlignItems::Center,         
            flex_direction: FlexDirection::Column,   
            width: Val::Percent(100.0), 
            height: Val::Percent(100.0), 
            ..default()
        })
        .with_children(|parent| {
            for i in 0..3 {
                let mut btn = parent.spawn(());
                match game_db.load_gamesave(i) {
                    Some(gamesave) => {
                        match gamesave {
                            GameSave::Story(story_save) => build_existing_story_save_button(&mut btn, &story_save),
                        }
                    },
                    None => build_new_gamesave_button(&mut btn, i),
                }
            }
        });        
}



pub fn build_existing_story_save_button(entity_commands: &mut EntityCommands, story_save: &StorySave) {
    entity_commands.try_insert((
        Button,
        ContinueGamesaveButton(story_save.save_id),
        Node {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(Color::srgb_u8(200, 200, 200)),
        Text::new("Continue Game"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        DespawnOnStateExit::App(AppState::MainMenu)
    ));
}

pub fn build_new_gamesave_button(entity_commands: &mut EntityCommands, gamesave_id: usize) {
    entity_commands.try_insert((
        Node {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(10.0)),
            
            ..default()
        },
        BackgroundColor(Color::srgb_u8(200, 200, 200)),
        Button,
        Text::new("New Game"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        DespawnOnStateExit::App(AppState::MainMenu),
        NewGameButton(gamesave_id)
        )).insert(DespawnOnStateExit::App(AppState::MainMenu));
}