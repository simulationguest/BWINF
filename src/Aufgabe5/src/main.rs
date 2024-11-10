use bimap::{BiHashMap, BiMap};
use colored::Colorize;
use petgraph::prelude::UnGraph;
use std::{
	error::Error,
	fs::File,
	io::{BufReader, Lines},
	str::FromStr,
};

use petgraph::{algo::astar, graph::NodeIndex};

fn main() -> Result<(), Box<dyn Error>> {
	let lines = shared::file_picker::read_lines("Huepfburg")?;

	let (graph, elements) = parse_file(lines);

	let sasha = *elements.get_by_left(&1u8).unwrap();
	let mika = *elements.get_by_left(&2u8).unwrap();

	let path = astar(&graph, sasha, |node| node == mika, |_| 0, |_| 0);

	display(path, elements);

	Ok(())
}

fn display(path: Option<(i32, Vec<NodeIndex>)>, elements: BiHashMap<u8, NodeIndex>) {
	match path {
		None => println!("No path found between the two :("),
		Some(path) => {
			println!("Route found: {} steps", path.1.len());

			let mid = path.1.len() / 2;

			for (index, step) in path.1.iter().enumerate() {
				let value = elements.get_by_right(step).unwrap();
				let mut value = format!("{:?}", value);

				if index == 0 {
					value = value.bold().red().to_string();
				} else if index == path.1.len() - 1 {
					value = value.bold().blue().to_string();
				} else if index == mid {
					value = value.bold().green().to_string();
				}

				print!("{}", value);

				if index < mid {
					print!(" → ");
				} else if index < path.1.len() - 1 {
					print!(" ← ");
				}
			}
			println!();
		}
	}
}

fn parse_file(file: Lines<BufReader<File>>) -> (UnGraph<u8, ()>, BiMap<u8, NodeIndex>) {
	let mut edges: BiHashMap<u8, NodeIndex> = BiMap::new();
	let mut graph = UnGraph::<u8, ()>::new_undirected();

	let mut file = file.skip(1);

	while let Some(Ok(line)) = file.next() {
		let pair = Pair::from_str(&line).unwrap();

		let indices = pair.map(|n| match edges.get_by_left(&n) {
			Some(edge) => *edge,
			None => {
				let idx = graph.add_node(*n);
				edges.insert(*n, idx);
				idx
			}
		});

		graph.add_edge(indices.0, indices.1, ());
	}

	return (graph, edges);
}

struct Pair<T>(T, T);

impl<T> FromStr for Pair<T>
where
	T: FromStr,
{
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let pair: (&str, &str) = s.split_once(' ').unwrap();
		Ok(Self(
			pair.0.parse().unwrap_or_else(|_| panic!()),
			pair.1.parse().unwrap_or_else(|_| panic!()),
		))
	}
}

impl<T> Pair<T> {
	fn map<F, R>(&self, mut f: F) -> Pair<R>
	where
		F: FnMut(&T) -> R,
	{
		Pair(f(&self.0), f(&self.1))
	}
}
