use bevy::prelude::*;
use std::collections::HashMap;
use crate::playing_cards::{Rank, Suite};

// カードサイズ・グリッド定数
pub const CARD_W: f32 = 80.0;
pub const CARD_H: f32 = 120.0;
pub const H_GAP: f32 = 8.0;
pub const V_GAP: f32 = 10.0;
pub const COLS: usize = 13;
pub const ROWS: usize = 4;

/// グリッド座標からワールド座標を計算する
pub fn card_position(col: usize, row: usize) -> Vec2 {
    let total_w = COLS as f32 * CARD_W + (COLS - 1) as f32 * H_GAP;
    let start_x = -total_w / 2.0 + CARD_W / 2.0;
    Vec2::new(
        start_x + col as f32 * (CARD_W + H_GAP),
        220.0 - row as f32 * (CARD_H + V_GAP),
    )
}

// ─── カード識別コンポーネント ───────────────────────────────────────────────────
#[derive(Component, Clone)]
pub struct CardEntity {
    pub rank: Rank,
    pub suite: Suite,
    pub grid_index: usize,
}

// ─── カードの表裏・マッチ状態 ──────────────────────────────────────────────────
#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum CardFaceState {
    FaceDown,
    FaceUp,
    Matched,
}

// ─── フリップアニメーションコンポーネント ────────────────────────────────────
/// カードエンティティに付与し、アニメーション終了後に自動削除される
#[derive(Component)]
pub struct FlipAnimation {
    pub timer: Timer,
    pub phase: FlipPhase,
    pub target_face_up: bool,
}

#[derive(PartialEq, Eq, Clone)]
pub enum FlipPhase {
    Phase1, // scale.x: 1.0 → 0.0（縮小）
    Phase2, // scale.x: 0.0 → 1.0（展開）、テクスチャ差し替え済み
}

// ─── UI マーカーコンポーネント ─────────────────────────────────────────────────
#[derive(Component)]
pub struct PlayerScoreText;
#[derive(Component)]
pub struct CpuScoreText;
#[derive(Component)]
pub struct TurnIndicatorText;
#[derive(Component)]
pub struct GameOverScreen;
#[derive(Component)]
pub struct UiRoot;

// ─── ゲームデータリソース ──────────────────────────────────────────────────────
#[derive(Resource)]
pub struct GameData {
    pub player_score: u32,
    pub cpu_score: u32,
    pub pairs_found: u32,
}

/// 現在めくられている最大 2 枚のカードを追跡するリソース
#[derive(Resource, Default)]
pub struct FlippedCards {
    pub first: Option<Entity>,
    pub second: Option<Entity>,
}

/// CPU が過去に見たカードを記憶するリソース
#[derive(Resource, Default)]
pub struct CpuMemory {
    pub seen: HashMap<usize, (Rank, Suite)>,
}

/// 今誰のターンか
#[derive(Resource)]
pub struct WhoseTurn {
    pub current: Player,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Human,
    Cpu,
}

/// CPU のターンタイマーとフェーズ
#[derive(Resource)]
pub struct CpuTurnTimer {
    pub pick_timer: Timer,
    pub pick_phase: CpuPickPhase,
}

impl Default for CpuTurnTimer {
    fn default() -> Self {
        Self {
            pick_timer: Timer::from_seconds(1.2, TimerMode::Once),
            pick_phase: CpuPickPhase::FirstCard,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum CpuPickPhase {
    FirstCard,
    SecondCard,
}

/// カード裏面テクスチャのハンドル（全カードで共有）
#[derive(Resource)]
pub struct CardTextures {
    pub back: Handle<Image>,
}

/// ゲームフェーズ（ターン管理）
#[derive(Resource)]
pub struct GamePhase(pub Phase);

#[derive(PartialEq, Eq, Clone)]
pub enum Phase {
    PlayerTurn,    // プレイヤーのクリック待ち
    CpuTurn,       // CPU の行動（タイマー付き）
    Animating,     // フリップアニメーション中（入力ブロック）
    CheckingMatch, // 2 枚めくった後の判定待ち（0.8 秒）
}

#[cfg(test)]
mod tests {
    use super::*;

    // ─── card_position のグリッド配置テスト ──────────────────────────────────

    /// 左端カードの座標を計算する
    ///
    /// total_w = COLS * CARD_W + (COLS-1) * H_GAP = 13*80 + 12*8 = 1136
    /// start_x = -1136/2 + 80/2 = -568 + 40 = -528
    #[test]
    fn card_position_first_card() {
        let pos = card_position(0, 0);
        assert!(
            (pos.x - (-528.0)).abs() < 0.001,
            "左端 x = {} (期待: -528)", pos.x
        );
        assert!(
            (pos.y - 220.0).abs() < 0.001,
            "最上段 y = {} (期待: 220)", pos.y
        );
    }

    #[test]
    fn card_position_last_card() {
        // card_position(12, 3): x = -528 + 12*88 = 528, y = 220 - 3*130 = -170
        let pos = card_position(COLS - 1, 3);
        assert!(
            (pos.x - 528.0).abs() < 0.001,
            "右端 x = {} (期待: 528)", pos.x
        );
        assert!(
            (pos.y - (-170.0)).abs() < 0.001,
            "最下段 y = {} (期待: -170)", pos.y
        );
    }

    #[test]
    fn card_position_horizontal_spacing() {
        let p0 = card_position(0, 0);
        let p1 = card_position(1, 0);
        let expected_step = CARD_W + H_GAP;
        assert!(
            (p1.x - p0.x - expected_step).abs() < 0.001,
            "横間隔 = {} (期待: {})", p1.x - p0.x, expected_step
        );
    }

    #[test]
    fn card_position_vertical_spacing() {
        let p0 = card_position(0, 0);
        let p1 = card_position(0, 1);
        let expected_step = CARD_H + V_GAP;
        assert!(
            (p0.y - p1.y - expected_step).abs() < 0.001,
            "縦間隔 = {} (期待: {})", p0.y - p1.y, expected_step
        );
    }

    #[test]
    fn card_position_grid_centered() {
        // グリッドが X 方向に中央揃えされていることを確認
        // 左端と右端の x 座標の合計が 0 になるはず
        let first = card_position(0, 0);
        let last  = card_position(COLS - 1, 0);
        assert!(
            (first.x + last.x).abs() < 0.001,
            "中央揃え: {} + {} ≈ 0", first.x, last.x
        );
    }

    #[test]
    fn card_position_same_row_different_cols_same_y() {
        // 同じ行のカードは y 座標が一致する
        let p0 = card_position(0, 2);
        let p5 = card_position(5, 2);
        assert!((p0.y - p5.y).abs() < 0.001);
    }

    #[test]
    fn card_position_same_col_different_rows_same_x() {
        // 同じ列のカードは x 座標が一致する
        let p0 = card_position(3, 0);
        let p3 = card_position(3, 3);
        assert!((p0.x - p3.x).abs() < 0.001);
    }
}
