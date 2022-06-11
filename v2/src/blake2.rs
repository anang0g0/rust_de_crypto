use blake2::{Blake2b512, Blake2s256, Digest};
use hex_literal::hex;

fn main(){
// create a Blake2b512 object
let mut hasher = Blake2b512::new();

// write input message
hasher.update(b"hello world");

// read hash digest and consume hasher
let res = hasher.finalize();
println!("{:0x}",res);

assert_eq!(res[..], hex!("021ced8799296ceca557832ab941a50b4a11f83478cf141f51f933f653ab9fbcc05a037cddbed06e309bf334942c4e58cdf1a46e237911ccd7fcf9787cbc7fd0")[..]);

// same example for Blake2s256:
let mut hasher = Blake2s256::new();
hasher.update(b"hello world");
let res = hasher.finalize();
println!("{:0x}",res);
assert_eq!(res[..], hex!("
    9aec6806794561107e594b1f6a8a6b0c92a0cba9acf5e5e93cca06f781813b0b
")[..]);
}