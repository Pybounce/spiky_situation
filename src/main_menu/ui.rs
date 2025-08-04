
use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::{common::states::{AppState, DespawnOnStateExit}, databases::save_db::{GameSave, SaveDb}, game::endless::components::EndlessRun, main_menu::{get_stage_ids, StartGame}};

#[derive(Component)]
pub struct ContinueGameButton;

#[derive(Component)]
pub struct NewGameButton;


pub fn check_new_game_interaction(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<NewGameButton>)>,
    mut start_game_writer: EventWriter<StartGame>,
    save_db: Res<SaveDb>
) {
    for interaction in &mut interaction_query {

        match interaction {
            Interaction::Pressed => {
                let stage_ids = get_stage_ids();
                let new_run = EndlessRun::new(stage_ids, 10);
                start_game_writer.send(StartGame::Endless(new_run));
                save_db.delete_game_save();
            }
            _ => ()
        }
    }
}

pub fn check_continue_game_interaction(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<ContinueGameButton>)>,
    mut start_game_writer: EventWriter<StartGame>,
    save_db: Res<SaveDb>
) {
    for interaction in &mut interaction_query {

        match interaction {
            Interaction::Pressed => {
                if let Some(existing_run) = save_db.get_existing_save() {
                    match existing_run {
                        GameSave::Endless(endless_run) => start_game_writer.send(StartGame::Endless(endless_run)),
                    };
                };
                
            }
            _ => ()
        }
    }
}



pub fn build_main_menu_ui(
    mut commands: Commands,
    save_db: Res<SaveDb>
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
            if let Some(existing_save) = save_db.get_existing_save() {
                let mut btn = parent.spawn(());
                match existing_save {
                    GameSave::Endless(endless_run) => build_existing_endless_save_button(&mut btn, &endless_run),
                }
            }


            parent.spawn((
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
                NewGameButton
            ));
        }).insert(DespawnOnStateExit::App(AppState::MainMenu));
}



pub fn build_existing_endless_save_button(entity_commands: &mut EntityCommands, _endless_run: &EndlessRun) {
    entity_commands.try_insert((
        Button,
        ContinueGameButton,
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