use colored::Colorize;
use std::error::Error;
use sudoku::{bruteforce, smart_find, Sudoku};

use crate::sudoku::{rotate, Permutation};

mod sudoku;

fn main() -> Result<(), Box<dyn Error>> {
	let file = shared::file_picker::to_string("Sudoku")?;
	let (start, goal) = Sudoku::parse_file(&file)?;

	let smart_result = rotate(start.clone(), &goal, |start, goal| {
		smart_find(start, goal, goal.get_patterns())
	});
	show_result(smart_result, "Smart");

	let brute_result = rotate(start, &goal, |start, goal| bruteforce(start, goal, 4));
	show_result(brute_result, "Bruteforce");

	Ok(())
}

fn show_result(res: Option<Vec<Permutation>>, algo: &str) {
	let out = format!("Ergebnis für Lösungsweg {}:", algo.blue().to_string())
		.bold()
		.to_string();
	println!("{out}");
	match res {
		Some(perms) => {
			for (i, p) in perms.iter().enumerate() {
				println!("{i}: {p}");
			}
		}
		None => println!("Es wurde kein Weg gefunden."),
	}
	spacer();
}

fn spacer() {
	println!()
}
