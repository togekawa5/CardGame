use bevy::prelude::*;

/// プレイヤーまたは CPU がカードをめくる要求
#[derive(Event)]
pub struct CardFlipRequested {
    pub entity: Entity,
    pub target_face_up: bool,
}

/// 1 枚のカードのフリップアニメーションが完了した
#[derive(Event)]
pub struct FlipAnimationComplete {
    pub entity: Entity,
}

/// 2 枚のカードが判定された結果
#[derive(Event)]
pub struct MatchResult {
    pub matched: bool,
    pub first: Entity,
    pub second: Entity,
}
