use super::value::Value;

#[derive(Debug)]
pub struct View {
	pub offset: u64,
	pub size: u32, // consider values have to be less than 4GB
}

#[derive(Debug)]
pub enum ViewValue {
	Full(Value),
	View(View),
}
