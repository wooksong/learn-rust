pub enum Card {
    King,
    Queen,
    Jack,
    Numbered(u32, String),
}

pub fn card_description(card: &Card) -> String {
    match card {
        Card::King => "King".to_string(),
        Card::Queen => "Queen".to_string(),
        Card::Jack => "Jack".to_string(),
        Card::Numbered(value, suit) => format!("{value} of {suit}")
    }
}
