use crate::*;
use std::fmt;

impl fmt::Display for Suit {
	#[rustfmt::skip]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let c = match self {
			Suit::Spades   => '♠',
			Suit::Hearts   => '♥',
			Suit::Clubs    => '♣',
			Suit::Diamonds => '♦',
		};
		write!(f, "{}", c)
	}
}

impl fmt::Display for Rank {
	#[rustfmt::skip]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let c = match self {
			Rank::Ace   => 'A',
			Rank::Two   => '2',
			Rank::Three => '3',
			Rank::Four  => '4',
			Rank::Five  => '5',
			Rank::Six   => '6',
			Rank::Seven => '7',
			Rank::Eight => '8',
			Rank::Nine  => '9',
			Rank::Ten   => 'T',
			Rank::Jack  => 'J',
			Rank::Queen => 'Q',
			Rank::King  => 'K',
		};
		write!(f, "{}", c)
	}
}

impl fmt::Display for Card {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if f.alternate() {
			match self.suit().color() {
				Red => write!(f, "\x1b[31;47m"),
				Black => write!(f, "\x1b[30;47m"),
			}?;
		}
		write!(f, "{}{}", self.suit(), self.rank())?;
		if f.alternate() {
			write!(f, "\x1b[m")?;
		}
		Ok(())
	}
}

impl fmt::Debug for Card {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?} of {:?}", self.rank(), self.suit())
	}
}

impl fmt::Debug for ClosedState {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut l = f.debug_list();
		for i in 0..7 {
			l.entry(&self.n_closed(i));
		}
		l.finish()
	}
}

impl fmt::Debug for Deck {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_list().entries(self.cards.iter()).finish()
	}
}

impl fmt::Debug for EncodedGameState {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:016x}{:032x}", self.0, self.1)
	}
}

impl fmt::Debug for Foundation {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut l = f.debug_list();
		for suit in Suit::all() {
			l.entry(&self.top_card(suit));
		}
		l.finish()
	}
}

impl fmt::Debug for Stack {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut l = f.debug_list();
		for i in 0..self.len() {
			l.entry(&self.get(i).unwrap());
		}
		l.finish()
	}
}

impl fmt::Debug for Stock {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		struct CardBitmask<'a>(&'a Stock);
		impl fmt::Debug for CardBitmask<'_> {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				write!(f, "{:024b}", self.0.state >> 8)
			}
		}
		f.debug_struct("Stock")
			.field("n_closed_cards", &self.n_closed_cards())
			.field("card_bitmask", &CardBitmask(self))
			.finish()
	}
}
