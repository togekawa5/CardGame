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

pub fn front_texture_path(card: PlayingCard) -> String {
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

pub fn back_texture_path() -> String {
    format!("textures/playing_cards/card_back.png")
}