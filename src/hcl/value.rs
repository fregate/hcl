#[derive(Debug)]
pub enum Value {
	Nil,
	Int(i128, u128),
	Real(f64),
	Bool(bool),
	Words(String),
}
