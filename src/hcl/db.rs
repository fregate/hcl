use self::version::Version;

use super::Db;
use super::{Error, ErrorKind};
use crate::hcl::value::Value;
use crate::hcl::file_view::ViewValue;

use std::fs;
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;

mod version;

impl Db {
	pub fn new(mut file: fs::File) -> Result<Db, Error> {
		let mut size = file.metadata()?.len();
		if size == 0u64 {
			let v = Version::current().primitive().to_le_bytes();
			return match file.write(&v) {
				Ok(_) => Ok(Db {
					db: Impl {
						file,
						map: HashMap::new(),
					}
				}),
				Err(err) => Err(Error::from(err)),
			}
		}

		// read version
		let mut buff = [0u8; 2];
		file.read_exact(&mut buff)?;

		let version = Version::from(&buff);
		println!("found file {:?}", version);
		if version != Version::current() {
			println!("migration process from version {:?} to verison {:?}...", version, Version::current());
			// migration
		}

		size = size - 2;
		while size > 0 {
			// todo: read 4k block, parse 4k block
			// read https://codecapsule.com/2014/10/18/implementing-a-key-value-store-part-7-optimizing-data-structures-for-ssds/
			let mut buff = [0u8; 1];
			file.read_exact(&mut buff)?;
			size = size - 1;
		}

		Ok(Db {
			db: Impl {
				file,
				map: HashMap::new(),
			}
		})
	}

	pub fn get<'a>(&self, key: &'a str) -> Result<Value, Error> {
		Err(Error::new(ErrorKind::Db, "'get' not implemented"))
	}

	pub fn put<'a>(&self, key: &'a str, value: Value) -> Result<&str, Error> {
		Err(Error::new(ErrorKind::Db, "'put' not implemented"))
	}

	pub fn delete<'a>(&self, key: &'a str) -> Result<(), Error> {
		Err(Error::new(ErrorKind::Db, "'delete' not implemented"))
	}
}

#[derive(Debug)]
pub struct Impl {
	file: fs::File,
	map: HashMap<String, ViewValue>,
}
