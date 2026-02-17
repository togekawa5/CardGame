use bevy::prelude::*;
use rand::prelude::IndexedRandom;

use crate::playing_cards::Rank;

use super::{
    CardEntity, CardFaceState, CardFlipRequested, CpuMemory, CpuPickPhase, CpuTurnTimer,
    FlipAnimationComplete, FlippedCards, GamePhase, Phase, Player, WhoseTurn,
};

/// CPU のターン処理（ディレイ付きでカードを選択する）
pub fn cpu_turn(
    mut phase: ResMut<GamePhase>,
    time: Res<Time>,
    mut cpu_timer: ResMut<CpuTurnTimer>,
    cpu_memory: Res<CpuMemory>,
    mut flipped: ResMut<FlippedCards>,
    cards: Query<(Entity, &CardEntity, &CardFaceState)>,
    mut commands: Commands,
    whose_turn: Res<WhoseTurn>,
) {
    if phase.0 != Phase::CpuTurn || whose_turn.current != Player::Cpu {
        return;
    }

    cpu_timer.pick_timer.tick(time.delta());
    if !cpu_timer.pick_timer.is_finished() {
        return;
    }

    // 裏向きのカード一覧を収集
    let face_down: Vec<(Entity, &CardEntity)> = cards
        .iter()
        .filter(|(_, _, s)| **s == CardFaceState::FaceDown)
        .map(|(e, c, _)| (e, c))
        .collect();

    if face_down.is_empty() {
        return;
    }

    match cpu_timer.pick_phase {
        CpuPickPhase::FirstCard => {
            let chosen = cpu_pick_first(&face_down, &cpu_memory);
            flipped.first = Some(chosen);
            commands.trigger(CardFlipRequested {
                entity: chosen,
                target_face_up: true,
            });
            phase.0 = Phase::Animating;
        }
        CpuPickPhase::SecondCard => {
            let Some(first_entity) = flipped.first else {
                return;
            };
            let Ok((_, first_card, _)) = cards.get(first_entity) else {
                return;
            };
            let chosen =
                cpu_pick_second(first_card, &face_down, &cpu_memory, first_entity);
            flipped.second = Some(chosen);
            commands.trigger(CardFlipRequested {
                entity: chosen,
                target_face_up: true,
            });
            phase.0 = Phase::Animating;
        }
    }
}

/// フリップ完了時にカードを CPU の記憶に記録する（Observer）
pub fn record_to_cpu_memory(
    event: On<FlipAnimationComplete>,
    card_query: Query<(&CardEntity, &CardFaceState)>,
    mut cpu_memory: ResMut<CpuMemory>,
) {
    let completed_entity = event.event().entity;
    if let Ok((card, state)) = card_query.get(completed_entity) {
        if *state == CardFaceState::FaceUp {
            // 表向きになったカードを記憶に登録
            cpu_memory.seen.insert(card.grid_index, (card.rank, card.suite));
        }
    }
}

// ─── CPU 選択ロジック ──────────────────────────────────────────────────────────

/// 1 枚目のカードを選ぶ
///
/// - 記憶の中に同ランクのペアがあれば → そちらを優先選択（確実マッチ）
/// - なければ → ランダム選択
pub(crate) fn cpu_pick_first(
    face_down: &[(Entity, &CardEntity)],
    memory: &CpuMemory,
) -> Entity {
    // 記憶内で同ランクのカードが 2 枚以上あるか調べる
    if let Some(entity) = find_known_pair_first(face_down, memory) {
        return entity;
    }
    // ランダム選択
    let mut rng = rand::rng();
    face_down.choose(&mut rng).unwrap().0
}

/// 2 枚目のカードを選ぶ
///
/// - 1 枚目と同ランクのカードが記憶にあれば → そちらを選択（確実マッチ）
/// - なければ → ランダム選択（1 枚目を除く）
pub(crate) fn cpu_pick_second(
    first_card: &CardEntity,
    face_down: &[(Entity, &CardEntity)],
    memory: &CpuMemory,
    first_entity: Entity,
) -> Entity {
    // 記憶の中から 1 枚目と同ランクを探す
    for (idx, (rank, _)) in &memory.seen {
        if *rank == first_card.rank {
            if let Some((e, _)) = face_down.iter().find(|(_, c)| c.grid_index == *idx) {
                if *e != first_entity {
                    return *e;
                }
            }
        }
    }
    // 記憶になければランダム（1 枚目以外）
    let mut rng = rand::rng();
    let candidates: Vec<_> = face_down
        .iter()
        .filter(|(e, _)| *e != first_entity)
        .collect();
    candidates.choose(&mut rng).map(|(e, _)| *e).unwrap_or(first_entity)
}

