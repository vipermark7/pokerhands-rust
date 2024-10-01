use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Hash)]
enum Suit {
    Clubs,
    Hearts,
    Diamonds,
    Spades,
}

#[derive(Eq, PartialEq, Hash)]
enum Value {
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
    suit: Suit,
    value: Value,
}

type Hand = [Card; 5];


fn main() {
    println!("Hello, world!");
    let games = HashMap::from([
        ("game1", "2H 3D 5S 9C KD|2C 3H 4S 8C AH"),
        ("game2", "2H 4S 4C 2D 4H|2S 8S AS QS 3S"),
        ("game3", "2H 3D 5S 9C KD|2C 3H 4S 8C KH"),
        ("game4", "2H 3D 5S 9C KD|2D 3H 5C 9S KH")
    ]);
    let parsed = games.into_values()
        .map(|game| game.split_once("|"));
    let _ = dbg!(parsed);
}

/// return true of two of the cards have the same suit and same rank
fn two_of_a_kind(cards: Hand) -> bool {
    HashSet::from(cards).len() == 4
}
// High Card: Hands which do not fit any higher category
// are ranked by the value of their highest card. If the
// highest cards have the same value, the hands are ranked
// by the next highest, and so on.
// fn high_card(cards: Hand) -> bool {
//
// }
// Pair: 2 of the 5 cards in the hand have the same value.
// Hands which both contain a pair are ranked by the value of
// the cards forming the pair. If these values are the same,
// the hands are ranked by the values of the cards not forming
// the pair, in decreasing order.
// fn pair(cards: Hand) -> bool {
//     let values = cards.iter().map(|c| c.value);
//     HashSet::from(cards).len() == 3
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn two_of_a_kind_should_work() {
        let hand: Hand = [
            Card { suit: Suit::Clubs, value: Value::Ace },
            Card { suit: Suit::Clubs, value: Value::Ace },
            Card { suit: Suit::Diamonds, value: Value::Two },
            Card { suit: Suit::Hearts, value: Value::Two },
            Card { suit: Suit::Spades, value: Value::Ace }
        ];
        assert!(two_of_a_kind(hand))
    }

    #[test]
    fn two_of_a_kind_should_work() {
        let hand: Hand = [
            Card { suit: Suit::Clubs, value: Value::Ace },
            Card { suit: Suit::Clubs, value: Value::Ace },
            Card { suit: Suit::Diamonds, value: Value::Two },
            Card { suit: Suit::Hearts, value: Value::Two },
            Card { suit: Suit::Spades, value: Value::Ace }
        ];
        assert!(two_of_a_kind(hand))
    }
}
