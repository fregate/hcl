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
pub struct Error {
	code: i32,
	what: String,
}
