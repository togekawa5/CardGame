use bevy::prelude::*;

use super::{
    CardEntity, CardFaceState, CardTextures, FlipAnimation, FlipAnimationComplete, FlipPhase,
};

/// カードのフリップアニメーションを毎フレーム更新する
///
/// Phase1: scale.x を 1.0 → 0.0 に縮める
/// Phase2: テクスチャを差し替えて scale.x を 0.0 → 1.0 に展開する
pub fn animate_card_flips(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut FlipAnimation,
        &mut Transform,
        &mut Sprite,
        &CardEntity,
        &CardFaceState,
    )>,
    card_textures: Res<CardTextures>,
    asset_server: Res<AssetServer>,
) {
    for (entity, mut anim, mut transform, mut sprite, card, _face_state) in query.iter_mut() {
        anim.timer.tick(time.delta());

        match anim.phase {
            FlipPhase::Phase1 => {
                // scale.x: 1.0 → 0.0
                let progress = anim.timer.fraction();
                transform.scale.x = 1.0 - progress;

                if anim.timer.is_finished() {
                    // テクスチャを差し替え
                    if anim.target_face_up {
                        let path = front_texture_path(&card);
                        sprite.image = asset_server.load(path);
                    } else {
                        sprite.image = card_textures.back.clone();
                    }
                    // Phase2 に移行
                    anim.phase = FlipPhase::Phase2;
                    anim.timer = Timer::from_seconds(0.15, TimerMode::Once);
                    transform.scale.x = 0.0;
                }
            }
            FlipPhase::Phase2 => {
                // scale.x: 0.0 → 1.0
                let progress = anim.timer.fraction();
                transform.scale.x = progress;

                if anim.timer.is_finished() {
                    transform.scale.x = 1.0;
                    // アニメーションコンポーネントを削除してからイベントを発火
                    commands.entity(entity).remove::<FlipAnimation>();
                    commands.trigger(FlipAnimationComplete { entity });
                }
            }
        }
    }
}

/// PlayingCard の suite/rank からテクスチャパスを生成する
fn front_texture_path(card: &CardEntity) -> String {
    use crate::playing_cards::{Rank, Suite};

    let suite_str = match card.suite {
        Suite::Hearts => "heart",
        Suite::Diamonds => "diamond",
        Suite::Clubs => "club",
        Suite::Spades => "spade",
    };
    let rank_str = match card.rank {
        Rank::Ace => "01",
        Rank::Two => "02",
        Rank::Three => "03",
        Rank::Four => "04",
        Rank::Five => "05",
        Rank::Six => "06",
        Rank::Seven => "07",
        Rank::Eight => "08",
        Rank::Nine => "09",
        Rank::Ten => "10",
        Rank::Jack => "11",
        Rank::Queen => "12",
        Rank::King => "13",
    };
    format!("textures/playing_cards/card_{}_{}.png", suite_str, rank_str)
}
