use bevy::prelude::*;

mod card_token;
mod playing_cards;
mod game;

use game::{AppState, GamePlugin};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "神経衰弱 (Concentration)".to_string(),
                    resolution: (1280_u32, 720_u32).into(),
                    ..default()
                }),
                ..default()
            }),
        )
        .init_state::<AppState>()
        .add_systems(Startup, setup_camera)
        .add_plugins(GamePlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
