use super::Error;

impl Error {
	pub fn new<'a>(code: i32, what: &'a str) ->Error {
		Error{ code, what: String::from(what) }
	}
}
