extern crate base64;

use std::str;
use base64::{encode, decode};

fn run<E: std::convert::From<std::str::Utf8Error>>() -> Result<(),E> {
    let hello = b"hello rustaceans";
    let encoded = encode(hello);
    let decoded = decode(&encoded)?;

    println!("origin: {}", str::from_utf8(hello)?);
    println!("base64 encoded: {}", encoded);
    println!("back to origin: {}", str::from_utf8(&decoded)?);

    Ok(())
}

fn main() {
    run::<E>();
    println!("Hello, world!");
}
