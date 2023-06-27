use rand::Rng;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8)
}

impl Suit {
	pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        let random_nr: u8 = rng.gen_range(1, 5);

        Self::translate(random_nr)
	}

	pub fn translate(value: u8) -> Suit {

        match value {
            1 => Suit::Heart,
            2 => Suit::Spade,
            3 => Suit::Diamond,
            4 => Suit::Club,
            _ => panic!()
        }
	}
}

impl Rank {
	pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        let random_nr: u8 = rng.gen_range(1, 13);

        Self::translate(random_nr)
	}

	pub fn translate(value: u8) -> Rank {

        match value {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            2..=10 => Rank::Number(value),
            _ => panic!()
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct Card {
	pub suit: Suit,
	pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool{
    if card.suit == Suit::Spade && card.rank == Rank::Ace {
        return true
    }
    false
}
