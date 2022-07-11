use super::Db;
use super::Error;
use crate::hcl::value::Value;
use crate::hcl::file_view::View;

use std::fs;
use std::collections::HashMap;
use std::io::Read;

impl Db {
	pub fn new(mut file: fs::File) -> Result<Db, Error> {
		if file.metadata()?.len() == 0u64 {
			return Ok(Db {
				db: Impl {
					file,
					map: HashMap::new()
				}
			})
		}

		// read version
		let mut buff = [0u8; 4];
		if let Err(err) = file.read_exact(&mut buff) {
			return Err(Error::new(-3, format!("{}", err).as_ref()))
		}

		Ok(Db {
			db: Impl {
				file,
				map: HashMap::new()
			}
		})
	}

	pub fn get<'a>(&self, key: &'a str) -> Result<Value, Error> {
		Err(Error::new(-2, "'get' not implemented"))
	}

	pub fn put<'a>(&self, key: &'a str, value: Value) -> Result<&str, Error> {
		Err(Error::new(-2, "'put' not implemented"))
	}

	pub fn delete<'a>(&self, key: &'a str) -> Result<(), Error> {
		Err(Error::new(-2, "'delete' not implemented"))
	}
}

#[derive(Debug)]
pub struct Impl {
	file: fs::File,
	map: HashMap<String, View>
}
