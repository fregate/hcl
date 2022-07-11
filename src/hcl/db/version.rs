
use std::convert::From;

#[derive(Debug)]
pub struct Version(u16);

impl Version {
	pub fn current() -> Version {
		Version(1u16)
	}

	pub fn primitive(&self) -> u16 {
		self.0
	}
}

impl From<&[u8; 2]> for Version {
	fn from(arr: &[u8; 2]) -> Self {
		let v = u16::from_le_bytes(*arr);
		Version(v)
	}
}

impl PartialEq for Version {
	fn eq(&self, other: &Version) -> bool {
		self.0 == other.0
	}
}
