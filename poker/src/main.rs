use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash)]
enum Suit {
    Clubs,
    Hearts,
    Diamonds,
    Spades,
}

#[derive(Eq, PartialEq, Hash)]
enum Rank {
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

#[derive(Eq, PartialEq, Hash)]
struct Card {
    // repr of a Card with a suit and rank
    suit: Suit,
    value: Rank,
}

type Hand = [Card; 5];

fn main() {
    println!("Hello, world!");
}

/// return true of two of the cards have the same suit and same rank
fn two_of_a_kind(cards: Hand) -> bool {
    HashSet::from(cards).len() == 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_of_a_kind_should_work() {
        let hand: Hand = [
            Card { suit: Suit::Clubs, value: Rank::Ace },
            Card { suit: Suit::Clubs, value: Rank::Ace },
            Card { suit: Suit::Diamonds, value: Rank::Two },
            Card { suit: Suit::Hearts, value: Rank::Two },
            Card { suit: Suit::Spades, value: Rank::Ace }
        ];
        assert!(two_of_a_kind(hand))
    }
}
