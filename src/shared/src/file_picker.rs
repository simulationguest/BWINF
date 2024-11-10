use inquire::Select;
use std::{
	collections::HashMap,
	ffi::OsString,
	fs::{self, DirEntry, File},
	io::{BufRead, BufReader, Lines},
	path::PathBuf,
};

use super::Error;

fn picker(program_name: &str) -> Result<PathBuf, FilePickerError> {
	let mut choices: HashMap<String, DirEntry> = HashMap::new();

	fs::read_dir("./examples")?.try_for_each(|item| -> Result<(), FilePickerError> {
		let item = item?;
		if item.path().is_file() {
			let name = item.file_name().into_string()?;
			choices.insert(name, item);
		}
		Ok(())
	})?;

	let query = format!("Select one file to run using {program_name}");
	let result = Select::new(&query, choices.keys().collect()).prompt()?;

	// here cannot be an error, hence ``.unwrap()`` is fine
	let file = choices.get(result).unwrap();

	Ok(file.path())
}

pub fn to_string(program_name: &str) -> Result<String, FilePickerError> {
	let path = picker(program_name)?;
	Ok(fs::read_to_string(path)?)
}

pub fn read_lines(program_name: &str) -> Result<Lines<BufReader<File>>, FilePickerError> {
	let path = picker(program_name)?;
	let file = File::open(path)?;
	Ok(BufReader::new(file).lines())
}

#[derive(Error, Debug)]
pub enum FilePickerError {
	#[error(transparent)]
	IoError(#[from] std::io::Error),
	#[error(transparent)]
	InquireError(#[from] inquire::InquireError),
	#[error("Filename is not valid utf8")]
	OsStringError,
}

impl From<OsString> for FilePickerError {
	fn from(_: OsString) -> Self {
		Self::OsStringError
	}
}
