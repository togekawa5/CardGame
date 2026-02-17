use bevy::prelude::*;

use super::{
    AppState, CardEntity, CardFaceState, CardFlipRequested, CpuPickPhase, CpuTurnTimer,
    FlipAnimation, FlipAnimationComplete, FlipPhase, FlippedCards, GameData, GamePhase, MatchResult,
    Phase, Player, WhoseTurn, CARD_H, CARD_W,
};

/// プレイヤーのマウスクリックを検出してカードを選択する
pub fn handle_player_click(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    phase: Res<GamePhase>,
    mut flipped: ResMut<FlippedCards>,
    cards: Query<(Entity, &Transform, &CardFaceState)>,
    mut commands: Commands,
) {
    // プレイヤーのターンかつ左クリック時のみ
    if phase.0 != Phase::PlayerTurn {
        return;
    }
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((camera, cam_transform)) = cameras.single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok(world_pos) = camera.viewport_to_world_2d(cam_transform, cursor_pos) else {
        return;
    };

    let half_w = CARD_W / 2.0;
    let half_h = CARD_H / 2.0;

    for (entity, transform, face_state) in &cards {
        // 裏向きカードのみ選択可能
        if *face_state != CardFaceState::FaceDown {
            continue;
        }
        // すでに 1 枚目で選択済みのカードは再選択不可
        if flipped.first == Some(entity) {
            continue;
        }

        let pos = transform.translation.xy();
        if world_pos.x >= pos.x - half_w
            && world_pos.x <= pos.x + half_w
            && world_pos.y >= pos.y - half_h
            && world_pos.y <= pos.y + half_h
        {
            // FlippedCards に登録
            if flipped.first.is_none() {
                flipped.first = Some(entity);
            } else {
                flipped.second = Some(entity);
            }
            commands.trigger(CardFlipRequested {
                entity,
                target_face_up: true,
            });
            return; // 1 クリックにつき 1 枚まで
        }
    }
}

/// CardFlipRequested イベントを受け取り、FlipAnimation コンポーネントを付与する（Observer）
pub fn handle_flip_request(
    event: On<CardFlipRequested>,
    mut commands: Commands,
    mut card_states: Query<&mut CardFaceState>,
    mut phase: ResMut<GamePhase>,
) {
    let ev = event.event();
    // FlipAnimation をエンティティに付与
    commands.entity(ev.entity).insert(FlipAnimation {
        timer: Timer::from_seconds(0.15, TimerMode::Once),
        phase: FlipPhase::Phase1,
        target_face_up: ev.target_face_up,
    });

    // カードの論理状態を即時更新
    if let Ok(mut state) = card_states.get_mut(ev.entity) {
        if ev.target_face_up {
            *state = CardFaceState::FaceUp;
        } else {
            *state = CardFaceState::FaceDown;
        }
    }

    // アニメーション中はフェーズを Animating に
    phase.0 = Phase::Animating;
}

/// FlipAnimationComplete イベントを受け取り、次のゲームフェーズを決定する（Observer）
pub fn handle_flip_complete(
    _event: On<FlipAnimationComplete>,
    animations: Query<(), With<FlipAnimation>>,
    flipped: Res<FlippedCards>,
    whose_turn: Res<WhoseTurn>,
    mut cpu_timer: ResMut<CpuTurnTimer>,
    mut phase: ResMut<GamePhase>,
    mut whose_turn_mut: ResMut<WhoseTurn>,
) {
    // 残りのアニメーションがある場合はまだ待機
    if !animations.is_empty() {
        return;
    }
    if phase.0 != Phase::Animating {
        return;
    }

    if flipped.second.is_some() {
        // 2 枚目のフリップ完了 → マッチ判定フェーズへ
        phase.0 = Phase::CheckingMatch;
    } else if flipped.first.is_some() {
        // 1 枚目のフリップ完了 → 同じプレイヤーが 2 枚目を選ぶ
        match whose_turn.current {
            Player::Human => phase.0 = Phase::PlayerTurn,
            Player::Cpu => {
                cpu_timer.pick_phase = CpuPickPhase::SecondCard;
                cpu_timer.pick_timer =
                    Timer::from_seconds(0.8, TimerMode::Once);
                phase.0 = Phase::CpuTurn;
            }
        }
    } else {
        // FlippedCards が空 = 裏返しアニメーション完了 → ターン交代
        match whose_turn.current {
            Player::Human => {
                whose_turn_mut.current = Player::Cpu;
                cpu_timer.pick_phase = CpuPickPhase::FirstCard;
                cpu_timer.pick_timer =
                    Timer::from_seconds(1.2, TimerMode::Once);
                phase.0 = Phase::CpuTurn;
            }
            Player::Cpu => {
                whose_turn_mut.current = Player::Human;
                phase.0 = Phase::PlayerTurn;
            }
        }
    }
}

