

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
    pub const fn new(value: i8, suit:Suit) -> Resukt<Self, BoundError>{
        if value >= 1 && value <= 13 {
            Ok(Self(value, suit))
        } else {
            Err(BoundError, value, 1, 13)
        }
    }
}

struct Community {
    flop: list(Card, Card, Card),
    turn: Card,
    river: Card,
}

struct Discard {
    pile: (Card)
}

struct Deck{

}

