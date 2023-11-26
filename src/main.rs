mod deck; //Import the deck module
mod poker_orderings;

fn main() {
    let deck = deck::Deck::new();
    println!("{:?}", deck);
}
