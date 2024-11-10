use std::{collections::HashSet, fmt::Display, hash::Hash};

use super::Sudoku;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permutation {
	None,
	SwapRows {
		first: usize,
		second: usize,
	},
	SwapRowBlocks {
		first: usize,
		second: usize,
	},
	SwapRowsInBlock {
		block: usize,
		first: usize,
		second: usize,
	},
	SwapColumns {
		first: usize,
		second: usize,
	},
	SwapColumnBlocks {
		first: usize,
		second: usize,
	},
	SwapColumnsInBlock {
		block: usize,
		first: usize,
		second: usize,
	},
	SwapDigits(u8, u8),
	Rotate,
}

impl Permutation {
	pub fn get_possible() -> HashSet<Permutation> {
		let mut list: HashSet<Permutation> = HashSet::new();
		for _ in 0..6 {
			for first in 0..3 {
				for second in (first + 1)..3 {
					list.insert(Permutation::SwapColumns { first, second });
					list.insert(Permutation::SwapRows { first, second });
				}
			}
		}

		for a in 1..=9 {
			for b in (a + 1)..=9 {
				list.insert(Permutation::SwapDigits(a, b));
			}
		}

		list
	}

	pub fn apply(self, mut s: Sudoku) -> Sudoku {
		match self {
			Self::None => {}
			Self::Rotate => {
				let mut tmp: u8;
				let n = s.0.len();

				for i in 0..n {
					for j in 0..i {
						tmp = s.0[i][j];
						s.0[i][j] = s.0[j][i];
						s.0[j][i] = tmp;
					}
				}

				for i in 0..n {
					for j in 0..(n / 2) {
						tmp = s.0[i][j];
						s.0[i][j] = s.0[i][n - j - 1];
						s.0[i][n - j - 1] = tmp;
					}
				}
			}

			Self::SwapColumns { first, second } => {
				let mut tmp: u8;
				for row in 0..9 {
					tmp = s.0[row][first];
					s.0[row][first] = s.0[row][second];
					s.0[row][second] = tmp
				}
			}

			Self::SwapColumnsInBlock {
				block,
				first,
				second,
			} => {
				let block = block * 3;
				let first = first + block;
				let second = second + block;

				s = Self::SwapColumns { first, second }.apply(s)
			}

			Self::SwapColumnBlocks { first, second } => {
				let first = first * 3;
				let second = second * 3;

				let mut tmp: u8;
				for row in 0..9 {
					for col in 0..3 {
						let first = first + col;
						let second = second + col;
						tmp = s.0[row][first];
						s.0[row][first] = s.0[row][second];
						s.0[row][second] = tmp
					}
				}
			}

			Self::SwapRows { first, second } => {
				s.0.swap(first, second);
			}

			Self::SwapRowsInBlock {
				block,
				first,
				second,
			} => {
				if block > 2 || first > 2 || second > 2 {
					panic!("what on earth are you doin mate");
				}

				let block = block * 3;
				let first = block + first;
				let second = block + second;

				s = Self::SwapRows { first, second }.apply(s);
			}

			Self::SwapRowBlocks { first, second } => {
				if first > 2 || second > 2 || first == second {
					panic!("wtf bro");
				}

				let first = first * 3;
				let second = second * 3;

				// tmp speichert die momentan getauschte Zeile
				let mut tmp: [u8; 9];
				for i in 0..3 {
					tmp = s.0[first + i];
					s.0[first + i] = s.0[second + i];
					s.0[second + i] = tmp
				}
			}

			Self::SwapDigits(a, b) => {
				for row in s.0.iter_mut() {
					for col in row.iter_mut() {
						// not pretty but it works
						if *col == a {
							*col = b;
						} else if *col == b {
							*col = a;
						}
					}
				}
			}
		}
		s
	}
}

impl Display for Permutation {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Permutation::None => write!(f, "Keine Permutation"),
			Permutation::SwapRows { first, second } => {
				write!(f, "Vertauschung der Reihen {first} und {second}")
			}
			Permutation::SwapRowBlocks { first, second } => {
				write!(f, "Vertauschung der Reihenblöcke {first} und {second}")
			}
			Permutation::SwapRowsInBlock {
				block,
				first,
				second,
			} => write!(
				f,
				"Vertauschung der Reihen {first} und {second} im Block {block}"
			),
			Permutation::SwapColumns { first, second } => {
				write!(f, "Vertauschung der Spalten {first} und {second}")
			}
			Permutation::SwapColumnBlocks { first, second } => {
				write!(f, "Vertauschung der Spaltenblöcke {first} und {second}")
			}
			Permutation::SwapColumnsInBlock {
				block,
				first,
				second,
			} => write!(
				f,
				"Vertauschung der Spalten {first} und {second} in Block {block}"
			),
			Permutation::SwapDigits(first, second) => {
				write!(f, "Vertauschung der Ziffern {first} und {second}")
			}
			Permutation::Rotate => write!(f, "Drehung um 90 Grad"),
		}
	}
}
