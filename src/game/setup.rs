use bevy::prelude::*;
use rand::seq::SliceRandom;

use crate::playing_cards::PlayingCard;

use super::{
    CardEntity, CardFaceState, CardTextures, CpuMemory, CpuTurnTimer, FlippedCards, GameData,
    GamePhase, Phase, Player, WhoseTurn, UiRoot, PlayerScoreText, CpuScoreText, TurnIndicatorText,
    CARD_H, CARD_W, COLS, card_position,
};

/// ゲーム開始時: カードをシャッフルしてグリッドに配置する
pub fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_data: ResMut<GameData>,
    mut flipped: ResMut<FlippedCards>,
    mut cpu_memory: ResMut<CpuMemory>,
    mut whose_turn: ResMut<WhoseTurn>,
    mut cpu_timer: ResMut<CpuTurnTimer>,
    mut phase: ResMut<GamePhase>,
) {
    // リソースをリセット
    *game_data = GameData {
        player_score: 0,
        cpu_score: 0,
        pairs_found: 0,
    };
    *flipped = FlippedCards::default();
    *cpu_memory = CpuMemory::default();
    whose_turn.current = Player::Human;
    *cpu_timer = CpuTurnTimer::default();
    phase.0 = Phase::PlayerTurn;

    // 52 枚のカードインデックスをシャッフル
    let mut indices: Vec<usize> = (0..52).collect();
    let mut rng = rand::rng();
    indices.shuffle(&mut rng);

    let back_texture: Handle<Image> =
        asset_server.load("textures/playing_cards/card_back.png");

    // CardTextures リソースを挿入（flip_animation.rs で使用）
    commands.insert_resource(CardTextures {
        back: back_texture.clone(),
    });

    // 52 枚のカードエンティティを生成
    for (i, &card_index) in indices.iter().enumerate() {
        let card = PlayingCard::from(card_index as i32);
        let (suite, rank) = match &card {
            PlayingCard::Standard(s, r) => (*s, *r),
            PlayingCard::Joker(_) => continue, // ジョーカーはスキップ（0..52 なので起こらない）
        };

        let col = i % COLS;
        let row = i / COLS;
        let pos = card_position(col, row);

        commands.spawn((
            Sprite {
                image: back_texture.clone(),
                custom_size: Some(Vec2::new(CARD_W, CARD_H)),
                ..default()
            },
            Transform::from_xyz(pos.x, pos.y, 0.0),
            CardEntity {
                rank,
                suite,
                grid_index: i,
            },
            CardFaceState::FaceDown,
        ));
    }
}

/// UI バー（上部: スコア・ターン表示）を生成する
pub fn setup_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(58.0),
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            BackgroundColor(Color::srgba(0.08, 0.08, 0.12, 0.92)),
            UiRoot,
        ))
        .with_children(|parent| {
            // プレイヤースコア（左）
            parent.spawn((
                Text::new("プレイヤー: 0 ペア"),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::srgb(0.4, 0.9, 0.4)),
                PlayerScoreText,
            ));

            // ターン表示（中央）
            parent.spawn((
                Text::new("あなたのターン"),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TurnIndicatorText,
            ));

            // CPU スコア（右）
            parent.spawn((
                Text::new("CPU: 0 ペア"),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.4, 0.4)),
                CpuScoreText,
            ));
        });
}
