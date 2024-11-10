use crate::{Permutation, Sudoku};
use std::{cell::RefCell, rc::Rc};

struct Node {
	parent: Option<Rc<RefCell<Node>>>,
	sudoku: Option<Box<Sudoku>>,
	perm: Permutation,
}

impl Node {
	fn walk<F: FnMut(&mut Self)>(&mut self, mut for_each: F) {
		for_each(self);
		if let Some(node) = &self.parent {
			node.borrow_mut().walk(for_each);
		}
	}
}

pub fn bruteforce(start: Sudoku, goal: &Sudoku, depth: u8) -> Option<Vec<Permutation>> {
	let root = Rc::new(RefCell::new(Node {
		parent: None,
		perm: Permutation::None,
		sudoku: Some(Box::new(start.clone())),
	}));

	// The "leaves" are the bottom nodes of the tree
	let mut leaves = vec![Rc::clone(&root)];

	for _ in 0..depth {
		let possible_permutations = Permutation::get_possible();

		let mut new_leaves: Vec<Rc<RefCell<Node>>> = Vec::new();

		for node in leaves.drain(..) {
			let mut possible_permutations = possible_permutations.clone();
			node.borrow_mut().walk(|n| {
				possible_permutations.remove(&n.perm);
			});

			for permutation in possible_permutations.drain() {
				let sudoku = *node.borrow().sudoku.clone().unwrap();
				let new_sudoku = permutation.apply(sudoku);

				if new_sudoku == *goal {
					let mut solution: Vec<Permutation> = vec![permutation];

					node.borrow_mut().walk(|n| {
						if n.perm != Permutation::None {
							solution.push(n.perm);
						}
					});

					return Some(solution);
				}

				let new_node = Rc::new(RefCell::new(Node {
					parent: Some(Rc::clone(&node)),
					perm: permutation,
					sudoku: Some(Box::new(new_sudoku)),
				}));

				new_leaves.push(new_node);
			}

			node.borrow_mut().sudoku = None;
		}
		leaves = new_leaves;
	}
	None
}
