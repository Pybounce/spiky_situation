
use bevy::prelude::*;

use crate::{common::states::{AppState, DespawnOnStateExit}, databases::game_db::{GameDb, Level}, game::story::StorySave, story_overworld::PlayLevelButton};


pub fn build_story_overworld(
    story_save_opt: Option<Res<StorySave>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    game_db: Res<GameDb>
) {
    match story_save_opt {
        Some(story_save) => {
            commands.spawn(Node {
                justify_content: JustifyContent::Center, 
                align_items: AlignItems::Center,         
                flex_direction: FlexDirection::Column,   
                width: Val::Percent(100.0), 
                height: Val::Percent(100.0), 
                ..default()
            })
            .with_children(|parent| {
                for level_id in 0..5 {
                    let mut btn = parent.spawn(());
                    match game_db.load_level(level_id) {
                        Some(level) => build_level_button(&mut btn, level_id, &level, story_save.completed_levels.contains(&level_id)),
                        None => build_no_level_btn(&mut btn),
                    }
                }
            }); 
        },
        None => {
            app_state.set(AppState::MainMenu);
        },

    }
}



fn build_level_button(entity_commands: &mut EntityCommands, level_id: usize, level: &Level, completed: bool) {
    let c = match completed {
        true => Color::srgb_u8(0, 255, 0),
        false => Color::srgb_u8(255, 255, 255),
    };
    entity_commands.try_insert((
        Button,
        PlayLevelButton(level_id),
        Node {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(Color::srgb_u8(200, 200, 200)),
        Text::new(format!("{}", level.name)),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(c),
        DespawnOnStateExit::App(AppState::StoryOverworld)
    ));
}

fn build_no_level_btn(entity_commands: &mut EntityCommands) {
    entity_commands.try_insert((
        Button,
        Node {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(Color::srgb_u8(200, 200, 200)),
        Text::new("NA"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgb_u8(255, 0, 0)),
        DespawnOnStateExit::App(AppState::StoryOverworld)
    ));
}
