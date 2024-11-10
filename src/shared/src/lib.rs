use std::{fmt::Debug, str::FromStr};

use thiserror::Error;

pub mod file_picker;

#[cfg(windows)]
pub const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
pub const LINE_ENDING: &'static str = "\n";

pub fn split_to_pairs<'a, T>(file: &'a str) -> impl Iterator<Item = (T, T)> + 'a
where
	T: FromStr,
	<T as FromStr>::Err: Debug,
{
	file.split(LINE_ENDING).map(|line| -> (T, T) {
		let (first, second): (&str, &str) = line.split_once(' ').unwrap();
		(first.parse::<T>().unwrap(), second.parse::<T>().unwrap())
	})
}

pub fn default<T>(val: Option<T>, default: T) -> T {
	match val {
		Some(x) => x,
		None => default,
	}
}
