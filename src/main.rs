pub mod hcl;

fn main() {
    let hcl = hcl::Hcl::new();

    let db = hcl.open("file_path").unwrap();
    let res = db.get("aaa");
    println!("res: {:?}", res);
}
