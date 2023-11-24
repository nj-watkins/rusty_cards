use std::error::Error;
use std::fmt;
use rand::thread_rng;
use rand::seq::SliceRandom;
use itertools::iproduct;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug)]
struct BoundError(i8, i8, i8);

impl Error for BoundError {
    fn description(&self) -> &str {
        "Raised when value is out of known bounds."
    }
    fn cause(&self) -> Option<&dyn Error> {
        None // would include a derivative error here if it existed, but this is on the usSer/calling library.
    }
}

impl fmt::Display for BoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Value {} is out of bounds, which are [{} - {}]",
            self.0, self.1, self.2
        )
    }
}

#[derive(Debug, EnumIter)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

#[derive(Debug)]
pub struct Card {
    pub value: i8,
    pub suit: Suit,
}

impl Card{
    /// This function creates a new card with the value and suit specified, 
    /// and returns a BoundError if the value is not a valid integer.
    pub fn new(value: i8, suit:Suit) -> Result<Self, BoundError>{
        if value >= 1 && value <= 13 {
            Ok(Self{value, suit})
        } else {
            Err(BoundError(value, 1, 13))
        }
    }
}

#[derive(Debug)]
pub struct Discard {
    pub pile: Vec<Card>,
}

#[derive(Debug)]
pub struct Deck{
    pub shoe: Vec<Card>,
}

impl Deck{
    pub fn new() -> Self{
        // create a standard 52 card deck   
        let mut shoe: Vec<Card> = iproduct!(1..=13, Suit::iter())
            .map(|(v, s)| Card{value:v, suit:s})
            .collect();
            Self{shoe}
        }
    pub fn shuffle(mut self) -> Self{
        let mut random_number_generator = thread_rng();
        self.shoe.shuffle(&mut random_number_generator);
        // shuffle is a method provided by SliceRandom
        self
    }
}