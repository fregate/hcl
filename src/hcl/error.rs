use super::Error;

use std::io;
use std::convert::From;

impl Error {
	pub fn new<'a>(code: i32, what: &'a str) ->Error {
		Error{ code, what: String::from(what) }
	}
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error { code: -1, what: format!("{}", err) }
    }
}
