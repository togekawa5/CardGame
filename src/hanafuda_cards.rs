use crate::card_token::CardToken;

enum HanafudaMonth {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

enum HanafudaCardType {
    Bright,
    Animal,
    Ribbon,
    Chaff,
}

pub struct HanafudaCard {
    month: HanafudaMonth,
    card_type: HanafudaCardType,
}

impl CardToken for HanafudaCard {
    fn size(&self) -> (f32, f32) {
        (0.335, 0.54) // Example size in some units
    }

    fn flip(&mut self) {
        // Implementation for flipping the card
    }

    fn is_face_up(&self) -> bool {
        // Implementation to check if the card is face up
        true
    }

    fn display(&self) -> String {
        format!("{:?} - {:?}", self.month, self.card_type)
    }

    fn front_texture_path(&self) -> String {
        format!("textures/hanafuda/{:?}_{:?}_front.png", self.month, self.card_type)
    }

    fn back_texture_path(&self) -> String {
        "textures/hanafuda/card_back.png".to_string()
    }
}


