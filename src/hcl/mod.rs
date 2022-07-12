mod db;
mod error;
mod file_view;
mod hcl;
mod value;

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
}

#[derive(Debug)]
pub struct Error {
	kind: ErrorKind,
	what: String,
}
