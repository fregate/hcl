use self::version::Version;

use super::{Db, file_view};
use super::{Error, ErrorKind};
use crate::hcl::value::Value;
use crate::hcl::file_view::ViewValue;

use std::cell::{RefCell, RefMut, Ref};
use std::cmp::Ordering;
use std::rc::Rc;
use std::{fs, mem};
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};
use std::io::Write;

mod version;

const SIGNATURE: &str = "hcldb";
const SIGNATURE_TEMP: &str = "hcltb";
const FILE_MUTABLE_SUFFIX: &str = "mut";
// const FILE_MERGE_SUFFIX: &str = "merge";

impl Db {
	pub fn new(path: &str, mut file: fs::File) -> Result<Box<Db>, Error> {
		let mut size = file.metadata()?.len();
		if size == 0u64 {
			file.write(SIGNATURE.as_bytes())?;
			let v = Version::current().primitive().to_le_bytes();
			file.write(&v)?;

			let file = Rc::new(RefCell::new(file));
			return Ok(Box::new(Db {
					db: Impl {
							mutable_file: Rc::clone(&file),
							file: Rc::clone(&file),
							map: HashMap::new(),
							path: String::from(path),
						}
					}));
		}

		// read signature
		let mut buff = vec![0; SIGNATURE.len()];
		file.read_exact(&mut buff)?;
		let buff = std::str::from_utf8(&buff)?;
		if Ordering::Equal != SIGNATURE.cmp(buff) {
			return Err(Error::new(ErrorKind::Parser, "wrong file signature"));
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

		// File format.
		// key-value pair format:
		// key_size(u32);key_string([key_size]);value_type(u8);value(depends on value_type)
		// value sizes:
		// nil(0): 0
		// int(1): 8 (i64)
		// real(2): 8 (f64)
		// bool(3): 1 (u8)
		// string(4): size (4), string (size)
		// value(5): key-value (?) - parse recursive
		// file structure:
		// version(u16);key-pairs(?);eof
		let mut map = HashMap::new();
		size = size - (mem::size_of::<Version>() + SIGNATURE.len()) as u64;
		while size > 0 {
			// todo: read 4k block, parse 4k block
			// read https://codecapsule.com/2014/10/18/implementing-a-key-value-store-part-7-optimizing-data-structures-for-ssds/
			let key = parse_key(&mut file, &mut size)?;
			let value = parse_value(&mut file, &mut size)?;

			map.insert(key, value);
		}

		let mfile = make_temp_file(path)?;
		let mfile = Rc::new(RefCell::new(mfile));

		Ok(Box::new(Db {
			db: Impl {
				file: Rc::new(RefCell::new(file)),
				map,
				mutable_file: mfile,
				path: String::from(path),
			}
		}))
	}

	pub fn get<'a>(&self, key: &'a str) -> Result<Value, Error> {
		Err(Error::new(ErrorKind::Db, "'get' not implemented"))
	}

	pub fn put<'a>(&self, key: &'a str, value: &Value) -> Result<(), Error> {
		let mut file = self.db.mutable();
		put_key(&mut file, key)?;
		put_value(&mut file, value)
	}

	pub fn delete<'a>(&self, key: &'a str) -> Result<(), Error> {
		Err(Error::new(ErrorKind::Db, "'delete' not implemented"))
	}
}

fn make_temp_file(path: &str) -> Result<fs::File, Error> {
	let file = fs::File::create(format!("{}-{}", path, FILE_MUTABLE_SUFFIX).as_str())?;
	Ok(file)
}

#[derive(Debug)]
pub struct Impl {
	file: Rc<RefCell<fs::File>>,
	map: HashMap<String, ViewValue>,
	mutable_file: Rc<RefCell<fs::File>>,
	path: String,
}

impl Impl {
	fn mutable(&self) -> RefMut<fs::File> {
		self.mutable_file.borrow_mut()
	}

	fn immutable(&self) -> Ref<fs::File> {
		self.mutable_file.borrow()
	}

	fn main(&self) -> Ref<fs::File> {
		self.file.borrow()
	}
}

