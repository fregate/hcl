use super::Hcl;
use super::Db;
use super::Error;

impl Hcl {
	pub fn new() -> Hcl {
		Hcl
	}

	pub fn open<'a>(&self, file_path: &'a str) -> Result<Db, Error> {
		// Result::Err(Error::new(-1, "Something went wrong!"))
		Result::Ok(Db::new())
	}
}
