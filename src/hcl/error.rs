use super::Error;
use super::ErrorKind;

use std::io;
use std::convert::From;

impl Error {
	pub fn new<'a>(kind: ErrorKind, what: &'a str) ->Error {
		Error{ kind, what: String::from(what) }
	}
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error { kind: ErrorKind::FileIO, what: format!("{}", err) }
    }
}
