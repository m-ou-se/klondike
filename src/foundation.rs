use intbits::Bits;
use crate::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Foundation {
	state: u16,
}

impl Foundation {
	pub const fn new() -> Self {
		Self { state: 0 }
	}

	pub const fn is_complete(&self) -> bool {
		self.state == 0xDDDD
	}

	pub fn add(&mut self, card: Card) -> Result<(), ()> {
		if card.rank().num() == self.n_cards(card.suit()) + 1 {
			self.state += 1 << card.suit().num() * 4;
			Ok(())
		} else {
			Err(())
		}
	}

	pub fn top_card(&self, suit: Suit) -> Option<Card> {
		match self.n_cards(suit) {
			0 => None,
			x => Some(Rank::from_num(x).of(suit)),
		}
	}

	pub fn contains(&self, card: Card) -> bool {
		card.rank().num() <= self.n_cards(card.suit())
	}

	pub fn to_bits(&self) -> u16 {
		self.state
	}

	pub fn from_bits(bits: u16) -> Self {
		Self { state: bits }
	}

	pub fn n_cards(&self, suit: Suit) -> usize {
		self.state.bits(suit.num() * 4..).bits(..4).into()
	}
}

#[test]
fn test() {
	let mut f = Foundation::new();
	assert_eq!(f.top_card(Spades), None);
	assert_eq!(f.top_card(Hearts), None);
	assert_eq!(f.top_card(Clubs), None);
	assert_eq!(f.top_card(Diamonds), None);
	assert!(!f.contains(Ace.of(Hearts)));
	assert!(f.add(Ace.of(Hearts)).is_ok());
	assert!(f.contains(Ace.of(Hearts)));
	assert!(!f.contains(Two.of(Hearts)));
	assert_eq!(f.top_card(Spades), None);
	assert_eq!(f.top_card(Hearts), Some(Ace.of(Hearts)));
	assert_eq!(f.top_card(Clubs), None);
	assert_eq!(f.top_card(Diamonds), None);
	assert!(f.add(Ace.of(Hearts)).is_err());
	assert!(f.add(Three.of(Hearts)).is_err());
	assert!(f.add(Two.of(Hearts)).is_ok());
	assert!(f.add(Two.of(Spades)).is_err());
	assert!(f.add(Ace.of(Spades)).is_ok());
	assert_eq!(f.top_card(Spades), Some(Ace.of(Spades)));
	assert_eq!(f.top_card(Hearts), Some(Two.of(Hearts)));
	assert_eq!(f.top_card(Clubs), None);
	assert_eq!(f.top_card(Diamonds), None);
	assert!(f.contains(Two.of(Hearts)));
	assert!(!f.contains(Three.of(Hearts)));
	for rank in Rank::all() {
		let card = rank.of(Diamonds);
		assert!(!f.contains(card));
		assert!(f.add(card).is_ok());
		assert!(f.add(card).is_err());
		assert_eq!(f.top_card(Diamonds), Some(card));
		assert!(f.contains(card));
	}
}
