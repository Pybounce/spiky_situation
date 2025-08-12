
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
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_content: AlignContent::SpaceBetween,
            border: UiRect::all(Val::Percent(4.0)),
            ..default()
        })
        .with_children(|parent| {

            parent.spawn(
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(50.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                }
            ).with_children(|top_row_parent| {
                top_row_parent.spawn((
                    Node {
                        min_width: Val::Px(150.),
                        min_height: Val::Px(150.),
                        max_width: Val::Percent(5.),
                        max_height: Val::Percent(5.),
                        width: Val::Auto,
                        height: Val::Auto,
                        border: UiRect::left(Val::Px(5.0)).with_top(Val::Px(5.0)),
                        ..default()
                    },
                    BorderColor(Color::WHITE),

                ));
                top_row_parent.spawn((
                    Node {
                        min_width: Val::Px(150.),
                        min_height: Val::Px(150.),
                        max_width: Val::Percent(5.),
                        max_height: Val::Percent(5.),
                        width: Val::Auto,
                        height: Val::Auto,
                        border: UiRect::right(Val::Px(5.0)).with_top(Val::Px(5.0)),
                        ..default()
                    },
                    BorderColor(Color::WHITE),

                ));
            });
            parent.spawn(
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(50.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::FlexEnd,
                    ..default()
                }
            ).with_children(|bottom_row_parent| {
                bottom_row_parent.spawn((
                    Node {
                        min_width: Val::Px(150.),
                        min_height: Val::Px(150.),
                        max_width: Val::Percent(5.),
                        max_height: Val::Percent(5.),
                        width: Val::Auto,
                        height: Val::Auto,
                        border: UiRect::left(Val::Px(5.0)).with_bottom(Val::Px(5.0)),
                        ..default()
                    },
                    BorderColor(Color::WHITE),
                ));
                bottom_row_parent.spawn((
                    Node {
                        min_width: Val::Px(150.),
                        min_height: Val::Px(150.),
                        max_width: Val::Percent(5.),
                        max_height: Val::Percent(5.),
                        width: Val::Auto,
                        height: Val::Auto,
                        border: UiRect::right(Val::Px(5.0)).with_bottom(Val::Px(5.0)),
                        ..default()
                    },
                    BorderColor(Color::WHITE),

                ));
            });

            //parent.spawn((
            //    Node {
            //        ..default()
            //    },
            //    TextFont {
            //        font_size: 28.0,
            //        ..default()
            //    },
            //    TextColor(Color::WHITE),
            //    Text::new(""),
            //    EndlessStagesCompleteText,
            //    DespawnOnStateExit::App(AppState::Game)
            //));
//
            //parent.spawn((
            //    Node {
            //        ..default()
            //    },
            //    TextFont {
            //        font_size: 28.0,
            //        ..default()
            //    },
            //    TextColor(Color::WHITE),
            //    Text::new(""),
            //    EndlessLivesRemainingText,
            //    DespawnOnStateExit::App(AppState::Game)
            //));
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