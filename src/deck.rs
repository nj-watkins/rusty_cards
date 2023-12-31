use std::error::Error;
use std::fmt;
use rand::thread_rng;
use rand::seq::SliceRandom;
use itertools::iproduct;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug)]
pub struct BoundError(i8, i8, i8);

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

#[derive(Debug, Clone, Copy, EnumIter, Hash, PartialEq, Eq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug)]
pub struct Card {
    pub rank: i8,
    pub suit: Suit,
}

impl Card{
    /// This function creates a new card with the rank and suit specified, 
    /// and returns a BoundError if the rank is not a valid integer.
    pub fn new(rank: i8, suit:Suit) -> Result<Self, BoundError>{
        if rank >= 1 && rank <= 13 {
            Ok(Self{rank, suit})
        } else {
            Err(BoundError(rank, 1, 13))
        }
    }
}

impl AsRef<Card> for Card {
    fn as_ref(&self) -> &Card {
        self
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
        let shoe: Vec<Card> = iproduct!(1..=13, Suit::iter()) //iterating over Cartesian product
            .map(|(v, s)| Card{rank:v, suit:s}) //calling the Card constructor on each pair of rank and suit
            .collect(); //collecting the individual Cards into a Vec, for the Deck constructor
            Self{shoe}
        }
    pub fn shuffle(mut self) -> Self{
        let mut random_number_generator = thread_rng();
        self.shoe.shuffle(&mut random_number_generator);
        // shuffle is a method provided by SliceRandom
        self
    }
}

pub trait CardCollector {
    fn collect_cards(&self) -> Vec<&Card>;
}

// Implement the trait for tuples of cards with any length (up to a limit)
#[macro_export]
macro_rules! implement_tuple_collector {
    ($($n:ident),*) => {
        pub impl<$($n),*> CardCollector for ($($n),*)
        where
            $($n: AsRef<Card>),*
        {
            fn collect_cards(&self) -> Vec<&Card> {
                let mut collected_cards = Vec::new();
                $(collected_cards.push(self.$n.as_ref());)*
                collected_cards
            }
        }
    };
}

// #[macro_export]
// macro_rules! implement_ref_tuple_collector {
//     ($($n:ident),*) => {
//         impl<$($n),*> CardCollector for &($( $n, )*)
//         where
//             $( $n: AsRef<Card>, )*
//         {
//             fn collect_cards(&self) -> Vec<&Card> {
//                 let mut collected_cards = Vec::new();
//                 $(collected_cards.push(self.$n.as_ref());)*
//                 collected_cards
//             }
//         }
//     };
// }

// #[macro_export]
// macro_rules! implement_ref_tuple_collector {
//     ($($n:ident),*) => {
//         impl<$($n),*> CardCollector for &($( $n, )*)
//         where
//             $( $n: AsRef<crate::deck::Card>, )*
//         {
//             fn collect_cards(&self) -> Vec<&crate::deck::Card> {
//                 let mut collected_cards = Vec::new();
//                 $(collected_cards.push(self.$n.as_ref());)*
//                 collected_cards
//             }
//         }
//     };
// }

#[macro_export]
macro_rules! implement_ref_tuple_collector {
    ($($T:ident),*) => {
        impl<$($T),*> CardCollector for &($( $T, )*)
        where
            $( $T: AsRef<crate::deck::Card>, )*
        {
            fn collect_cards(&self) -> Vec<&crate::deck::Card> {
                let mut collected_cards = Vec::new();
                $(collected_cards.push(self.$T.as_ref());)*
                collected_cards
            }
        }
    };
}

// macro_rules! implement_ref_tuple_collector {
//     ($($n:ident),*) => {
//         pub impl<'a, $($n),*> CardCollector for (&'a $($n),*)
//         where
//             $($n: AsRef<Card>),*
//         {
//             fn collect_cards(&self) -> Vec<&Card> {
//                 let mut collected_cards = Vec::new();
//                 $(collected_cards.push(self.$n.as_ref());)*
//                 collected_cards
//             }
//         }
//     };
// }