use crate::card_token::CardToken;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suite{
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rank{
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug)]
pub enum PlayingCard {
    Standard(Suite, Rank),
    Joker(i32), // 1 or 2 jokers
}

const CARD_WIDTH: f32 = 0.59;
const CARD_HEIGHT: f32 = 0.89;

impl CardToken for PlayingCard{
    fn size(&self) -> (f32, f32) {
        (CARD_WIDTH, CARD_HEIGHT)
    }

    fn flip(&mut self) {
        // Implementation for flipping the card
    }

    fn is_face_up(&self) -> bool {
        // Implementation to check if the card is face up
        true
    }

    fn display(&self) -> String {
        match self {
            PlayingCard::Standard(suite, rank) => {
                format!("{:?} of {:?}", rank, suite)
            }
            PlayingCard::Joker(num) => {
                format!("Joker {}", num)
            }
        }
    }

    fn front_texture_path(&self) -> String {
        front_texture_path(self)
    }

    fn back_texture_path(&self) -> String {
        back_texture_path()
    }

}

impl From<i32> for PlayingCard{
    fn from(value: i32) -> Self {
        if value < 52 {
            let suite = match value / 13 {
                1 => Suite::Hearts,
                2 => Suite::Diamonds,
                3 => Suite::Clubs,
                0 => Suite::Spades,
                _ => unreachable!(),
            };
            let rank = match value % 13 {
                0 => Rank::Ace,
                1 => Rank::Two,
                2 => Rank::Three,
                3 => Rank::Four,
                4 => Rank::Five,
                5 => Rank::Six,
                6 => Rank::Seven,
                7 => Rank::Eight,
                8 => Rank::Nine,
                9 => Rank::Ten,
                10 => Rank::Jack,
                11 => Rank::Queen,
                12 => Rank::King,
                _ => unreachable!(),
            };
            PlayingCard::Standard(suite, rank)
        } else {
            PlayingCard::Joker(value - 52)
        }
    }
}

impl Into<i32> for PlayingCard{
    fn into(self) -> i32 {
        match self {
            PlayingCard::Standard(suite, rank) => {
                let suite_value = match suite {
                    Suite::Hearts => 1,
                    Suite::Diamonds => 2,
                    Suite::Clubs => 3,
                    Suite::Spades => 0,
                };
                let rank_value = match rank {
                    Rank::Ace => 1,
                    Rank::Two => 2,
                    Rank::Three => 3,
                    Rank::Four => 4,
                    Rank::Five => 5,
                    Rank::Six => 6,
                    Rank::Seven => 7,
                    Rank::Eight => 8,
                    Rank::Nine => 9,
                    Rank::Ten => 10,
                    Rank::Jack => 11,
                    Rank::Queen => 12,
                    Rank::King => 13,
                };
                suite_value * 13 + rank_value
            }
            PlayingCard::Joker(num) => {
                52 + num
            }
        }
    }

}

fn front_texture_path(card: &PlayingCard) -> String {
    match card {
        PlayingCard::Standard(suite, rank) => {
            let suite_str = match suite {
                Suite::Hearts => "heart",
                Suite::Diamonds => "diamond",
                Suite::Clubs => "club",
                Suite::Spades => "spade",
            };
            let rank_str = match rank {
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
        PlayingCard::Joker(_) => {
            format!("textures/playing_cards/card_joker.png")
        }
    }
}

fn back_texture_path() -> String {
    format!("textures/playing_cards/card_back.png")
}

#[cfg(test)]
mod tests {
    use super::*;

    // ─── From<i32> スイート割り当てテスト ────────────────────────────────────────

    #[test]
    fn from_i32_spades_range() {
        // 0..=12 → Spades
        assert!(matches!(PlayingCard::from(0),  PlayingCard::Standard(Suite::Spades, Rank::Ace)));
        assert!(matches!(PlayingCard::from(12), PlayingCard::Standard(Suite::Spades, Rank::King)));
    }

    #[test]
    fn from_i32_hearts_range() {
        // 13..=25 → Hearts
        assert!(matches!(PlayingCard::from(13), PlayingCard::Standard(Suite::Hearts, Rank::Ace)));
        assert!(matches!(PlayingCard::from(25), PlayingCard::Standard(Suite::Hearts, Rank::King)));
    }

    #[test]
    fn from_i32_diamonds_range() {
        // 26..=38 → Diamonds
        assert!(matches!(PlayingCard::from(26), PlayingCard::Standard(Suite::Diamonds, Rank::Ace)));
        assert!(matches!(PlayingCard::from(38), PlayingCard::Standard(Suite::Diamonds, Rank::King)));
    }

    #[test]
    fn from_i32_clubs_range() {
        // 39..=51 → Clubs
        assert!(matches!(PlayingCard::from(39), PlayingCard::Standard(Suite::Clubs, Rank::Ace)));
        assert!(matches!(PlayingCard::from(51), PlayingCard::Standard(Suite::Clubs, Rank::King)));
    }

    // ─── From<i32> ランク順テスト ─────────────────────────────────────────────

    #[test]
    fn from_i32_rank_order_in_spades() {
        // Spades (0..13)：オフセットがそのままランクに対応する
        let expected = [
            Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five,
            Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
            Rank::Jack, Rank::Queen, Rank::King,
        ];
        for (offset, expected_rank) in expected.iter().enumerate() {
            match PlayingCard::from(offset as i32) {
                PlayingCard::Standard(Suite::Spades, rank) => {
                    assert_eq!(&rank, expected_rank, "index={offset}");
                }
                other => panic!("index={offset}: Spades を期待したが {:?}", other),
            }
        }
    }

    // ─── 52 枚すべてが Standard カード ────────────────────────────────────────

    #[test]
    fn from_i32_all_52_are_standard() {
        for i in 0..52_i32 {
            assert!(
                matches!(PlayingCard::from(i), PlayingCard::Standard(_, _)),
                "index={i} は Standard カードであるべき"
            );
        }
    }

    // ─── ジョーカーテスト ─────────────────────────────────────────────────────

    #[test]
    fn from_i32_joker() {
        assert!(matches!(PlayingCard::from(52), PlayingCard::Joker(0)));
        assert!(matches!(PlayingCard::from(53), PlayingCard::Joker(1)));
    }

    // ─── テクスチャパステスト ─────────────────────────────────────────────────

    #[test]
    fn texture_path_format() {
        // パス形式を確認
        let card = PlayingCard::Standard(Suite::Hearts, Rank::Ace);
        assert_eq!(card.front_texture_path(), "textures/playing_cards/card_heart_01.png");

        let card = PlayingCard::Standard(Suite::Spades, Rank::King);
        assert_eq!(card.front_texture_path(), "textures/playing_cards/card_spade_13.png");

        let card = PlayingCard::Standard(Suite::Diamonds, Rank::Ten);
        assert_eq!(card.front_texture_path(), "textures/playing_cards/card_diamond_10.png");

        let card = PlayingCard::Standard(Suite::Clubs, Rank::Jack);
        assert_eq!(card.front_texture_path(), "textures/playing_cards/card_club_11.png");
    }

    #[test]
    fn back_texture_path_format() {
        let card = PlayingCard::Standard(Suite::Hearts, Rank::Ace);
        assert_eq!(card.back_texture_path(), "textures/playing_cards/card_back.png");
    }
}