fn parse_key(file: &mut fs::File, size: &mut u64) -> Result<String, Error> {
	let mut buff = [0u8; 1];
	file.read_exact(&mut buff)?;
	*size = *size - 1;

	let key_size = u8::from_le_bytes(buff);
	let mut buff = vec![0; key_size as usize];
	file.read_exact(&mut buff)?;

	let string = String::from_utf8(buff.to_vec())?;
	*size = *size - key_size as u64;
	Ok(string)
}

fn put_key(file: &mut fs::File, key: &str) -> Result<(), Error> {
	if key.len() > u8::max_value() as usize {
		Err(Error::new(ErrorKind::Db, "key value too long"))
	} else {
		let size_arr = (key.len() as u8).to_le_bytes();
		file.write(&size_arr)?;
		file.write(key.as_bytes())?;
		Ok(())
	}
}

fn parse_value(file: &mut fs::File, size: &mut u64) -> Result<ViewValue, Error> {
	let mut buff = [0u8; 1];
	file.read_exact(&mut buff)?;
	*size = *size - 1;

	let val = match buff[0] {
		0 => ViewValue::Full(Value::Nil),
		1 => {
			let val_int = parse_value_int(file)?;
			*size = *size - 8;
			ViewValue::Full(val_int)
		},
		2 => {
			let val_real = parse_value_real(file)?;
			*size = *size - 8;
			ViewValue::Full(val_real)
		},
		3 => {
			let val_bool = parse_value_bool(file)?;
			*size = *size - 1;
			ViewValue::Full(val_bool)
		},
		4 => parse_value_words(file, size)?,
		5 => parse_value_recursive(file, size)?,
		_ => ViewValue::Full(Value::Nil),
	};

	Ok(val)
}

fn put_value(file: &mut fs::File, value: &Value) -> Result<(), Error> {
	match value {
		Value::Nil => put_value_nil(file),
		Value::Int(v) => put_value_int(file, *v),
		_ => Err(Error::new(ErrorKind::Parser, format!("unknown value kind: {:?}", value).as_str()))
	}
}

// fn parse_value_nil() {} - no implementation

fn put_value_nil(file: &mut fs::File) -> Result<(), Error> {
	let buff = [0u8; 1];
	file.write(&buff)?;
	Ok(())
}

fn parse_value_int(file: &mut fs::File) -> Result<Value, Error> {
	let mut buff = [0u8; 8];
	file.read_exact(&mut buff)?;
	Ok(Value::Int(i64::from_le_bytes(buff)))
}

fn put_value_int(file: &mut fs::File, value: i64) -> Result<(), Error> {
	let buff = 1u8.to_le_bytes();
	file.write(&buff)?;
	let buff = value.to_le_bytes();
	file.write(&buff)?;
	Ok(())
}

fn parse_value_real(file: &mut fs::File) -> Result<Value, Error> {
	let mut buff = [0u8; 8];
	file.read_exact(&mut buff)?;
	Ok(Value::Real(f64::from_le_bytes(buff)))
}

fn parse_value_bool(file: &mut fs::File) -> Result<Value, Error> {
	let mut buff = [0u8; 1];
	file.read_exact(&mut buff)?;
	Ok(Value::Bool(match buff[0] {
		0 => false,
		_ => true,
	}))
}

fn parse_value_words(file: &mut fs::File, size: &mut u64) -> Result<ViewValue, Error> {
	let mut buff = [0u8; 4];
	file.read_exact(&mut buff)?;
	*size = *size - 4;

	let val_size = u32::from_le_bytes(buff);
	let val = if val_size > 32 {
			file.seek(SeekFrom::Current(val_size as i64))?;
			ViewValue::View(file_view::View{ offset: *size, size: val_size})
		} else {
			let mut buff = vec![0; val_size as usize];
			file.read_exact(&mut buff)?;

			let string = String::from_utf8(buff.to_vec())?;
			*size = *size - val_size as u64;

			ViewValue::Full(Value::Words(string))
		};

	Ok(val)
}

fn parse_value_recursive(file: &mut fs::File, size: &mut u64) -> Result<ViewValue, Error> {
	Err(Error::new(ErrorKind::Parser, "'parse_value_recursive' not implemented"))
}
