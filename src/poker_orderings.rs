mod deck;

#[derive(Debug, PartialEq, Eq)]
pub enum HandClass{
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

pub struct CardHash{
    rank_hash: HashMap<i8, i8>,
    suit_hash: HashMap<Suit, i8>
}

pub fn hash_cards(cards:Vec<Card>) -> CardHash {
    let mut rank_hash: HashMap<i8, i8>
    let mut suit_hash: HashMap<Suit, i8>
    for card in &cards:
        // iterate over each card in the cards vector
        *rank_hash.entry(card.rank.clone()).or_insert(0) += 1;
        *suit_hash.entry(card.suit.clone()).or_insert(0) += 1;
        // if the card's rank or suit doesn't exist in the HashMap already,
        // make a clone and set its corresponding counter to 0 (then iterate to 1)
    CardHash{rank_hash, suit_hash}
}

pub fn is_hash_flushable(card_hash: &CardHash) -> Result<bool, &'static str> {
    for (&key, &value) in &card_hash.suit_hash {
        // iterate over all key, value pairs, return true if there is a suit with more than 5
        if let deck::Suit::(_)= key {
        // if let is a special pattern matching capability in Rust
            if value >= 5: {
                return Ok(true)
            }
        else {
            Err{"The CardHash tried to pass off an invalid suit."}
        }
    }
    Ok(false)
}
