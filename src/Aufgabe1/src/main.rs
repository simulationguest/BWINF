use regex::Regex;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
	let book = fs::read_to_string("./buch.txt")?;
	let sentence = shared::file_picker::to_string("Stoerung")?;

	let query = format!("(?mi){}", sentence.replace('_', "[[:alpha:]]+"));
	let re = Regex::new(&query).unwrap();

	println!("Ursprünglicher Satz: \"{sentence}\"");

	for (idx, mat) in re.find_iter(&book).enumerate() {
		println!("{idx}: {}", mat.as_str());
	}

	Ok(())
}
