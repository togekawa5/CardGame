use bevy::prelude::*;

mod components;
mod events;
mod setup;
mod flip_animation;
mod turn_logic;
mod cpu_ai;
mod ui;
mod cleanup;

pub use components::*;
pub use events::*;

/// アプリケーション全体の状態
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Playing,
    GameOver,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // リソース初期化
            .insert_resource(GameData {
                player_score: 0,
                cpu_score: 0,
                pairs_found: 0,
            })
            .insert_resource(FlippedCards::default())
            .insert_resource(CpuMemory::default())
            .insert_resource(WhoseTurn {
                current: Player::Human,
            })
            .insert_resource(CpuTurnTimer::default())
            .insert_resource(GamePhase(Phase::PlayerTurn))
            // ─── Observer 登録（イベント駆動ロジック）────────────────────────
            .add_observer(turn_logic::handle_flip_request)
            .add_observer(turn_logic::handle_flip_complete)
            .add_observer(turn_logic::handle_match_result)
            .add_observer(cpu_ai::record_to_cpu_memory)
            // ─── 状態遷移フック ────────────────────────────────────────────
            .add_systems(
                OnEnter(AppState::Playing),
                (setup::setup_game, setup::setup_ui).chain(),
            )
            .add_systems(OnEnter(AppState::GameOver), ui::spawn_game_over)
            .add_systems(OnExit(AppState::Playing), cleanup::cleanup_game)
            .add_systems(OnExit(AppState::GameOver), cleanup::cleanup_game_over)
            // ─── ゲームプレイ中のシステム（毎フレーム更新）───────────────
            .add_systems(
                Update,
                (
                    turn_logic::handle_player_click,
                    flip_animation::animate_card_flips,
                    turn_logic::check_match_timing,
                    cpu_ai::cpu_turn,
                    ui::update_score_ui,
                    ui::update_turn_indicator,
                    ui::update_matched_card_visuals,
                )
                    .chain()
                    .run_if(in_state(AppState::Playing)),
            )
            // ゲームオーバー画面でのリスタート
            .add_systems(
                Update,
                ui::handle_restart.run_if(in_state(AppState::GameOver)),
            );
    }
}
