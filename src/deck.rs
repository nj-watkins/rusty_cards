use rand::thread_rng;
use rand::seq::SliceRandom;
use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
enum Suit{
    Heart,
    Diamond,
    Club,
    Spade,
}
struct Card {
    value: i8,
    suit: Suit,
}

struct PlayerHand {
    HoleOne: Card,
    HoleTwo: Card,
}

impl Card{
    /// This function creates a new card with the value and suit specified, 
    /// and returns a BoundError if the value is not a valid integer.
    pub const fn new(value: i8, suit:Suit) -> Result<Self, BoundError>{
        if value >= 1 && value <= 13 {
            Ok(Self(value, suit))
        } else {
            Err(BoundError, value, 1, 13)
        }
    }
}

struct Community {
    flop: std::Vec<Card>,
    turn: Card,
    river: Card,
}

struct Discard {
    pile: std::Vec<Card>
}

struct Deck{
    shoe: std::Vec<Card>
}

impl Deck{
    const fn new() -> Self{
        // create a standard 52 card deck   
        let mut shoe: Vec<Card> = iproduct!(1..14, Suit::iter())
            .map(|(s, v)| Card{value:v, suit:s})
            .collect();
    }
    const fn shuffle() -> Self{
        shoe.shuffle(&mut thread_rng());
    }
}