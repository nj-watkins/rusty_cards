use rand::thread_rng;
use rand::seq::SliceRandom;
use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

#[derive(Debug)]
pub struct Card {
    value: i8,
    suit: Suit,
}

pub impl Card{
    /// This function creates a new card with the value and suit specified, 
    /// and returns a BoundError if the value is not a valid integer.
    pub fn new(value: i8, suit:Suit) -> Result<Self, BoundError>{
        if value >= 1 && value <= 13 {
            Ok(Self{value, suit});
        } else {
            Err(BoundError, value, 1, 13);
        }
    }
}

#[derive(Debug)]
pub struct Discard {
    pub pile: std::Vec<Card>,
}

#[derive(Debug)]
pub struct Deck{
    pub shoe: std::Vec<Card>,
}

#[derive(Debug)]
pub impl Deck{
    pub fn new() -> Self{
        // create a standard 52 card deck   
        let mut shoe: Vec<Card> = iproduct!(1..=13, Suit::iter())
            .map(|(v, s)| Card{value:v, suit:s})
            .collect();

            Self {shoe}
        }
    pub fn shuffle(&mut self) -> Self{
        let mut random_number_generator = thread_rng();
        self.shoe.shuffle(&mut random_number_generator);
        // shuffle is a method provided by SliceRandom
    }
}