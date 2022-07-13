pub mod hcl;

use std::path;

fn main() {
    let hcl = hcl::Hcl::new();

    let db = hcl.open(path::Path::new("db.hcl")).unwrap();
    println!("db: {:?}", db);

    let x = db.put("key", hcl::value::Value::Nil).unwrap();
    let y = db.put("key2", hcl::value::Value::Int(-1)).unwrap();
}
