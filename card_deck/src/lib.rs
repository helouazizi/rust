use rand::prelude::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
#[derive(Debug, Clone, Copy,PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club
}
#[derive(Debug, Clone, Copy,PartialEq)]
pub enum Rank {
    Ace, King, Queen ,Jack
}

impl Suit {
    pub fn random() -> Suit {
        let   swits = [Suit::Heart, Suit::Spade,Suit::Club,Suit::Diamond];
        let indx = rand::thread_rng().gen_range(0..swits.len());
        swits[indx]
    }

    pub fn translate(value: u8) -> Suit {
            match value {
            0 => Suit::Heart,
            1=> Suit::Diamond,
            2 => Suit::Spade,
            3 => Suit::Club,
            _ => panic!("Invalid value for Rank: {}", value), // or handle gracefully
       }
    }
}

impl Rank {
    pub fn random() -> Rank {

        let ranks = [Rank::Ace, Rank::Jack,Rank::King,Rank::Queen];
        let indx = rand::thread_rng().gen_range(0..ranks.len());
        ranks[indx]
    }

    pub fn translate(value: u8) -> Rank {
       match value {
         0 => Rank::Ace,
         1=> Rank::King,
         2 => Rank::Jack,
         3 => Rank::Queen,
        _ => panic!("Invalid value for Rank: {}", value), // or handle gracefully
       }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {

    card.rank == Rank::Ace && card.suit == Suit::Spade

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
