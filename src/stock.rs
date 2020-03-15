use crate::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Stock {
	pub(crate) state: u32,
}

impl Stock {
	pub const fn new() -> Self {
		Self {
			state: 0xFFFF_FF00,
		}
	}

	pub const fn is_empty(&self) -> bool {
		self.state == 0
	}

	pub const fn n_closed_cards(&self) -> usize {
		((self.state & !0xFF) << (self.state & 0xFF)).count_ones() as usize
	}

	pub const fn n_cards(&self) -> usize {
		(self.state >> 8).count_ones() as usize
	}

	pub fn top_open_card(&self, deck: &Deck) -> Option<Card> {
		match self.state & 0xFF {
			0 => None,
			i => Some(deck.cards[24 - i as usize]),
		}
	}

	fn open(&mut self) -> Result<(), ()> {
		match (self.state & !0xFF) << (self.state & 0xFF) {
			0 => Err(()),
			x => {
				self.state += x.leading_zeros() + 1;
				Ok(())
			}
		}
	}

	pub fn next(&mut self, n: usize) {
		for i in 0..n {
			if self.open().is_err() {
				if i == 0 {
					self.state &= !0xFF;
				}
				return;
			}
		}
	}

	pub fn take(&mut self) -> Result<(), ()> {
		if let Some(n) = (self.state & 0xFF).checked_sub(1) {
			self.state &= !(0x8000_0000 >> n);
			let x = self.state >> 8 >> (24 - n);
			if x == 0 {
				self.state &= !0xFF;
			} else {
				self.state -= x.trailing_zeros() + 1;
			}
			Ok(())
		} else {
			Err(())
		}
	}
}

#[test]
fn test() {
	let deck = Deck::new(); // Sorted deck

	let mut stock = Stock::new(); // Uses cards 0..24 from the deck.

	// Starting situation:
	//  Closed cards: (bottom) ♠A ♠2 ♠3 ... ♥9 ♥T ♥J (top)
	//  No open cards.

	assert_eq!(stock.is_empty(), false);
	assert_eq!(stock.n_cards(), 24);
	assert_eq!(stock.n_closed_cards(), 24);
	assert_eq!(stock.top_open_card(&deck), None);

	stock.next(1);

	assert_eq!(stock.top_open_card(&deck), Some(Jack.of(Hearts)));
	assert_eq!(stock.top_open_card(&deck), Some(Jack.of(Hearts)));

	stock.next(3);

	assert_eq!(stock.top_open_card(&deck), Some(Eight.of(Hearts)));

	assert_eq!(stock.is_empty(), false);
	assert_eq!(stock.n_cards(), 24);
	assert_eq!(stock.n_closed_cards(), 20);

	assert_eq!(stock.take(), Ok(()));

	assert_eq!(stock.is_empty(), false);
	assert_eq!(stock.n_cards(), 23);
	assert_eq!(stock.n_closed_cards(), 20);
	assert_eq!(stock.top_open_card(&deck), Some(Nine.of(Hearts)));

	stock.next(1);

	assert_eq!(stock.is_empty(), false);
	assert_eq!(stock.n_cards(), 23);
	assert_eq!(stock.n_closed_cards(), 19);
	assert_eq!(stock.top_open_card(&deck), Some(Seven.of(Hearts)));

	assert_eq!(stock.take(), Ok(()));
	assert_eq!(stock.top_open_card(&deck), Some(Nine.of(Hearts)));
	assert_eq!(stock.take(), Ok(()));
	assert_eq!(stock.top_open_card(&deck), Some(Ten.of(Hearts)));
	assert_eq!(stock.take(), Ok(()));
	assert_eq!(stock.top_open_card(&deck), Some(Jack.of(Hearts)));
	assert_eq!(stock.take(), Ok(()));
	assert_eq!(stock.top_open_card(&deck), None);
	assert_eq!(stock.take(), Err(()));
	assert_eq!(stock.top_open_card(&deck), None);
	assert_eq!(stock.is_empty(), false);
	assert_eq!(stock.n_cards(), 19);
	assert_eq!(stock.n_closed_cards(), 19);

	stock.next(17);

	assert_eq!(stock.top_open_card(&deck), Some(Three.of(Spades)));
	assert_eq!(stock.take(), Ok(()));

	assert_eq!(stock.n_closed_cards(), 2);
	assert_eq!(stock.n_cards(), 18);

	stock.next(10);

	assert_eq!(stock.n_closed_cards(), 0);
	assert_eq!(stock.n_cards(), 18);
	assert_eq!(stock.top_open_card(&deck), Some(Ace.of(Spades)));

	stock.next(10);

	assert_eq!(stock.n_closed_cards(), 18);
	assert_eq!(stock.n_cards(), 18);

	assert_eq!(stock.top_open_card(&deck), None);

	stock.next(18);

	for _ in 0..18 {
		assert_eq!(stock.take(), Ok(()));
	}

	assert_eq!(stock.is_empty(), true);
	assert_eq!(stock.n_cards(), 0);
	assert_eq!(stock.n_closed_cards(), 0);
	assert_eq!(stock.top_open_card(&deck), None);

	assert_eq!(stock.take(), Err(()));

	stock.next(1);

	assert_eq!(stock.is_empty(), true);
	assert_eq!(stock.n_cards(), 0);
	assert_eq!(stock.n_closed_cards(), 0);
	assert_eq!(stock.top_open_card(&deck), None);

	assert_eq!(stock.take(), Err(()));
}
