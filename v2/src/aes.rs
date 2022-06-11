use aes_gcm::{Aes256Gcm, Key, Nonce}; // Or `Aes128Gcm`
use aes_gcm::aead::{Aead, NewAead};

fn main(){
let key = Key::from_slice(b"an example very very secret key.");
let cipher = Aes256Gcm::new(key);
let text="虐げられるものの言葉に耳を傾けよ";
let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
let byte=text.as_bytes();
let ciphertext = cipher.encrypt(nonce, byte.as_ref())
    .expect("encryption failure!"); // NOTE: handle this error to avoid panics!
println!("{:?}",ciphertext);

let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
    .expect("decryption failure!"); // NOTE: handle this error to avoid panics!

assert_eq!(&plaintext, "虐げられるものの言葉に耳を傾けよ".as_bytes());
println!("{:?}",String::from_utf8(plaintext).as_ref());
}

