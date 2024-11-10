#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Sudoku(pub [[u8; 9]; 9]);

impl Sudoku {
	pub fn col_iter(&self) -> ColIter {
		ColIter {
			index: 0,
			sudoku: self,
		}
	}
}

pub struct ColIter<'a> {
	index: usize,
	sudoku: &'a Sudoku,
}

//TODO use GATs to return a ref
impl<'a> Iterator for ColIter<'a> {
	type Item = [u8; 9];

	fn next(self: &mut ColIter<'a>) -> Option<Self::Item> {
		let mut col = [0u8; 9];

		if self.index >= 9 {
			return None;
		}

		for (row, item) in col.iter_mut().enumerate() {
			*item = self.sudoku.0[row][self.index];
		}

		self.index += 1;
		Some(col)
	}
}
