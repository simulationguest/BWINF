use super::{Permutation, Sudoku};

pub fn rotate(
	mut start: Sudoku,
	goal: &Sudoku,
	algorithm: fn(start: Sudoku, goal: &Sudoku) -> Option<Vec<Permutation>>,
) -> Option<Vec<Permutation>> {
	for i in 0..4_u8 {
		let result = algorithm(start.clone(), &goal);

		if let Some(mut solution) = result {
			for _ in 0..i {
				solution.push(Permutation::Rotate);
			}
			return Some(solution);
		}

		if i < 3 {
			start = Permutation::Rotate.apply(start)
		}
	}
	None
}
