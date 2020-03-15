use crate::*;
use std::num::NonZeroU8;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Card {
	n: NonZeroU8,
}

impl Rank {
	pub const fn of(self, suit: Suit) -> Card {
		Card {
			n: unsafe { NonZeroU8::new_unchecked(self.num() as u8 + ((suit.num() as u8) << 4)) },
		}
	}
}

impl Card {
	pub fn all() -> impl Iterator<Item = Self> {
		Suit::all().flat_map(|s| Rank::all().map(move |r| r.of(s)))
	}

	pub fn suit(self) -> Suit {
		Suit::from_num(usize::from(self.n.get() >> 4))
	}

	pub fn rank(self) -> Rank {
		Rank::from_num(usize::from(self.n.get() & 0xF))
	}

	pub fn to_bits(self, two_extra_bits: u8) -> u8 {
		Self::option_to_bits(Some(self), two_extra_bits)
	}

	pub fn from_bits(bits: u8) -> (Self, u8) {
		let (card, extra) = Self::option_from_bits(bits);
		(card.unwrap(), extra)
	}

	pub fn option_to_bits(card: Option<Self>, two_extra_bits: u8) -> u8 {
		assert!(two_extra_bits < 4);
		card.map_or(0, |c| c.n.get()) | two_extra_bits << 6
	}

	pub fn option_from_bits(bits: u8) -> (Option<Self>, u8) {
		(NonZeroU8::new(bits & 0x3F).map(|n| Self { n }), bits >> 6)
	}
}

#[test]
fn test() {
	assert_eq!(Card::all().count(), 52);
	for (i, card) in Card::all().enumerate() {
		let suit = Suit::from_num(i / 13);
		let rank = Rank::from_num(i % 13 + 1);
		assert_eq!(card.suit(), suit);
		assert_eq!(card.rank(), rank);
		let extra = i as u8 % 4;
		let bits = card.to_bits(extra);
		assert_eq!(Card::option_to_bits(Some(card), extra), bits);
		assert_eq!(Card::from_bits(bits), (card, extra));
		assert_eq!(Card::option_from_bits(bits), (Some(card), extra));
	}
	assert_eq!(Card::option_to_bits(None, 0), 0);
	assert_eq!(Card::option_from_bits(0), (None, 0));
}
