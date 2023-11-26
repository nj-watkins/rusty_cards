mod deck; //Import the deck module
mod poker_orderings;
mod texas_holdem;

fn main() {
    let deck = deck::Deck::new();
    println!("{:?}", deck);
}
