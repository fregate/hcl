use super::Hcl;
use super::Db;
use super::Error;

use std::path;
use std::fmt;
use std::fs;

impl Hcl {
	pub fn new() -> Hcl {
		Hcl
	}

	/// Open existing or create new file for store db entries.
	pub fn open<'a>(&self, path: &'a path::Path) -> Result<Db, Error> {
		let res = fs::File::open(path);
		if let Ok(file) = res {
			return Result::Ok(Db::new(file));
		}

		let err = match res {
			Ok(_) => panic!(""),
			Err(e) => e
		};
		Result::Err(Error::new(-1, format!("{}", err).as_ref()))
	}
}
