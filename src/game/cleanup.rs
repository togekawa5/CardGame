use bevy::prelude::*;

use super::{CardEntity, GameOverScreen, UiRoot};

/// AppState::Playing を抜けるとき、ゲームエンティティをすべて削除する
pub fn cleanup_game(
    mut commands: Commands,
    cards: Query<Entity, With<CardEntity>>,
    ui_roots: Query<Entity, With<UiRoot>>,
) {
    for entity in cards.iter() {
        commands.entity(entity).despawn();
    }
    for entity in ui_roots.iter() {
        commands.entity(entity).despawn();
    }
}

/// AppState::GameOver を抜けるとき、ゲームオーバー画面を削除する
pub fn cleanup_game_over(
    mut commands: Commands,
    screens: Query<Entity, With<GameOverScreen>>,
) {
    for entity in screens.iter() {
        commands.entity(entity).despawn();
    }
}
