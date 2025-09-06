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
    Ace, King, Queen ,Jack , Number(u8)
}

impl Suit {
    pub fn random() -> Suit {
        let indx = rand::thread_rng().gen_range(0..4);
        Suit::translate(indx)
    }

    pub fn translate(value: u8) -> Suit {
            match value {
            0 => Suit::Heart,
            1=> Suit::Diamond,
            2 => Suit::Spade,
            _ => Suit::Club,
       }
    }
}

impl Rank {
    pub fn random() -> Rank {

        let indx = rand::thread_rng().gen_range(1..14);
        Rank::translate(indx)
    }

    pub fn translate(value: u8) -> Rank {
       match value {
         1 => Rank::Ace,
         13=> Rank::King,
         11 => Rank::Jack,
         12 => Rank::Queen,
         _ => Rank::Number(value)
       }
    }
}
#[derive(Debug, Clone, Copy,PartialEq)]
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
