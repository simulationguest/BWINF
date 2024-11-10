#![feature(test)]

extern crate test;
use test::Bencher;

mod sudoku;

use sudoku::{bruteforce, rotate, smart_find, Permutation, Sudoku};

fn main() {}

#[bench]
fn test_smart(b: &mut Bencher) {
	let file = std::fs::read_to_string("./examples/sudoku2.txt").unwrap();
	let (start, goal) = Sudoku::parse_file(&file).unwrap();

	//let goal = Permutation::SwapColumns { first: 4, second: 5 }.apply(goal);

	b.iter(|| {
		rotate(start.clone(), &goal, |start, goal| {
			smart_find(start, goal, goal.get_patterns())
		})
	})
}

#[bench]
fn test_brute(b: &mut Bencher) {
	let file = std::fs::read_to_string("./examples/sudoku2.txt").unwrap();
	let (start, goal) = Sudoku::parse_file(&file).unwrap();

	//let goal = Permutation::SwapColumns { first: 4, second: 5 }.apply(goal);

	b.iter(|| {
		rotate(start.clone(), &goal, |start, goal| {
			bruteforce(start, goal, 2)
		})
	})
}
