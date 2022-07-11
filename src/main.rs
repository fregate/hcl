pub mod hcl;

use std::path;

fn main() {
    let hcl = hcl::Hcl::new();

    let db = hcl.open(path::Path::new("db.hcl")).unwrap();
    let res = db.get("aaa");
    println!("res: {:?}", res);
}
