use crate::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Stack {
	base: Option<Card>,
	next: u16,
}

impl Stack {
	pub const fn new() -> Self {
		Stack {
			base: None,
			next: 0,
		}
	}

	pub const fn single(card: Card) -> Self {
		Stack {
			base: Some(card),
			next: 1,
		}
	}

	pub fn len(&self) -> usize {
		usize::from(self.next & 0xF)
	}

	pub fn get(&self, i: usize) -> Option<Card> {
		if i == 0 {
			self.base
		} else if i < self.len() {
			let base = self.base.unwrap();
			let color = if i & 1 == 0 {
				base.suit().color()
			} else {
				base.suit().color().opposite()
			};
			let variant = Variant::from_bit(self.next >> i + 3 & 1 == 1);
			let suit = Suit::from_variant(color, variant);
			let rank = Rank::from_num(base.rank().num() - i);
			Some(rank.of(suit))
		} else {
			None
		}
	}

	pub fn last(&self) -> Option<Card> {
		self.get(self.len().checked_sub(1)?)
	}

	// TODO: Don't accept cards for an empty stack.
	pub fn add(&mut self, card: Card) -> Result<(), ()> {
		if let Some(last) = self.last() {
			if last.suit().color() != card.suit().color()
				&& last.rank().num() - 1 == card.rank().num()
			{
				if card.suit().variant().as_bit() {
					self.next |= 1 << self.len() + 3;
				}
				self.next += 1;
				Ok(())
			} else {
				Err(())
			}
		} else if card.rank() == King {
			self.base = Some(card);
			self.next = 1;
			Ok(())
		} else {
			Err(())
		}
	}

	pub fn take(&mut self) -> Result<Card, ()> {
		if let Some(card) = self.last() {
			self.next -= 1;
			if self.len() == 0 {
				self.base = None;
			} else {
				self.next &= !(1 << self.len() + 3)
			}
			Ok(card)
		} else {
			Err(())
		}
	}

	pub fn contains(&self, card: Card) -> bool {
		self.base
			.and_then(|base| base.rank().num().checked_sub(card.rank().num()))
			.and_then(|offset| self.get(offset))
			== Some(card)
	}

	pub fn to_bits(&self, two_extra_bits: u8) -> (u8, u16) {
		(Card::option_to_bits(self.base, two_extra_bits), self.next)
	}

	pub fn from_bits(bits: (u8, u16)) -> (Self, u8) {
		let (base, extra) = Card::option_from_bits(bits.0);
		(Self { base, next: bits.1 }, extra)
	}
}

#[test]
fn test_single() {
	let mut stack = Stack::new();
	assert!(!stack.contains(Nine.of(Hearts)));
	assert_eq!(stack.len(), 0);
	assert_eq!(stack.to_bits(0), (0, 0));
	assert_eq!(Stack::from_bits((0, 0)), (Stack::new(), 0));
	assert_eq!(stack.take(), Err(()));
	assert!(stack.add(Nine.of(Hearts)).is_ok());
	assert!(stack.contains(Nine.of(Hearts)));
	assert!(!stack.contains(Nine.of(Diamonds)));
	assert!(!stack.contains(Eight.of(Spades)));
	assert_eq!(stack.len(), 1);
	assert_eq!(stack, Stack::single(Nine.of(Hearts)));
	assert_eq!(Stack::from_bits(stack.to_bits(2)), (stack.clone(), 2));
	assert_eq!(stack.take(), Ok(Nine.of(Hearts)));
	assert_eq!(stack.len(), 0);
	assert_eq!(stack.take(), Err(()));
}

#[test]
fn test_multiple() {
	let mut stack = Stack::single(Seven.of(Clubs));
	assert_eq!(stack.len(), 1);
	assert!(stack.contains(Seven.of(Clubs)));
	assert!(!stack.contains(Six.of(Diamonds)));
	assert!(stack.add(Six.of(Diamonds)).is_ok());
	assert_eq!(stack.len(), 2);
	assert!(stack.contains(Seven.of(Clubs)));
	assert!(stack.contains(Six.of(Diamonds)));
	assert!(!stack.contains(Six.of(Hearts)));
	assert!(stack.add(Five.of(Diamonds)).is_err());
	assert_eq!(stack.len(), 2);
	assert!(stack.add(Five.of(Clubs)).is_ok());
	assert_eq!(stack.len(), 3);
	assert!(stack.add(Four.of(Hearts)).is_ok());
	assert_eq!(stack.len(), 4);
	assert!(stack.add(Three.of(Spades)).is_ok());
	assert_eq!(stack.len(), 5);
	assert_eq!(Stack::from_bits(stack.to_bits(1)), (stack.clone(), 1));
	assert_eq!(stack.get(0), Some(Seven.of(Clubs)));
	assert_eq!(stack.get(1), Some(Six.of(Diamonds)));
	assert_eq!(stack.get(2), Some(Five.of(Clubs)));
	assert_eq!(stack.get(3), Some(Four.of(Hearts)));
	assert_eq!(stack.get(4), Some(Three.of(Spades)));
	assert_eq!(stack.get(5), None);
	assert!(stack.contains(Four.of(Hearts)));
	assert!(!stack.contains(Four.of(Diamonds)));
	assert!(stack.contains(Three.of(Spades)));
	assert!(!stack.contains(Three.of(Clubs)));
	assert!(!stack.contains(Two.of(Hearts)));
	assert!(!stack.contains(Two.of(Diamonds)));
	assert_eq!(stack.take(), Ok(Three.of(Spades)));
	assert_eq!(stack.take(), Ok(Four.of(Hearts)));
	assert_eq!(stack.take(), Ok(Five.of(Clubs)));
	assert_eq!(stack.take(), Ok(Six.of(Diamonds)));
	assert_eq!(stack.take(), Ok(Seven.of(Clubs)));
	assert_eq!(stack.take(), Err(()));
	assert_eq!(stack, Stack::new());
}
