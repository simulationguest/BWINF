use image::{ImageBuffer, Rgb};
use log::{info, LevelFilter};
use rand::{Rng, random};
use rand::rngs::ThreadRng;
use std::fmt::Debug;
use std::io::Write;
use std::ops::Add;
use std::{collections::HashMap, fmt::Display};

// its cheaper to clone this thing than to create a ref
// (at least on 64-bit systems, where a pointer has 2x the size of this)
#[derive(Clone, Hash, PartialEq, Eq, Copy)]
struct Coordinate {
	x: u16,
	y: u16,
}

impl Display for Coordinate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "( {} | {} )", self.x, self.y)
	}
}

impl Debug for Coordinate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "( {} | {} )", self.x, self.y)
	}
}

#[derive(Debug)]
struct Growth {
	rate: u16,
}

#[derive(Debug)]
struct Sides<T> {
	left: T,
	right: T,
	top: T,
	bottom: T,
}

enum Side {
	Left,
	Right,
	Top,
	Bottom,
}

impl<T> Sides<T> {
	fn for_each<F>(&mut self, mut modifier: F)
	where
		F: FnMut(&mut T, Side),
	{
		modifier(&mut self.left, Side::Left);
		modifier(&mut self.right, Side::Right);
		modifier(&mut self.top, Side::Top);
		modifier(&mut self.bottom, Side::Bottom);
	}

	fn new<F>(mut init: F) -> Self
	where
		F: FnMut() -> T,
	{
		Self {
			left: init(),
			right: init(),
			bottom: init(),
			top: init(),
		}
	}
}

#[derive(Debug)]
struct Crystal {
	growth: Sides<Growth>,
	orientation: Coordinate,
}

impl Crystal {
	fn grow(&mut self, core: Coordinate, grid_size: u16, time: u16) -> Vec<Coordinate> {
		let mut new_bounds = Vec::new();

		self.growth.for_each(|growth, side| {
			let advancement = time * growth.rate;

			let mut boundary = core;
			match side {
				Side::Left => {
					if let Some(new) = boundary.x.checked_sub(advancement) {
						boundary.x = new;
					} else {
						return;
					}
				}
				Side::Right => {
					let new = boundary.x + advancement;
					if new >= grid_size {
						return;
					}
					boundary.x = new;
				}
				Side::Top => {
					if let Some(new) = boundary.y.checked_sub(advancement) {
						boundary.y = new;
					} else {
						return;
					}
				}
				Side::Bottom => {
					let new = boundary.y + advancement;
					if new >= grid_size {
						return;
					}
					boundary.y = new;
				}
			}

			new_bounds.push(boundary);
		});

		new_bounds
	}
}

struct Grid {
	time: u16,
	size: u16,
	matrix: Vec<Vec<Option<Coordinate>>>,
	crystals: HashMap<Coordinate, Crystal>,
}

// TODO: Handle bounds
// this is horrible
fn random_orientation(rng: &mut  ThreadRng, core: u16) -> u16 {
		let offset: i32 = rng.gen_range(-16..64);
		offset.add(core as i32) as u16
	}

impl Grid {
	fn new(size: u16, num_crystals: u8) -> Self {
		let mut crystals = HashMap::new();

		let mut rng = rand::thread_rng();

		for i in 0..num_crystals {
			info!("\n==========================\nCreating crystal {i}");
			let core: Coordinate;
			loop {
				let x = rng.gen_range(0..size);
				let y = rng.gen_range(0..size);

				let c = Coordinate { x, y };

				if let None = crystals.get(&c) {
					core = c;
					break;
				}
			}
			info!("Coordinates: {core}");


			let crystal = Crystal {
				growth: Sides::new(|| Growth {
					rate: rng.gen_range(0..32),
				}),
				orientation: Coordinate {
					x: random_orientation(&mut rng, core.x),
					y: random_orientation(&mut rng, core.y),
				},
			};

			info!("Growth rates: {:?}", crystal);

			crystals.insert(core, crystal);

			info!("Done creating crystal");
		}

		Self {
			matrix: vec![vec![None; size as usize]; size as usize],
			time: 0,
			crystals,
			size,
		}
	}

	fn tick(&mut self) {
		self.time += 1;

		info!("Tick No {}", self.time);

		for (coord, crystal) in self.crystals.iter_mut() {
			info!("Ticking crystal at {}", coord);

			let new_bounds = crystal.grow(coord.clone(), self.size, self.time);

			for boundary in new_bounds {
				let existing_field = &self.matrix[boundary.x as usize][boundary.y as usize];
				match existing_field {
					Some(c) => info!("Crystal already exists: {c}"),
					None => {
						self.matrix[boundary.x as usize][boundary.y as usize] = Some(coord.clone());
					}
				}
			}
		}
	}

	fn to_img(&self) -> ImageBuffer<Rgb<i32>, Vec<i32>> {
		let size = self.size as u32;
		let mut img = ImageBuffer::from_fn(size, size, |x, y| image::Rgb([200, 0, 0]));

		img
	}
}

fn main() {
	env_logger::Builder::new()
		.format(|buf, record| writeln!(buf, "[{}] {}", record.level(), record.args()))
		.filter(None, LevelFilter::Info)
		.init();

	let mut grid = Grid::new(10000, 20);

	for i in 0..100 {
		grid.tick();
	}
	
	for (x, row) in grid.matrix.iter().enumerate() {
		for (y, col) in row.iter().enumerate() {
			if let Some(c) = col {
				//println!("{x} | {y} : {c}");
			}
		}
	}
}
