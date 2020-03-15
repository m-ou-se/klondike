use crate::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ClosedState {
	state: u16,
}

impl ClosedState {
	pub const fn new() -> Self {
		Self { state: 0o123456 }
	}

	pub fn n_closed(&self, column: usize) -> usize {
		if column == 0 {
			0
		} else {
			usize::from(self.state >> shift(column) & 0o7)
		}
	}

	pub fn open(&mut self, column: usize, deck: &Deck) -> Result<Card, ()> {
		if let Some(n) = self.n_closed(column).checked_sub(1) {
			self.state -= 1 << shift(column);
			let index = 51 - [0, 6, 11, 15, 18, 20][n] - column;
			Ok(deck.cards[index])
		} else {
			Err(())
		}
	}

	pub fn to_bits(&self, two_extra_bits: u8) -> u16 {
		assert!(two_extra_bits < 4);
		let a = u16::from(two_extra_bits >> 1);
		let b = u16::from(two_extra_bits & 1);
		self.state | a << 14 | b << 11
	}

	pub fn from_bits(bits: u16) -> (Self, u8) {
		let state = bits & 0o133777;
		let a = bits >> 14 & 1;
		let b = bits >> 11 & 1;
		(Self { state }, (a << 1 | b) as u8)
	}
}

fn shift(column: usize) -> u8 {
	assert!(column < 7);
	(18 - column * 3) as u8
}

#[test]
fn test() {
	let deck = Deck::new();
	// With an ordered deck, the initially closed cards should be:
	//
	//  0  1  2  3  4  5  6
	// -- ♦Q ♦J ♦T ♦9 ♦8 ♦7
	//       ♦5 ♦4 ♦3 ♦2 ♦A
	//          ♣Q ♣J ♣T ♣9
	//             ♣7 ♣6 ♣5
	//                ♣3 ♣2
	//                   ♥K
	//

	let mut state = ClosedState::new();
	for i in 0..7 {
		assert_eq!(state.n_closed(i), i);
	}
	assert_eq!(ClosedState::from_bits(state.to_bits(0)), (state.clone(), 0));
	assert_eq!(state.open(0, &deck), Err(()));
	assert_eq!(state.open(1, &deck), Ok(Queen.of(Diamonds)));
	assert_eq!(state.open(1, &deck), Err(()));
	assert_eq!(state.n_closed(0), 0);
	assert_eq!(state.n_closed(1), 0);
	assert_eq!(state.n_closed(2), 2);
	assert_eq!(state.open(5, &deck), Ok(Three.of(Clubs)));
	assert_eq!(ClosedState::from_bits(state.to_bits(1)), (state.clone(), 1));
	assert_eq!(state.open(5, &deck), Ok(Six.of(Clubs)));
	assert_eq!(state.open(5, &deck), Ok(Ten.of(Clubs)));
	assert_eq!(ClosedState::from_bits(state.to_bits(3)), (state.clone(), 3));
	assert_eq!(state.n_closed(5), 2);
	assert_eq!(state.open(6, &deck), Ok(King.of(Hearts)));
	assert_eq!(state.n_closed(6), 5);
	assert_eq!(state.open(5, &deck), Ok(Two.of(Diamonds)));
	assert_eq!(ClosedState::from_bits(state.to_bits(2)), (state.clone(), 2));
	assert_eq!(state.open(5, &deck), Ok(Eight.of(Diamonds)));
	assert_eq!(state.open(5, &deck), Err(()));
	assert_eq!(state.n_closed(5), 0);
}
