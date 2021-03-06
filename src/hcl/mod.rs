mod db;
mod error;
mod file_view;
mod hcl;
pub mod value;

#[derive(Debug)]
pub struct Hcl;

#[derive(Debug)]
pub struct Db {
	db: db::Impl
}

#[derive(Debug)]
pub enum ErrorKind {
	FileIO,
	Db,
	Parser,
}

#[derive(Debug)]
pub struct Error {
	kind: ErrorKind,
	what: String,
}
