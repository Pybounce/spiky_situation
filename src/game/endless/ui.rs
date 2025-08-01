
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
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                left: Val::Px(20.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(15.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                "complete: 3",
                TextStyle {
                    font_size: 28.0,
                    color: Color::WHITE,
                    ..default()
                }),
                EndlessStagesCompleteText,
                DespawnOnStateExit::App(AppState::Game)
            ));

            parent.spawn((
                TextBundle::from_section(
                "Lives: 3",
                TextStyle {
                    font_size: 28.0,
                    color: Color::WHITE,
                    ..default()
                }),
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
            text.sections[0].value = format!("Lives: {}", current_run.lives_remaining());
        }
    }
}

pub fn update_endless_stages_complete(
    mut query: Query<&mut Text, With<EndlessStagesCompleteText>>,
    current_run_opt: Option<Res<EndlessRun>>
) {
    if let Some(current_run) = current_run_opt {
        for mut text in &mut query {
            text.sections[0].value = format!("Score: {}", current_run.stages_complete());
        }
    }
}