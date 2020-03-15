#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Suit {
	Spades,
	Hearts,
	Clubs,
	Diamonds,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Color {
	Black,
	Red,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Variant {
	First,
	Second,
}

pub use Color::*;
pub use Suit::*;
pub use Variant::*;

impl Suit {
	pub fn all() -> impl Iterator<Item = Self> {
		(0..4).map(Self::from_num)
	}

	pub fn color(self) -> Color {
		match self {
			Spades => Black,
			Hearts => Red,
			Clubs => Black,
			Diamonds => Red,
		}
	}

	pub fn variant(self) -> Variant {
		match self {
			Spades => First,
			Hearts => First,
			Clubs => Second,
			Diamonds => Second,
		}
	}

	pub fn from_variant(color: Color, variant: Variant) -> Self {
		match (color, variant) {
			(Black, First) => Spades,
			(Red, First) => Hearts,
			(Black, Second) => Clubs,
			(Red, Second) => Diamonds,
		}
	}

	pub const fn num(self) -> usize {
		self as usize
	}

	pub fn from_num(n: usize) -> Self {
		match n {
			0 => Spades,
			1 => Hearts,
			2 => Clubs,
			3 => Diamonds,
			_ => panic!("Invalid suit number {}", n),
		}
	}
}

impl Color {
	pub fn opposite(self) -> Self {
		match self {
			Red => Black,
			Black => Red,
		}
	}
}

impl Variant {
	pub fn as_bit(self) -> bool {
		match self {
			First => false,
			Second => true,
		}
	}

	pub fn from_bit(b: bool) -> Self {
		match b {
			false => First,
			true => Second,
		}
	}
}

#[test]
fn test() {
	assert_eq!(Suit::all().count(), 4);
	for (i, suit) in Suit::all().enumerate() {
		let c = suit.color();
		let v = suit.variant();
		assert_eq!(Suit::from_variant(c, v), suit);
		assert_eq!(suit.num(), i);
		assert_eq!(Suit::from_num(i), suit);
		assert_eq!(Variant::from_bit(v.as_bit()), v);
		assert_ne!(Variant::from_bit(!v.as_bit()), v);
		assert_ne!(c, c.opposite());
		assert_eq!(c, c.opposite().opposite());
	}
}
