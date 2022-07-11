mod db;
mod error;
mod hcl;
mod value;

#[derive(Debug)]
pub struct Hcl;

#[derive(Debug)]
pub struct Db;

#[derive(Debug)]
pub struct Error {
	code: i32,
	what: String,
}
