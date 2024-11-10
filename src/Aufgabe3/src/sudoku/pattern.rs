use super::{Permutation, Sudoku};

pub type List = [Pattern; 9];

pub trait ListExt {
	fn find_matching(&self, other: &Self) -> Vec<Match>;
}

impl ListExt for List {
	//TODO: use hashmap
	fn find_matching(&self, other: &Self) -> Vec<Match> {
		let mut matches = Vec::new();

		for (a_index, a) in self.iter().enumerate() {
			for (b_index, b) in other.iter().enumerate() {
				if a == b && a_index != b_index {
					let mut a = a_index;
					let mut b = b_index;

					// Swap if necessary
					if b > a {
						std::mem::swap(&mut a, &mut b);
					}

					let m = Match {
						first: a as u8,
						second: b as u8,
					};

					if !matches.contains(&m) {
						matches.push(m);
					}
				}
			}
		}

		matches
	}
}

pub type Pattern = Vec<Fragment>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fragment {
	pub length: u8,
	pub is_zero: bool, // Empty: is zero
}

impl PartialEq<u8> for Fragment {
	fn eq(&self, other: &u8) -> bool {
		(*other != 0 && !self.is_zero) || (*other == 0 && self.is_zero)
	}
}

#[derive(Debug)]
pub struct Collection<T> {
	pub horizontal: T,
	pub vertical: T,
}

impl<T> Collection<T> {
	pub fn for_each<F>(&mut self, mut f: F)
	where
		F: FnMut(&T),
	{
		f(&self.vertical);
		f(&self.horizontal);
	}
}

impl Collection<List> {
	pub fn diff(&self, other: &Self) -> Diff {
		let horizontal = self.horizontal.find_matching(&other.horizontal);
		let vertical = self.vertical.find_matching(&other.vertical);

		Collection {
			horizontal,
			vertical,
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct Match {
	first: u8,
	second: u8,
}

pub type Diff = Collection<Vec<Match>>;

impl Diff {
	pub fn apply(&self, mut sudoku: Sudoku, path: &mut Vec<Permutation>) -> Sudoku {
		for m in self.horizontal.iter() {
			let permutation = Permutation::SwapRows {
				first: m.first as usize,
				second: m.second as usize,
			};
			path.push(permutation);
			sudoku = permutation.apply(sudoku);
		}

		for m in self.vertical.iter() {
			let permutation = Permutation::SwapColumns {
				first: m.first as usize,
				second: m.second as usize,
			};
			path.push(permutation);
			sudoku = permutation.apply(sudoku);
		}

		sudoku
	}
}
