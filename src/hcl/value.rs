#[derive(Debug)]
pub enum Value {
	Nil,
	Int(i64),
	Real(f64),
	Bool(bool),
	Words(String),
	Val(Box<Value>),
}
