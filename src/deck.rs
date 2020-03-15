use crate::*;

#[derive(Clone)]
pub struct Deck {
	pub cards: [Card; 52],
}

impl Deck {
	pub fn new() -> Self {
		let mut cards = [Ace.of(Clubs); 52];
		for (i, card) in Card::all().enumerate() {
			cards[i] = card;
		}
		Self { cards }
	}
}

#[test]
fn test() {
	let deck = Deck::new();
	assert_eq!(deck.cards[0], Ace.of(Spades));
	assert_eq!(deck.cards[1], Two.of(Spades));
	assert_eq!(deck.cards[2], Three.of(Spades));
	assert_eq!(deck.cards[13], Ace.of(Hearts));
	assert_eq!(deck.cards[30], Five.of(Clubs));
	assert_eq!(deck.cards[51], King.of(Diamonds));
}