/// 2 枚めくった後の 0.8 秒タイマーを管理し、MatchResult イベントを送出する
pub fn check_match_timing(
    phase: Res<GamePhase>,
    time: Res<Time>,
    mut local_timer: Local<Option<Timer>>,
    flipped: Res<FlippedCards>,
    cards: Query<&CardEntity>,
    mut commands: Commands,
) {
    if phase.0 != Phase::CheckingMatch {
        // フェーズが外れたらタイマーをリセット
        *local_timer = None;
        return;
    }

    // タイマーを初期化（初回）
    let timer = local_timer.get_or_insert_with(|| Timer::from_seconds(0.8, TimerMode::Once));
    timer.tick(time.delta());

    if !timer.is_finished() {
        return;
    }

    // タイマーリセット（次回のためにクリア）
    *local_timer = None;

    let (Some(first), Some(second)) = (flipped.first, flipped.second) else {
        return;
    };
    let (Ok(c1), Ok(c2)) = (cards.get(first), cards.get(second)) else {
        return;
    };

    let matched = c1.rank == c2.rank;
    commands.trigger(MatchResult {
        matched,
        first,
        second,
    });
}

/// MatchResult イベントを処理して、スコア更新・次ターン遷移・ゲーム終了を行う（Observer）
pub fn handle_match_result(
    event: On<MatchResult>,
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    mut flipped: ResMut<FlippedCards>,
    mut cpu_memory: ResMut<super::CpuMemory>,
    card_query: Query<&CardEntity>,
    mut card_states: Query<&mut CardFaceState>,
    mut phase: ResMut<GamePhase>,
    whose_turn: Res<WhoseTurn>,
    mut cpu_timer: ResMut<CpuTurnTimer>,
    mut next_app: ResMut<NextState<AppState>>,
) {
    let ev = event.event();
    if ev.matched {
        // マッチ成功: カードを Matched 状態に変更
        if let Ok(mut s) = card_states.get_mut(ev.first) {
            *s = CardFaceState::Matched;
        }
        if let Ok(mut s) = card_states.get_mut(ev.second) {
            *s = CardFaceState::Matched;
        }

        // CPU の記憶からマッチしたカードを削除
        if let Ok(c) = card_query.get(ev.first) {
            cpu_memory.seen.remove(&c.grid_index);
        }
        if let Ok(c) = card_query.get(ev.second) {
            cpu_memory.seen.remove(&c.grid_index);
        }

        // スコアを更新
        match whose_turn.current {
            Player::Human => game_data.player_score += 1,
            Player::Cpu => game_data.cpu_score += 1,
        }
        game_data.pairs_found += 1;

        // FlippedCards をクリア
        flipped.first = None;
        flipped.second = None;

        // ゲーム終了判定（26 ペアで終わり）
        if game_data.pairs_found >= 26 {
            next_app.set(AppState::GameOver);
        } else {
            // 同じプレイヤーが続けてプレイ
            match whose_turn.current {
                Player::Human => phase.0 = Phase::PlayerTurn,
                Player::Cpu => {
                    cpu_timer.pick_phase = CpuPickPhase::FirstCard;
                    cpu_timer.pick_timer =
                        Timer::from_seconds(0.8, TimerMode::Once);
                    phase.0 = Phase::CpuTurn;
                }
            }
        }
    } else {
        // ミスマッチ: FlippedCards をクリアしてから両カードを裏に戻す
        flipped.first = None;
        flipped.second = None;

        commands.trigger(CardFlipRequested {
            entity: ev.first,
            target_face_up: false,
        });
        commands.trigger(CardFlipRequested {
            entity: ev.second,
            target_face_up: false,
        });

        // Animating フェーズへ（裏返しアニメーション中）
        phase.0 = Phase::Animating;
    }
}
