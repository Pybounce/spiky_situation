
use bevy::prelude::*;

use crate::{common::states::{AppState, DespawnOnStateExit}, game::endless::components::EndlessRun};



#[derive(Component)]
pub struct EndlessLivesRemainingText;

#[derive(Component)]
pub struct EndlessStagesCompleteText;




pub fn add_endless_mode_ui(
    mut commands: Commands
) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(15.0),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Node {
                    ..default()
                },
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Text::new(""),
                EndlessStagesCompleteText,
                DespawnOnStateExit::App(AppState::Game)
            ));

            parent.spawn((
                Node {
                    ..default()
                },
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Text::new(""),
                EndlessLivesRemainingText,
                DespawnOnStateExit::App(AppState::Game)
            ));
        });

}


pub fn update_endless_lives_remaining_text(
    mut query: Query<&mut Text, With<EndlessLivesRemainingText>>,
    current_run_opt: Option<Res<EndlessRun>>
) {
    if let Some(current_run) = current_run_opt {
        for mut text in &mut query {
            text.0 = format!("Lives: {}", current_run.lives_remaining());
        }
    }
}

pub fn update_endless_stages_complete(
    mut query: Query<&mut Text, With<EndlessStagesCompleteText>>,
    current_run_opt: Option<Res<EndlessRun>>
) {
    if let Some(current_run) = current_run_opt {
        for mut text in &mut query {
            text.0 = format!("Score: {}", current_run.stages_complete());
        }
    }
}