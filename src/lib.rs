mod card;
mod closed;
mod deck;
mod display;
mod foundation;
mod gamestate;
mod rank;
mod stack;
mod stock;
mod suit;

pub use card::*;
pub use closed::ClosedState;
pub use deck::Deck;
pub use foundation::Foundation;
pub use gamestate::{EncodedGameState, GameState};
pub use rank::*;
pub use stack::Stack;
pub use stock::Stock;
pub use suit::*;

#[derive(Clone, Debug)]
pub struct Game<'a> {
	pub deck: &'a Deck,
	pub state: GameState,
}

impl<'a> Game<'a> {
	pub const fn new(deck: &'a Deck) -> Self {
		Self {
			deck,
			state: GameState::new(deck),
		}
	}

	pub fn action(&mut self, action: Action) -> Result<(), ()> {
		match action {
			Action::NextStock => {
				if self.state.stock.is_empty() {
					Err(())
				} else {
					self.state.stock.next(3);
					Ok(())
				}
			}
			Action::StockToFoundation => {
				let card = self.state.stock.top_open_card(&self.deck).ok_or(())?;
				self.state.foundation.add(card)?;
				self.state.stock.take().unwrap();
				Ok(())
			}
			Action::StockToStack(column) => {
				let card = self.state.stock.top_open_card(&self.deck).ok_or(())?;
				self.state.stacks[usize::from(column)].add(card)?;
				self.state.stock.take().unwrap();
				Ok(())
			}
			Action::StackToFoundation(column) => {
				let stack = &mut self.state.stacks[usize::from(column)];
				let card = stack.last().ok_or(())?;
				self.state.foundation.add(card)?;
				stack.take().unwrap();
				if stack.len() == 0 {
					if let Ok(card) = self.state.closed.open(usize::from(column), self.deck) {
						*stack = Stack::single(card);
					}
				}
				Ok(())
			}
			Action::StackToStack { from, to, n } => {
				assert_ne!(from, to);
				let from_col = usize::from(from);
				// Safe, because `to` and `from` are different.
				let from = unsafe { &mut *self.state.stacks.as_mut_ptr().add(from_col) };
				let to = unsafe { &mut *self.state.stacks.as_mut_ptr().add(usize::from(to)) };
				let n = usize::from(n);
				let offset = from.len().checked_sub(n).ok_or(())?;
				let card = from.get(offset).unwrap();
				to.add(card)?;
				for i in 1..n {
					to.add(from.get(offset + i).unwrap()).unwrap();
				}
				for _ in 0..n {
					from.take().unwrap();
				}
				if from.len() == 0 {
					if let Ok(card) = self.state.closed.open(from_col, self.deck) {
						*from = Stack::single(card);
					}
				}
				Ok(())
			}
		}
	}

	pub fn for_all_possible_actions(&self, mut f: impl FnMut(Action) -> bool) -> bool {
		if f(Action::StockToFoundation) { return true; }

		for i in 0..7 {
			if f(Action::StockToStack(i)) { return true; }
		}

		for i in 0..7 {
			if f(Action::StackToFoundation(i)) { return true; }
		}

		if f(Action::NextStock) { return true; }

		for from in 0..7 {
			for to in 0..7 {
				if from == to { continue; }
				//for n in 1..=self.state.stacks[usize::from(from)].len() as u8 {
				let n = self.state.stacks[usize::from(from)].len() as u8;
				let m = self.state.stacks[usize::from(to)].len();
				if m == 0 && self.state.closed.n_closed(usize::from(from)) == 0 {
					// nope
				} else {
					if n > 0 && f(Action::StackToStack { from, to, n }) { return true; }
				}
			}
		}

		false
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Action {
	NextStock,
	StockToFoundation,
	StockToStack(u8),
	StackToFoundation(u8),
	StackToStack {
		from: u8,
		to: u8,
		n: u8,
	}
}
