
use super::Db;
use super::Error;
use super::Hcl;

use std::fs;
use std::io;
use std::path;

impl Hcl {
	pub fn new() -> Hcl {
		Hcl
	}

	/// Open existing or create new file for store db entries.
	pub fn open<'a>(&self, path: &'a path::Path) -> Result<Db, Error> {
		let res = fs::File::open(path);

		let res = match res {
			Ok(file) => Ok(file),
			Err(error) => match error.kind() {
				io::ErrorKind::NotFound => match fs::File::create(path) {
					Ok(file) => Ok(file),
					Err(error) => Err(Error::new(-1, format!("{}", error).as_ref()))
				},
				_ => Err(Error::new(-1,format!("{}", error).as_ref() ))
			}
		};

		let res = match res {
			Ok(file) => Db::new(file),
			Err(error) => Err(error)
		};

		res
	}
}
