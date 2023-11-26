mod deck; //Import the deck module
mod texas_holdem;
mod poker_orderings;

fn main() {
    let deck = deck::Deck::new();
    println!("{:?}", deck);
}
