use bevy::prelude::*;

use super::{
    AppState, CardFaceState, CpuScoreText, GameData, GameOverScreen, GamePhase, Phase, Player,
    PlayerScoreText, TurnIndicatorText, WhoseTurn,
};

/// プレイヤーと CPU のスコア表示を更新する
pub fn update_score_ui(
    game_data: Res<GameData>,
    mut player_q: Query<&mut Text, (With<PlayerScoreText>, Without<CpuScoreText>)>,
    mut cpu_q: Query<&mut Text, (With<CpuScoreText>, Without<PlayerScoreText>)>,
) {
    if !game_data.is_changed() {
        return;
    }
    if let Ok(mut text) = player_q.single_mut() {
        text.0 = format!("プレイヤー: {} ペア", game_data.player_score);
    }
    if let Ok(mut text) = cpu_q.single_mut() {
        text.0 = format!("CPU: {} ペア", game_data.cpu_score);
    }
}

/// ターンインジケーターのテキストと色を更新する
pub fn update_turn_indicator(
    whose_turn: Res<WhoseTurn>,
    phase: Res<GamePhase>,
    mut text_q: Query<(&mut Text, &mut TextColor), With<TurnIndicatorText>>,
) {
    if !whose_turn.is_changed() && !phase.is_changed() {
        return;
    }
    let Ok((mut text, mut color)) = text_q.single_mut() else {
        return;
    };

    match (&whose_turn.current, &phase.0) {
        (Player::Human, Phase::PlayerTurn) => {
            text.0 = "あなたのターン".to_string();
            color.0 = Color::srgb(0.4, 1.0, 0.4);
        }
        (Player::Human, _) => {
            text.0 = "あなたのターン（待機中）".to_string();
            color.0 = Color::srgb(1.0, 1.0, 0.4);
        }
        (Player::Cpu, _) => {
            text.0 = "CPU のターン".to_string();
            color.0 = Color::srgb(1.0, 0.4, 0.4);
        }
    }
}

/// マッチしたカードをグレーアウト表示にする
pub fn update_matched_card_visuals(
    mut card_q: Query<(&CardFaceState, &mut Sprite), Changed<CardFaceState>>,
) {
    for (state, mut sprite) in card_q.iter_mut() {
        sprite.color = match state {
            CardFaceState::Matched => Color::srgba(0.5, 0.5, 0.5, 0.45),
            _ => Color::WHITE,
        };
    }
}

/// ゲームオーバー画面を生成する
pub fn spawn_game_over(mut commands: Commands, game_data: Res<GameData>) {
    let winner_text = if game_data.player_score > game_data.cpu_score {
        "あなたの勝ち！"
    } else if game_data.cpu_score > game_data.player_score {
        "CPU の勝ち！"
    } else {
        "引き分け！"
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.75)),
            GlobalZIndex(100),
            GameOverScreen,
        ))
        .with_children(|parent| {
            // 勝者テキスト
            parent.spawn((
                Text::new(winner_text),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.9, 0.2)),
            ));

            // スコア表示
            parent.spawn((
                Text::new(format!(
                    "プレイヤー {} : CPU {} （ペア数）",
                    game_data.player_score, game_data.cpu_score
                )),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // 再プレイ案内
            parent.spawn((
                Text::new("クリックで再プレイ"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
}

/// ゲームオーバー画面でクリックしたらゲームを再スタートする
pub fn handle_restart(
    mouse: Res<ButtonInput<MouseButton>>,
    mut next_app: ResMut<NextState<AppState>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        next_app.set(AppState::Playing);
    }
}
