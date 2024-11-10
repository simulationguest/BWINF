use super::Sudoku;
use shared::LINE_ENDING;
use std::{num::ParseIntError, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SudokuParseErr {
	#[error(transparent)]
	ParseIntError(#[from] ParseIntError),
	#[error("error splitting sudoku file")]
	SplitFileError(),
}

impl FromStr for Sudoku {
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut sudoku: Sudoku = Default::default();
		for (i, line) in s.splitn(9, shared::LINE_ENDING).enumerate() {
			for (j, num) in line.splitn(9, ' ').enumerate() {
				let num = num.parse::<u8>()?;
				sudoku.0[i][j] = num;
			}
		}
		Ok(sudoku)
	}

	type Err = SudokuParseErr;
}

impl Sudoku {
	pub fn parse_file(file: &str) -> Result<(Sudoku, Sudoku), SudokuParseErr> {
		let delimiter = LINE_ENDING.to_owned() + LINE_ENDING;

		match file.split_once(&delimiter) {
			Some((a, b)) => Ok((Sudoku::from_str(a)?, Sudoku::from_str(b)?)),
			None => Err(SudokuParseErr::SplitFileError()),
		}
	}
}
