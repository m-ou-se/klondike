use intbits::Bits;
use crate::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct GameState {
	pub stock: Stock,
	pub closed: ClosedState,
	pub stacks: [Stack; 7],
	pub foundation: Foundation,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EncodedGameState(pub(crate) u64, pub(crate) u128);

impl GameState {
	pub const fn new(deck: &Deck) -> Self {
		GameState {
			stock: Stock::new(),
			closed: ClosedState::new(),
			stacks: [
				Stack::single(deck.cards[51]),
				Stack::single(deck.cards[44]),
				Stack::single(deck.cards[38]),
				Stack::single(deck.cards[33]),
				Stack::single(deck.cards[29]),
				Stack::single(deck.cards[26]),
				Stack::single(deck.cards[24]),
			],
			foundation: Foundation::new(),
		}
	}

	pub fn encode(&self) -> EncodedGameState {
		let f = self.foundation.to_bits();
		let mut a = 0u64;
		let mut b = 0u128;
		for i in 0..7 {
			let (x, y) = self.stacks[i].to_bits(f.bits(i * 2..).bits(..2) as u8);
			a.set_bits(i * 8.., x.into());
			b.set_bits(i * 16.., y.into());
		}
		a.set_bits(56.., self.stock.state.bits(..8).into());
		a.set_bits(62.., f.bits(14..).into());
		b.set_bits(112.., self.closed.to_bits(0).into());
		EncodedGameState(a, b)
	}

	pub fn decode(bits: EncodedGameState, deck: &Deck) -> Self {
		let mut state = Self::new(deck);
		let mut f = 0u16;
		for i in 0..7 {
			let x = bits.0.bits(i * 8..) as u8;
			let y = bits.1.bits(i * 16..) as u16;
			let (stack, e) = Stack::from_bits((x, y));
			state.stacks[i] = stack;
			f.set_bits(i * 2.., e.into());
		}
		f.set_bits(14.., bits.0.bits(62..) as u16);
		state.foundation = Foundation::from_bits(f);
		state.closed = ClosedState::from_bits(bits.1.bits(112..) as u16).0;
		let mut stock = bits.0.bits(56..62) as u32;
		for (i, &card) in deck.cards[..24].iter().enumerate() {
			if
				!state.foundation.contains(card) &&
				!state.stacks.iter().any(|s| s.contains(card))
			{
				stock.set_bit(i + 8, true);
			}
		}
		state.stock = Stock { state: stock };
		state
	}
}

impl EncodedGameState {
	pub fn get_foundation(&self) -> Foundation {
		let mut f = 0u16;
		for i in 0..8 {
			let x = self.0.bits(i * 8..) as u8;
			f.set_bits(i * 2.., x.bits(6..).into());
		}
		Foundation::from_bits(f)
	}
}

#[test]
fn test_encode_decode() {
	let deck = Deck::new();
	let mut s = GameState::new();
	assert_eq!(s, GameState::decode(s.encode(), &deck));
	s.stock.next(3);
	assert_eq!(s, GameState::decode(s.encode(), &deck));
	s.stacks[1].add(s.stock.top_open_card(&deck).unwrap()).unwrap();
	s.stock.take();
	assert_eq!(s, GameState::decode(s.encode(), &deck));
	s.stock.next(3);
	assert_eq!(s, GameState::decode(s.encode(), &deck));
	s.stacks[2].add(s.closed.open(6, &deck).unwrap()).unwrap();
	s.stacks[3].add(s.closed.open(6, &deck).unwrap()).unwrap();
	assert_eq!(s, GameState::decode(s.encode(), &deck));
	s.stacks[4].add(s.closed.open(6, &deck).unwrap()).unwrap();
	s.stacks[5].add(s.closed.open(6, &deck).unwrap()).unwrap();
	s.foundation.add(s.closed.open(6, &deck).unwrap()).unwrap();
	assert_eq!(s, GameState::decode(s.encode(), &deck));
}
