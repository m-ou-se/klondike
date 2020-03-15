use std::collections::HashSet;
use rand::seq::SliceRandom;
use klondike::*;

fn main() {
	let mut deck = Deck::new();
	deck.cards.shuffle(&mut rand::thread_rng());

	let mut game = Game::new(&deck);

	print_board(&game);

	let mut e = Explorer::default();

	if e.explore(&game, 0) {
		println!("Won!");
	} else {
		println!("Didn't win. :(");
		println!("Closest I got:");

		let s = e.seen.iter().max_by_key(|s| {
			let f = s.get_foundation();
			f.n_cards(Spades) + f.n_cards(Hearts) + f.n_cards(Clubs) + f.n_cards(Diamonds)
		});

		game.state = GameState::decode(*s.unwrap(), &game.deck);

		print_board(&game);
	}

	dbg!(e.seen.len());
}

#[derive(Default, Debug)]
struct Explorer {
	seen: HashSet<EncodedGameState>,
	actions: Vec<(EncodedGameState, Action)>,
}

impl Explorer {
	fn explore(&mut self, game: &Game, depth: usize) -> bool {
		if depth > 500 {
			for (i, &(e, a)) in self.actions.iter().enumerate() {
				println!("{}: {:?}", i, a);
				let game = Game {
					deck: game.deck,
					state: GameState::decode(e, &game.deck),
				};
				print_board(&game);
			}
			panic!("Too many steps");
			return false;
		}

		let key = game.state.encode();
		if self.seen.insert(key) == false {
			return false;
		}

		if game.state.foundation.is_complete() {
			for (i, &(e, a)) in self.actions.iter().enumerate() {
				println!("{}: {:?}", i, a);
				let game = Game {
					deck: game.deck,
					state: GameState::decode(e, &game.deck),
				};
				print_board(&game);
			}
			return true;
		}

		let mut newgame = (*game).clone();
		game.for_all_possible_actions(|action| {
			if newgame.action(action).is_ok() {
				self.actions.push((newgame.state.encode(), action));
				if self.explore(&newgame, depth + 1) { return true; }
				self.actions.pop();
				newgame = (*game).clone();
			}
			false
		})
	}
}

fn print_board(game: &Game) {
	//print!("\x1b[H\x1b[2J");
	match game.state.stock.n_closed_cards() {
		0 => print!(" --"),
		n => print!(" {:#>2}", n),
	}
	print!("      ");
	for suit in Suit::all() {
		match game.state.foundation.top_card(suit) {
			Some(card) => print!(" {:#}", card),
			None => print!(" {}{}", suit, suit),
		}
	}
	println!();
	{
		let mut s = game.state.stock.clone();
		for _ in 0..3 {
			match s.top_open_card(&game.deck) {
				Some(card) => print!(" {:#}", card),
				None => break,
			}
			s.take().ok();
		}
	}
	println!();
	println!();
	for i in 0usize..19 {
		for c in 0..7 {
			match i.checked_sub(game.state.closed.n_closed(c)) {
				None => print!(" ##"),
				Some(j) => match game.state.stacks[c].get(j) {
					Some(card) => print!(" {:#}", card),
					None if i == 0 => print!(" --"),
					None => print!("   "),
				}
			}
		}
		println!();
	}
	println!();
}
