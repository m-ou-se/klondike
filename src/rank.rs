#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Rank {
	Ace = 1,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Jack,
	Queen,
	King,
}

pub use Rank::*;

impl Rank {
	pub fn all() -> impl Iterator<Item = Self> {
		(1..=13).map(Self::from_num)
	}

	pub const fn num(self) -> usize {
		self as usize
	}

	pub fn from_num(n: usize) -> Self {
		match n {
			1 => Ace,
			2 => Two,
			3 => Three,
			4 => Four,
			5 => Five,
			6 => Six,
			7 => Seven,
			8 => Eight,
			9 => Nine,
			10 => Ten,
			11 => Jack,
			12 => Queen,
			13 => King,
			_ => panic!("Invalid rank number {}", n),
		}
	}
}

#[test]
fn test() {
	assert_eq!(Rank::all().count(), 13);
	for (i, rank) in Rank::all().enumerate() {
		assert_eq!(rank.num(), i + 1);
		assert_eq!(Rank::from_num(i + 1), rank);
	}
}
