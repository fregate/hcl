pub mod hcl;

use std::path;

fn main() {
    let hcl = hcl::Hcl::new();

    let db = hcl.open(path::Path::new("db.hcl")).unwrap();
    println!("db: {:?}", db);
    let val = db.get("aaa");
    println!("val: {:?}", val);
}
