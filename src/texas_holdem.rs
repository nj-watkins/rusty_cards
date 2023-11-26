mod deck;

use crate::deck::Card;
use crate::deck::CardCollector;
use crate::deck::implement_ref_tuple_collector;
use crate::deck::implement_tuple_collector;

pub type PlayerHand = (Card, Card);
implement_tuple_collector!(Card, Card);
implement_ref_tuple_collector!(Card, Card);
// This syntax is a result of the implement_tuple_collector macro expecting a tuple of type identifiers.
// Apparently, we *cannot* substitute the alias PlayerHand here.  But PlayerHand instances should have collect_cards.

#[derive(Debug)]
pub struct Community {
    pub flop: (Card, Card, Card),
    pub turn: Card,
    pub river: Card,
}

impl CardCollector for Community {
    fn collect_cards(&self) -> Vec<&Card> {
        let mut collected_cards = Vec::new();
        collected_cards.push(&self.flop.0);
        collected_cards.push(&self.flop.1);
        collected_cards.push(&self.flop.2);
        collected_cards.push(&self.turn);
        collected_cards.push(&self.river);
        // Add more fields...
        collected_cards
    }
}
