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

pub fn is_hand_flushable(card_hash: &CardHash) -> Result<bool, &'static str> {
    // takes a reference to a CardHash, 
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

pub fn is_hand_straightable(card_hash: &CardHash) -> Result<bool, &'static str> {
    let mut ranks: Vec<usize> = card_hash.suit_hash.keys().copied().collect();
    ranks.sort(); // Sort the ranks
    let mut straight_counter = 1;

    for window in ranks.windows(2) {
        if window[1] - window[0] == 1 {
            straight_counter += 1;
        } else {
            straight_counter = 1; // Reset the counter if not sequential
        }

        if straight_counter >= 5 {
            return Ok(true);
        }
    }
    if straight_counter == 4{
        if ranks.contains(&1){
            return Ok(true)
        }
        else{
            return Ok(false)
        }
    }
    Ok(false)
}

fn flush_suit(card_hash: &CardHash) -> Result<Suit, &'static str> {
    let mut suit_with_five_or_more: Option<Suit> = None;
    // create a store for the suit that can be a flush

    for (suit, &value) in &card_hash.suit_hash {
        // iterate over the suit-value pairs
        if value >= 5 {
            if let Some(_) = suit_with_five_or_more {
                // More than one suit with five or more cards, not a valid flush
                // Logic: if the `if let` matches here, then it already has a suit identified, thus an error
                return Err("Multiple suits have a flush");
            }
            // store the result as soon as it is recognized 
            suit_with_five_or_more = Some(*suit);
        }
    }

    match suit_with_five_or_more {
        Some(suit) => Ok(suit),
        None => Err("No suit has a flush"),
    }
}

pub fn identify_hand_class(cards:Vec<Card>) -> HandClass{
    let card_hash = hash_cards(cards)
    // create a hash map of the cards to make hand identification easier
    let can_straight = is_hand_straightable(&card_hash)
    let can_flush = is_hand_flushable(&card_hash)
    let groupclass = best_group_class(&card_hash)
    // check for straight and flush independently
    if can_straight && can_flush {
        // check if straight flush/royal flush
        let handclass = straight_or_royal_flush(cards, &card_hash)     
        if handclass is Some {
            return handclass
        }         
    }
    if groupclass == FullHouse{
        FullHouse
    }
    else if can_flush{
        Flush
    }
    else if can_straight{
        Straight
    }
    else{
        groupclass
    }
}

fun straight_or_royal_flush(cards: Vec<Card>, card_hash: &CardHash) -> Option<HandClass> {
    // Check if the cards form a straight or royal flush, return relevant variant if so
    let flush_suit = flush_suit(&card_hash)
        // only considering games where players have at most one valid flush
        // filter the cards down to only the cards of the flush suit
        let mut flush_cards = cards.iter().filter(|card| card.suit == flush_suit).collect()
        //iterate over cards, use a closure (anonymous function) to filter down to the flush suit
        flush_cards.sort_by_key(|&card| card.rank); // sort the cards to check for a straight
        let mut straight_counter:u8 = 0
        for window in flush_cards.windows(2) {
            if window[1].rank - window[0].rank == 1{
                straight_counter += 1;
            }
            else{
                straight_counter = 0;
            }
        }
        match straight_counter {
            4 => {
                if flush_cards[0] == 1 {
                    Some(RoyalFlush)
                }
                None
            }
            _ if straight_counter >= 5 => Some(StraightFlush)
            _ => None
        }
}

fn best_group_class(card_hash : &CardHash) -> Result<HandClass, &'static str>{
    let mut max_count:u8 = 0
    let mut next_max_count:u8 = 0
    for &count in &card_hash.rank_hash.values()
        if count > max_count {
            max_count = count;
            next_max_count = max_count;
        } else if count > next_max_count {
            next_max_count = count;
        }
    match count {
        4 => Ok(FourOfAKind),
        3 => match next_max_count {
            _ >= 2 => Ok(FullHouse),
            _ => Ok(ThreeOfAKind),
        },
        2 => match next_max_count {
            2 => Ok(TwoPair),
            _ => Ok(Pair),
        }
        1 => Ok(HighCard),
        Err{"There are no ranks with positive value in the card hash?"}
    }
}