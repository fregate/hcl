
use super::Db;
use super::Error;
use super::Hcl;

use std::fs;
use std::io;

impl Hcl {
	pub fn new() -> Hcl {
		Hcl
	}

	/// Open existing or create new file for store db entries.
	pub fn open<'a>(&self, path: &str) -> Result<Box<Db>, Error> {
		let res = fs::File::open(path);

		let res = match res {
			Ok(file) => Ok(file),
			Err(error) => match error.kind() {
				io::ErrorKind::NotFound => match fs::File::create(path) {
					Ok(file) => Ok(file),
					Err(error) => Err(Error::from(error))
				},
				_ => Err(Error::from(error))
			}
		};

		let res = match res {
			Ok(file) => Db::new(path, file),
			Err(error) => Err(error)
		};

		res
	}
}
