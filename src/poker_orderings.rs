//mod deck;
//mod texas_holdem;

use std::collections::HashMap;
use crate::texas_holdem::Community;
use crate::texas_holdem::PlayerHand;
use crate::deck::Card;
use crate::deck::Suit;
use crate::deck::CardCollector;

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
    suit_hash: HashMap<Suit, i8>,
}

pub fn hash_cards(cards:&Vec<&Card>) -> CardHash {
    let mut rank_hash: HashMap<i8, i8> = Default::default();
    let mut suit_hash: HashMap<Suit, i8> = Default::default();
    for card in cards{
        // iterate over each card in the cards vector
        *rank_hash.entry(card.rank.clone()).or_insert(0) += 1;
        *suit_hash.entry(card.suit.clone()).or_insert(0) += 1;
        // if the card's rank or suit doesn't exist in the HashMap already,
        // make a clone and set its corresponding counter to 0 (then iterate to 1)
    }
    CardHash{rank_hash, suit_hash}
}

pub fn is_hand_flushable(card_hash: &CardHash) -> Result<bool, &'static str> {
    let mut has_flush_suit = false;

    for (&key, &value) in &card_hash.suit_hash {
        // iterate over all key-value pairs, set has_flush_suit if there is a suit with more than 5
        match key {
            // TODO: complain about this syntax on a Rust forum?
            crate::deck::Suit::Hearts | crate::deck::Suit::Diamonds | crate::deck::Suit::Clubs | crate::deck::Suit::Spades => {
                if value >= 5 {
                    has_flush_suit = true;
                    break;
                } else {
                    has_flush_suit = false;
                }
            }
            _ => return Err("The card hash had an unaccounted for suit type"),
        }
    }
    Ok(has_flush_suit)
}

pub fn is_hand_straightable(card_hash: &CardHash) -> Result<bool, &'static str> {
    let mut ranks: Vec<i8> = card_hash.rank_hash.keys()
                                                .copied()
                                                .collect();
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

pub fn identify_hand_class(cards:Vec<&Card>) -> Result<HandClass, &'static str>{
    let card_hash = hash_cards(&cards);
    // create a hash map of the cards to make hand identification easier
    let can_straight = is_hand_straightable(&card_hash);
    let can_flush = is_hand_flushable(&card_hash);
    let groupclass = best_group_class(&card_hash);
    // check for straight and flush independently
    let can_flush = match can_flush{
        Ok(value) => value,
        Err(err) => {
            println!("Error occurred: {}", err);
            return Err("Something errant!");
        }

    };
    let can_straight = match can_straight{
        Ok(value) => value,
        Err(err) => {
            println!("Error occurred: {}", err);
            return Err("Something errant!");
        }
    };  
    if can_straight && can_flush {
        // check if straight flush/royal flush
        let handclass = straight_or_royal_flush(cards, &card_hash);     
        if let Some(hand) = handclass  {

            return Ok(hand);
        }         
    }
    let groupclass = match groupclass{
        Ok(value) => value,
        Err(err) => {
            println!("Error occurred: {}", err);
            return Err("Something errant!");
        }
    };
    if groupclass == HandClass::FullHouse{
        return Ok(HandClass::FullHouse);
    }
    else if can_flush {
        return Ok(HandClass::Flush);
    }
    else if can_straight {
        return Ok(HandClass::Straight);
    }
    else{
        return Ok(groupclass);
    }
}

fn straight_or_royal_flush(cards: Vec<&Card>, card_hash: &CardHash) -> Option<HandClass> {
    // Check if the cards form a straight or royal flush, return relevant variant if so
    let flush_suit = flush_suit(&card_hash);
    let flush_suit = match flush_suit {
        Ok(value) => value,
        Err(err) => {
            println!("An error has occured: {}", err);
            return None;
        }
    };    
    // only considering games where players have at most one valid flush
    // filter the cards down to only the cards of the flush suit
    let mut flush_cards: Vec<&Card> = cards.iter()
                                           .filter(|&&card| card.suit == flush_suit)
                                           .copied()
                                           .collect();
    //iterate over cards, use a closure (anonymous function) to filter down to the flush suit
    flush_cards.sort_by_key(|&card| card.rank); // sort the cards to check for a straight
    let mut straight_counter:u8 = 1;
    for window in flush_cards.windows(2) {
        if window[1].rank - window[0].rank == 1{
            // if the difference between the lower rank card and the next 
            // rank card is exactly 1, increment the straight counter
            straight_counter += 1;
        }
        else{
            // if the difference is not exactly one, reset the straight counter
            straight_counter = 0;
        }
    }
    match straight_counter {
        4 => {
            // if the straight_counter ended at exactly four and the lowest rank card is an Ace,
            // then royal flush...
            if flush_cards[0].rank == 1 {
                return Some(HandClass::RoyalFlush);
            }
            None
        },
        _ => Some(HandClass::StraightFlush),
    }
}

