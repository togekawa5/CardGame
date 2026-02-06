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



