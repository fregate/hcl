
#[derive(Debug)]
pub struct View {
	offset: u64,
	size: u32, // consider values have to be less than 4GB
}

#[derive(Debug)]
pub enum ViewValue {
	Value,
	View,
}