fn best_group_class(card_hash : &CardHash) -> Result<HandClass, &'static str>{
    let mut max_count:i8 = 0;
    let mut next_max_count:i8 = 0;
    // ^-- initialize two counters to assess the maximum card counts considered over ranks
    for &count in card_hash.rank_hash.values() {
        if count >= max_count {
            // if the count is larger than the max count,
            // move the current max count into the next max count 
            // move the count into max count.
            next_max_count = max_count;
            max_count = count;
        } else if count >= next_max_count {
            next_max_count = count;
        }
    }
    match max_count {
        4 => Ok(HandClass::FourOfAKind),
        3 => match next_max_count {
            // if the max count is 3, we have to cases
            2..=std::i8::MAX => Ok(HandClass::FullHouse),
            // if next max count is 2, we have a Full House
            _ => Ok(HandClass::ThreeOfAKind),
        },
        2 => match next_max_count {
            2 => Ok(HandClass::TwoPair),
            _ => Ok(HandClass::Pair),
        }
        1 => Ok(HandClass::HighCard),
        _ => Err("There are no ranks with positive value in the card hash?")
    }
}

// fn create_hand_vector(player_hand: & PlayerHand, community: &Community) -> Vec<Card>{
//     let mut hand_vector: Vec<Card> = vec![];
//     hand_vector.extend(player_hand.collect_cards());
//     hand_vector.extend(community.collect_cards());
//     hand_vector
// }

fn create_hand_vector<'a>(
    player_hand: &'a PlayerHand,
    community: &'a Community,
) -> Vec<&'a Card> {
    let mut collector: Vec<&Card> = Vec::new();
    collector.extend(player_hand.collect_cards());
    collector.extend(community.collect_cards());
    collector
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::Suit::*;
    #[test]
    fn test_card_hash() {
        let ace_hearts = Card{rank:1, suit: Hearts};
        let two_hearts = Card{rank:2, suit: Hearts};
        let two_diamonds = Card{rank:2, suit: Diamonds};
        let three_hearts = Card{rank:3, suit: Hearts};
        let four_hearts = Card{rank:4, suit: Hearts};
        let five_hearts = Card{rank:5, suit: Hearts};
        let five_clubs = Card{rank:5, suit: Clubs};
        let six_clubs = Card{rank:6, suit:Clubs};
        let four_clubs = Card{rank:4, suit: Clubs};
        let four_diamonds = Card{rank:4, suit: Diamonds};
        let four_spades = Card{rank:4, suit: Spades};
        let ace_spades = Card{rank:1, suit: Spades};
        // ^-- Build cards for testing hand id
        let test_community_one = Community{
            flop: (ace_hearts, ace_spades, two_hearts),
            turn: three_hearts,
            river: four_hearts,
        };
        let test_hand_one = (five_hearts, four_spades);
        let test_hand_oneone = create_hand_vector(&test_hand_one, &test_community_one);
        let test_handclass_oneone = identify_hand_class(test_hand_oneone);
        assert_eq!(test_handclass_oneone, Ok(HandClass::StraightFlush));

        let test_hand_two = (four_clubs, four_diamonds);
        let test_hand_twoone = create_hand_vector(&test_hand_two, &test_community_one);
        let test_handclass_twoone = identify_hand_class(test_hand_twoone);
        assert_eq!(test_handclass_twoone, Ok(HandClass::FullHouse));

        let test_hand_three = (two_diamonds, six_clubs);
        let test_hand_threeone = create_hand_vector(&test_hand_three, &test_community_one);
        let test_handclass_threeone = identify_hand_class(test_hand_threeone);
        assert_eq!(test_handclass_threeone, Ok(HandClass::TwoPair));
    }
}