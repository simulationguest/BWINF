use super::{
	pattern::{self, Collection},
	Permutation, Sudoku,
};

impl Sudoku {
	fn match_numbers(self, other: &Self) -> (Self, Vec<Permutation>) {
		let mut path = Vec::new();

		let mut sudoku = self;

		for r in 0..9 {
			for c in 0..9 {
				let a = sudoku.0[r][c];
				let b = other.0[r][c];
				if a != b {
					let perm = Permutation::SwapDigits(a, b);
					sudoku = perm.apply(sudoku);
					path.push(perm);
				}
			}
		}

		(sudoku, path)
	}
}

pub fn smart_find(
	mut start: Sudoku,
	goal: &Sudoku,
	goal_patterns: Collection<pattern::List>,
) -> Option<Vec<Permutation>> {
	let mut path = Vec::new();

	let patterns = start.get_patterns();
	let diff = patterns.diff(&goal_patterns);

	start = diff.apply(start, &mut path);

	if start == *goal {
		return Some(path);
	}

	let (s, mut numbers) = start.match_numbers(goal);

	path.append(&mut numbers);

	if s == *goal {
		Some(path)
	} else {
		None
	}
}
