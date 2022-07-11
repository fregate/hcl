use super::Db;
use super::Error;
use crate::hcl::value::Value;

impl Db {
	pub fn new() -> Db {
		Db
	}

	pub fn get<'a>(&self, key: &'a str) -> Result<Value, Error> {
		Result::Err(Error::new(-2, "'get' not implemented"))
	}

	pub fn put<'a>(&self, key: &'a str, value: Value) -> Result<&str, Error> {
		Result::Err(Error::new(-2, "'put' not implemented"))
	}

	pub fn delete<'a>(&self, key: &'a str) -> Result<(), Error> {
		Result::Err(Error::new(-2, "'delete' not implemented"))
	}
}
