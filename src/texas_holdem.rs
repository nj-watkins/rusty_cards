#[derive(Debug)]
pub type PlayerHand = (Card, Card);

pub type FiveHand = (Card, Card, Card, Card, Card);
pub type SevenHand = (Card, Card, Card, Card, Card, Card, Card)

#[derive(Debug)]
pub struct Community {
    pub flop: (Card, Card, Card),
    pub turn: Card,
    pub river: Card,
}


