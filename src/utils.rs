#[derive(Debug, Clone)]
pub struct PokerCard {
    pub value: u8,
    pub suit: String,
    pub filename: String,
}

pub fn load_poker_deck() -> Vec<PokerCard> {
    let mut deck: Vec<PokerCard> = vec![];

    // Clubs
    deck.push(PokerCard {
        value: 1,
        suit: "Clubs".to_string(),
        filename: "card-clubs-1.png".to_string(),
    });
    deck.push(PokerCard {
        value: 2,
        suit: "Clubs".to_string(),
        filename: "card-clubs-2.png".to_string(),
    });
    deck.push(PokerCard {
        value: 3,
        suit: "Clubs".to_string(),
        filename: "card-clubs-3.png".to_string(),
    });
    deck.push(PokerCard {
        value: 4,
        suit: "Clubs".to_string(),
        filename: "card-clubs-4.png".to_string(),
    });
    deck.push(PokerCard {
        value: 5,
        suit: "Clubs".to_string(),
        filename: "card-clubs-5.png".to_string(),
    });
    deck.push(PokerCard {
        value: 6,
        suit: "Clubs".to_string(),
        filename: "card-clubs-6.png".to_string(),
    });
    deck.push(PokerCard {
        value: 7,
        suit: "Clubs".to_string(),
        filename: "card-clubs-7.png".to_string(),
    });
    deck.push(PokerCard {
        value: 8,
        suit: "Clubs".to_string(),
        filename: "card-clubs-8.png".to_string(),
    });
    deck.push(PokerCard {
        value: 9,
        suit: "Clubs".to_string(),
        filename: "card-clubs-9.png".to_string(),
    });
    deck.push(PokerCard {
        value: 10,
        suit: "Clubs".to_string(),
        filename: "card-clubs-10.png".to_string(),
    });
    deck.push(PokerCard {
        value: 11,
        suit: "Clubs".to_string(),
        filename: "card-clubs-11.png".to_string(),
    });
    deck.push(PokerCard {
        value: 12,
        suit: "Clubs".to_string(),
        filename: "card-clubs-12.png".to_string(),
    });
    deck.push(PokerCard {
        value: 13,
        suit: "Clubs".to_string(),
        filename: "card-clubs-13.png".to_string(),
    });
    // Diamonds
    deck.push(PokerCard {
        value: 1,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-1.png".to_string(),
    });
    deck.push(PokerCard {
        value: 2,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-2.png".to_string(),
    });
    deck.push(PokerCard {
        value: 3,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-3.png".to_string(),
    });
    deck.push(PokerCard {
        value: 4,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-4.png".to_string(),
    });
    deck.push(PokerCard {
        value: 5,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-5.png".to_string(),
    });
    deck.push(PokerCard {
        value: 6,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-6.png".to_string(),
    });
    deck.push(PokerCard {
        value: 7,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-7.png".to_string(),
    });
    deck.push(PokerCard {
        value: 8,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-8.png".to_string(),
    });
    deck.push(PokerCard {
        value: 9,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-9.png".to_string(),
    });
    deck.push(PokerCard {
        value: 10,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-10.png".to_string(),
    });
    deck.push(PokerCard {
        value: 11,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-11.png".to_string(),
    });
    deck.push(PokerCard {
        value: 12,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-12.png".to_string(),
    });
    deck.push(PokerCard {
        value: 13,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-13.png".to_string(),
    });
    // Hearts
    deck.push(PokerCard {
        value: 1,
        suit: "Hearts".to_string(),
        filename: "card-hearts-1.png".to_string(),
    });
    deck.push(PokerCard {
        value: 2,
        suit: "Hearts".to_string(),
        filename: "card-hearts-2.png".to_string(),
    });
    deck.push(PokerCard {
        value: 3,
        suit: "Hearts".to_string(),
        filename: "card-hearts-3.png".to_string(),
    });
    deck.push(PokerCard {
        value: 4,
        suit: "Hearts".to_string(),
        filename: "card-hearts-4.png".to_string(),
    });
    deck.push(PokerCard {
        value: 5,
        suit: "Hearts".to_string(),
        filename: "card-hearts-5.png".to_string(),
    });
    deck.push(PokerCard {
        value: 6,
        suit: "Hearts".to_string(),
        filename: "card-hearts-6.png".to_string(),
    });
    deck.push(PokerCard {
        value: 7,
        suit: "Hearts".to_string(),
        filename: "card-hearts-7.png".to_string(),
    });
    deck.push(PokerCard {
        value: 8,
        suit: "Hearts".to_string(),
        filename: "card-hearts-8.png".to_string(),
    });
    deck.push(PokerCard {
        value: 9,
        suit: "Hearts".to_string(),
        filename: "card-hearts-9.png".to_string(),
    });
    deck.push(PokerCard {
        value: 10,
        suit: "Hearts".to_string(),
        filename: "card-hearts-10.png".to_string(),
    });
    deck.push(PokerCard {
        value: 11,
        suit: "Hearts".to_string(),
        filename: "card-hearts-11.png".to_string(),
    });
    deck.push(PokerCard {
        value: 12,
        suit: "Hearts".to_string(),
        filename: "card-hearts-12.png".to_string(),
    });
    deck.push(PokerCard {
        value: 13,
        suit: "Hearts".to_string(),
        filename: "card-hearts-13.png".to_string(),
    });
    // Spades
    deck.push(PokerCard {
        value: 1,
        suit: "Spades".to_string(),
        filename: "card-spades-1.png".to_string(),
    });
    deck.push(PokerCard {
        value: 2,
        suit: "Spades".to_string(),
        filename: "card-spades-2.png".to_string(),
    });
    deck.push(PokerCard {
        value: 3,
        suit: "Spades".to_string(),
        filename: "card-spades-3.png".to_string(),
    });
    deck.push(PokerCard {
        value: 4,
        suit: "Spades".to_string(),
        filename: "card-spades-4.png".to_string(),
    });
    deck.push(PokerCard {
        value: 5,
        suit: "Spades".to_string(),
        filename: "card-spades-5.png".to_string(),
    });
    deck.push(PokerCard {
        value: 6,
        suit: "Spades".to_string(),
        filename: "card-spades-6.png".to_string(),
    });
    deck.push(PokerCard {
        value: 7,
        suit: "Spades".to_string(),
        filename: "card-spades-7.png".to_string(),
    });
    deck.push(PokerCard {
        value: 8,
        suit: "Spades".to_string(),
        filename: "card-spades-8.png".to_string(),
    });
    deck.push(PokerCard {
        value: 9,
        suit: "Spades".to_string(),
        filename: "card-spades-9.png".to_string(),
    });
    deck.push(PokerCard {
        value: 10,
        suit: "Spades".to_string(),
        filename: "card-spades-10.png".to_string(),
    });
    deck.push(PokerCard {
        value: 11,
        suit: "Spades".to_string(),
        filename: "card-spades-11.png".to_string(),
    });
    deck.push(PokerCard {
        value: 12,
        suit: "Spades".to_string(),
        filename: "card-spades-12.png".to_string(),
    });
    deck.push(PokerCard {
        value: 13,
        suit: "Spades".to_string(),
        filename: "card-spades-13.png".to_string(),
    });

    deck
}
