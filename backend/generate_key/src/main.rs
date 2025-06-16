use jwt_simple::prelude::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let key = HS256Key::generate();
    let mut file = File::create("secret.key").expect("Can open file for output");
    file.write_all(&key.to_bytes()).expect("Can write output");
}
