//use sha3::{Digest, Sha3_256};
use sha3::{Digest, Keccak256};

fn main() {
    // create a SHAKE256 object
    //let mut hasher = Keccak256::default();
    // create a SHA3-256 object
    let mut hasher = Keccak256::new();

    // write input message
    hasher.update(b"abc");

    // read hash digest
    let result = hasher.finalize();
    println!("{:0x}",result);

    /*
    // write input message
    // updateの引数は[u8]
    hasher.update(b"hello world");

    // read hash digest
    let size: usize = 200;
    // 引数はusize
    // 返り値はBox<[u8]>
    let result = hasher.finalize_boxed(size);

    for i in 0..size {
        print!("{} ", result[i]);
        if i % 10 == 9 {
            println!("");
        }
    }
    */
    println!("");
}
