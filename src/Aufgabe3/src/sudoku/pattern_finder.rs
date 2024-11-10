use super::{
	pattern::{self, Fragment, Pattern},
	Sudoku,
};

const EMPTY_PATTERN: Pattern = Pattern::new();

impl Sudoku {
	fn get_pattern(&self, iter: impl Iterator<Item = [u8; 9]>) -> pattern::List {
		let mut result = [EMPTY_PATTERN; 9];

		for (index, a) in iter.enumerate() {
			let mut pattern = Pattern::new();
			let mut fragment = Fragment {
				length: 0,
				is_zero: true,
			};

			for b in a {
				match fragment == b {
					true => fragment.length += 1,
					false => {
						pattern.push(fragment);
						fragment = Fragment {
							length: 1,
							is_zero: b == 0,
						}
					}
				}
			}

			result[index] = pattern;
		}

		result
	}

	pub fn get_patterns(&self) -> pattern::Collection<pattern::List> {
		pattern::Collection {
			horizontal: self.get_pattern(self.0.iter().cloned()),
			vertical: self.get_pattern(self.col_iter()),
		}
	}
}