/// 記憶内で確実なペアがある場合に、その 1 枚目を返す
pub(crate) fn find_known_pair_first(
    face_down: &[(Entity, &CardEntity)],
    memory: &CpuMemory,
) -> Option<Entity> {
    // 記憶にあるランク別のカウント
    let mut rank_entries: Vec<(Rank, usize)> = Vec::new();
    for (idx, (rank, _)) in &memory.seen {
        // face_down に実際にあるか確認
        if face_down.iter().any(|(_, c)| c.grid_index == *idx) {
            if let Some(entry) = rank_entries.iter_mut().find(|(r, _)| *r == *rank) {
                entry.1 += 1;
            } else {
                rank_entries.push((*rank, 1));
            }
        }
    }

    // 同ランクが 2 枚以上あれば、その 1 枚目を選択
    for (rank, count) in &rank_entries {
        if *count >= 2 {
            // 記憶の中からそのランクの最初のカードを face_down で探す
            for (idx, (mem_rank, _)) in &memory.seen {
                if *mem_rank == *rank {
                    if let Some((entity, _)) =
                        face_down.iter().find(|(_, c)| c.grid_index == *idx)
                    {
                        return Some(*entity);
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::playing_cards::{Rank, Suite};

    /// テスト用 Entity を n 個生成するヘルパー（World 経由で有効な Entity を取得）
    fn make_entities(n: usize) -> Vec<Entity> {
        let mut world = bevy::ecs::world::World::new();
        (0..n).map(|_| world.spawn_empty().id()).collect()
    }

    /// テスト用に CardEntity を生成するヘルパー
    fn card(grid_index: usize, rank: Rank, suite: Suite) -> CardEntity {
        CardEntity { rank, suite, grid_index }
    }

    // ─── find_known_pair_first ───────────────────────────────────────────────

    #[test]
    fn pair_empty_memory_returns_none() {
        let es = make_entities(1);
        let c0 = card(0, Rank::Ace, Suite::Spades);
        let memory = CpuMemory::default();
        assert!(find_known_pair_first(&[(es[0], &c0)], &memory).is_none());
    }

    #[test]
    fn pair_single_card_in_memory_returns_none() {
        let es = make_entities(1);
        let c0 = card(0, Rank::Ace, Suite::Spades);
        let mut memory = CpuMemory::default();
        memory.seen.insert(0, (Rank::Ace, Suite::Spades));
        // 1 枚だけではペアにならない
        assert!(find_known_pair_first(&[(es[0], &c0)], &memory).is_none());
    }

    #[test]
    fn pair_two_same_rank_in_memory_returns_some() {
        let es = make_entities(2);
        let c0 = card(0, Rank::Ace, Suite::Spades);
        let c1 = card(1, Rank::Ace, Suite::Hearts);
        let mut memory = CpuMemory::default();
        memory.seen.insert(0, (Rank::Ace, Suite::Spades));
        memory.seen.insert(1, (Rank::Ace, Suite::Hearts));
        let result = find_known_pair_first(&[(es[0], &c0), (es[1], &c1)], &memory);
        assert!(result.is_some(), "同ランク 2 枚がある場合は Some を返すべき");
    }

    #[test]
    fn pair_different_ranks_returns_none() {
        let es = make_entities(2);
        let c0 = card(0, Rank::Ace,  Suite::Spades);
        let c1 = card(1, Rank::King, Suite::Hearts);
        let mut memory = CpuMemory::default();
        memory.seen.insert(0, (Rank::Ace,  Suite::Spades));
        memory.seen.insert(1, (Rank::King, Suite::Hearts));
        // ランクが異なるのでペアなし
        assert!(find_known_pair_first(&[(es[0], &c0), (es[1], &c1)], &memory).is_none());
    }

    #[test]
    fn pair_one_card_not_in_face_down_returns_none() {
        // 記憶に 2 枚あるが、片方がすでにマッチ済み（face_down に存在しない）
        let es = make_entities(1);
        let c0 = card(0, Rank::Ace, Suite::Spades);
        let mut memory = CpuMemory::default();
        memory.seen.insert(0,  (Rank::Ace, Suite::Spades));
        memory.seen.insert(99, (Rank::Ace, Suite::Hearts)); // face_down にない
        assert!(find_known_pair_first(&[(es[0], &c0)], &memory).is_none());
    }

    // ─── cpu_pick_second ─────────────────────────────────────────────────────

    #[test]
    fn pick_second_finds_matching_rank_from_memory() {
        let es = make_entities(2);
        let first = card(0, Rank::King, Suite::Spades);
        let match_card = card(1, Rank::King, Suite::Hearts);
        let mut memory = CpuMemory::default();
        memory.seen.insert(1, (Rank::King, Suite::Hearts));
        let face_down = vec![(es[0], &first), (es[1], &match_card)];
        let result = cpu_pick_second(&first, &face_down, &memory, es[0]);
        assert_eq!(result, es[1], "記憶から同ランクを選ぶべき");
    }

    #[test]
    fn pick_second_skips_first_entity() {
        // 記憶がない場合は 1 枚目以外からランダム選択
        let es = make_entities(2);
        let first = card(0, Rank::Ace, Suite::Spades);
        let other = card(1, Rank::Two, Suite::Hearts);
        let memory = CpuMemory::default();
        let face_down = vec![(es[0], &first), (es[1], &other)];
        let result = cpu_pick_second(&first, &face_down, &memory, es[0]);
        assert_ne!(result, es[0], "1 枚目のカード自身を選んではいけない");
        assert_eq!(result, es[1], "唯一の候補 es[1] を選ぶべき");
    }

    #[test]
    fn pick_second_ignores_memory_match_if_its_the_first_entity() {
        // 記憶の中の同ランクが 1 枚目自身の場合は別のカードを選ぶ
        let es = make_entities(2);
        let first = card(0, Rank::Queen, Suite::Spades);
        let other = card(1, Rank::Three, Suite::Clubs);
        let mut memory = CpuMemory::default();
        // grid_index=0 の Spades/Queen = 1 枚目自身を記憶（無効なペア）
        memory.seen.insert(0, (Rank::Queen, Suite::Spades));
        let face_down = vec![(es[0], &first), (es[1], &other)];
        let result = cpu_pick_second(&first, &face_down, &memory, es[0]);
        assert_ne!(result, es[0], "記憶が 1 枚目自身でも選んではいけない");
    }
}
