


extern crate base64;
use std::str;
use base64::{encode, decode};
fn main() {
let hello = "日本語".as_bytes();

println!("何か入力を");
let mut data = String::new();
std::io::stdin().read_line(&mut data).ok();
println!("{}", data);

let encoded = encode(data.as_bytes());
let decoded = decode(&encoded).unwrap();

print!("origin: {}", str::from_utf8(data.as_bytes()).unwrap());
println!("base64 encoded: {}", encoded);
println!("back to origin: {}", str::from_utf8(&decoded).unwrap());
}
