#![allow(non_snake_case)]
use base64::{decode, encode};
use rand::prelude::*;
//use rand::rngs::adapter::ReseedingRng;
use rand::{Rng, SeedableRng};
//use rand_chacha::ChaCha20Core;
use sha3::Sha3_256;
use sha3::{Digest, Keccak256};
use std::io::Write;
use std::{process::exit, str};
use ndarray::prelude::*;

/*
    Fisher-Yates shuffle による方法
    配列の要素をランダムシャッフルする
*/
fn random_shuffule(mut array: [u8; 256], size: u16, seed: &[u8]) -> [u8; 256] {
    //let _i: usize;
    let mut _a: usize;
    let mut _b: usize;
    let mut sead:[u8;32]=[123;32];
    for i in 0..seed.len(){
        sead[i]=seed[i];
    }
    //let seed: u64 = 1;
    //let mut rng2 = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed(sead);
    for _i in (1..size).rev() {
        _a = (_i) as usize;
        let _b = rng.gen::<u8>() % _i as u8; // 32バイトシードで再現あり
        (array[_a], array[_b as usize]) = (array[_b as usize], array[_a])
    }

    array
}
fn shuffule(mut array: [u8; 32], size: u16, seed: &[u8]) -> [u8; 32] {
    //let _i: usize;
    let mut _a: usize;
    let mut _b: usize;
    let mut sead:[u8;32]=[123;32];
    for i in 0..seed.len(){
        sead[i]=seed[i];
    }
    //let seed: u64 = 1;
    //let mut rng2 = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed(sead);
    for _i in (1..size).rev() {
        _a = (_i) as usize;
        let _b = rng.gen::<u8>() % _i as u8; // 32バイトシードで再現あり
        (array[_a], array[_b as usize]) = (array[_b as usize], array[_a])
    }

    array
}

fn s2b(test: &str) -> &[u8] {
    //    let test: &str = "Test";
    let bytes: &[u8] = test.as_bytes();
    // convert bytes => str

    //println!("{}", test);
    println!("{:?}", bytes);

    bytes
}

fn b2s(bytes: &[u8]) -> String {
    //let res = bytes.iter().map(|&s| s as char).collect::<String>();
    let _converted: String = String::from_utf8(bytes.to_vec()).unwrap();

    _converted
}

fn S2str(data: &String) -> &str {
    let v = &data[0..data.len()];
    //println!("{:?}",v);

    return v;
}

fn pappy(a: &[u8]) -> [u8; 256] {
    // create a SHA3-256 object
    let mut count = 0;
    let mut buf:[u8;32]=[0;32];
    let mut u2: [u8; 256] = [0; 256];

    for _i in 0..a.len() {
        buf[_i%32] ^= a[_i];
    }
    
    for _i in 0..8 {
        let mut hasher = Sha3_256::default();
        //me=hasher.clone();
        hasher.update(buf);
        // read hash digest
        let mut result = hasher.finalize();

        for _i in 0..32 {
            buf[_i] ^= result[_i];
            u2[count] = result[_i];
            //print!("{},",result[i]);
            count = count + 1;
        }
        //println!("");
    }

    u2
}

fn p2(a: &[u8]) -> [u8; 32] {
    // create a SHA3-256 object
    let mut count = a.len();
    let mut buf: [u8; 32] = [0; 32];
    let mut u2: [u8; 256] = [0; 256];


    for _i in 0..count {
        buf[_i%32] ^= a[_i];
    }
    for _i in 0..2 {
        let mut hasher = Sha3_256::default();
        //me=hasher.clone();
        hasher.update(buf);
        // read hash digest
        let result = hasher.finalize();

        for _i in 0..32 {
            buf[_i] ^= result[_i];
        }
        //println!("");
    }
    //let mut o:&[u8]=&buf;
    //println!("oh={:?}",o);
    

    buf
}

fn ctr(data: &String, a: &[u8; 256], mat: &Array2<u8>) -> String {
    /*
     * S-box transformation table
     */
    const S_BOX: [u8; 256] = [
        // 0     1     2     3     4     5     6     7     8     9     a     b     c     d     e     f
        0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76, // 0
        0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0, // 1
        0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15, // 2
        0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75, // 3
        0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84, // 4
        0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf, // 5
        0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8, // 6
        0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2, // 7
        0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73, // 8
        0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb, // 9
        0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79, // a
        0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08, // b
        0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a, // c
        0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e, // d
        0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf, // e
        0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16, // f
    ];

    let mut buf: [u8; 256] = [0; 256];
    let byte = decode(data).unwrap(); //data.as_bytes();
    let seed2: [u8; 32] = [17; 32];
    let mut rng2: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed2);

    println!("len = {}", byte.len());
    println!("origin: {}", str::from_utf8(data.as_bytes()).unwrap());
    let mut me: [u8; 256] = [0; 256];
    let cycle = rng2.gen_range(1..256);
    let nonce:[u8;256]=[0;256];

    let j = byte.len();
    let mut be=seed2.clone();
    let mut be2:[u8;256]=[13;256];
    //let mut result:[u8;256]=[17;256];
    //result=pappy(result);

    //println!("{:?}",mat);
    //exit(1);
    for _i in 0..3{
        be2=pappy(&be2);
    }
    
    let mut msg:[u8;256]=[0;256];
    for i in 0..j {
        buf[i] = rng2.gen_range(0..=255); //nonce[i].clone();//byte[i];
        msg[i]=byte[i].clone();
        
        //buf[i]^=be[(i+1)%32];
    }

    //result=pappy(result);
    //println!("{:?}",result);

    for _k in 0..16 {
        // buf[_k]^=be[_k];
        be2=pappy(&be2);  
        
        //println!("=={:?}",be);

         for _i in 0..j {

            buf[_i] = S_BOX[((buf[_i] % 16) + (buf[_i] >> 4) * 16) as usize];
             //buf[_i]^=be[_i%32];
             //buf[_i] = a[buf[_i] as usize] as u8;
             buf[_i]^=be2[_i%32];
             //buf[_i] = mat[[a[(16 * j + _i)%101] as usize, buf[_i] as usize]];
             //buf[_i] = INV_S_BOX[((buf[_i] % 16) + (buf[_i] >> 4) * 16) as usize];
             msg[_i]^=buf[_i];
             
         }
 
     }

     println!("encrypted = {:?},{}", &msg[0..j],j);


    let encoded = encode(&msg[0..j]);
    //let enc = encoded.clone();
    println!("cipher text:");
    println!("{:?}", encoded);
    //exit(1);
    
    encoded
}

fn ofb(encoded: &String, a: &[u8; 256], mat: &Array2<u8>) -> String {
    /*
     * S-box transformation table
     */
    const INV_S_BOX: [u8; 256] = [
        // 0     1     2     3     4     5     6     7     8     9     a     b     c     d     e     f
        0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb, // 0
        0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb, // 1
        0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e, // 2
        0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25, // 3
        0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92, // 4
        0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84, // 5
        0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06, // 6
        0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b, // 7
        0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73, // 8
        0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e, // 9
        0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b, // a
        0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4, // b
        0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f, // c
        0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef, // d
        0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61, // e
        0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d, // f
    ];

    let mut buf: [u8; 256] = [0; 256];
    //let byte = data.as_bytes();
    let seed2: [u8; 32] = [17; 32];
    let mut rng2: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed2);
    let mut decoded = decode(&encoded).unwrap();
    println!("len = {}", decoded.len());
    println!("origin: {}", str::from_utf8(encoded.as_bytes()).unwrap());
    let mut me: [u8; 256] = [0; 256];
    let cycle = rng2.gen_range(1..256);
    let nonce:[u8;256]=[0;256];
    let j = decoded.len();
    let mut be=seed2.clone();
    let mut be2:[u8;256]=[13;256];
    //let mut result:[u8;256]=[17;256];
    //result=pappy(result);

    //println!("{:?}",mat);
    //exit(1);
    for _i in 0..3{
        be2=pappy(&be2);
    }
    
    for i in 0..j {
        buf[i] = rng2.gen_range(0..=255); //nonce[i];//byte[i];

        //buf[i]^=be[(i+1)%32];
    }

    //result=pappy(result);
    //println!("{:?}",result);


    
    for _k in 0..16 {
        // buf[_k]^=be[_k];
        be2=pappy(&be2);  
        //println!("=={:?}",be);
        //let it=p2(&be);
         for _i in 0..j {

             buf[_i] = INV_S_BOX[((buf[_i] % 16) + (buf[_i] >> 4) * 16) as usize];
             //buf[_i]^=be[_i%32];
             //buf[_i] = a[buf[_i] as usize] as u8;
             buf[_i]^=be2[_i%32];
             //buf[_i] = mat[[a[(16 * j + _i)%101] as usize, buf[_i] as usize]];
             //buf[_i] = INV_S_BOX[((buf[_i] % 16) + (buf[_i] >> 4) * 16) as usize];
             decoded[_i]^=buf[_i];             
         }
 
     }

 
    println!("encrypted = {:?}", &decoded[0..j]);

    //let encoded = encode(&buf[0..j]);
    //let enc = encoded.clone();

    for i in 0..j {
        buf[i] = decoded[i];
        //buf[i]^=be[(i+1)%32];
    }

    println!("plain text:");
    println!("decrypted = {:?}", &buf[0..j]);
   // println!("{:?}", String::from_utf8(buf[0..j].to_vec()));
    match String::from_utf8(buf.to_vec()) {
        Err(_why) => {
            println!("復号できませんでした");
            "baka".to_string()
        }
        Ok(str) => encode(str),
    }

}

fn mlt(x:u16, y:u16)->u16
{

  if x == 0 || y == 0 
  {
    println!("is 0");
   return 0
  }

 ((x + y - 2) % (256 - 1)) + 1
}


fn mltn(mut n:u16, mut x:u16)-> u16
{
  let mut ret = 1;
  while n > 0
  {
    if (n % 2) == 1{
      ret = mlt(ret, x) // n の最下位bitが 1 ならば x^(2^i) をかける
    }
    x = mlt(x, x);
    n = (n>>1) // n を1bit 左にずらす
  }
  
  ret
}

//有限体の元の逆数
fn oinv( a:u16)->u16
{
    const gf:[u8;256] = [0, 1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205, 135, 19, 38, 76, 152, 45, 90, 180, 117, 234, 201, 143, 3, 6, 12, 24, 48, 96, 192, 157, 39, 78, 156, 37, 74, 148, 53, 106, 212, 181, 119, 238, 193, 159, 35, 70, 140, 5, 10, 20, 40, 80, 160, 93, 186, 105, 210, 185, 111, 222, 161, 95, 190, 97, 194, 153, 47, 94, 188, 101, 202, 137, 15, 30, 60, 120, 240, 253, 231, 211, 187, 107, 214, 177, 127, 254, 225, 223, 163, 91, 182, 113, 226, 217, 175, 67, 134, 17, 34, 68, 136, 13, 26, 52, 104, 208, 189, 103, 206, 129, 31, 62, 124, 248, 237, 199, 147, 59, 118, 236, 197, 151, 51, 102, 204, 133, 23, 46, 92, 184, 109, 218, 169, 79, 158, 33, 66, 132, 21, 42, 84, 168, 77, 154, 41, 82, 164, 85, 170, 73, 146, 57, 114, 228, 213, 183, 115, 230, 209, 191, 99, 198, 145, 63, 126, 252, 229, 215, 179, 123, 246, 241, 255, 227, 219, 171, 75, 150, 49, 98, 196, 149, 55, 110, 220, 165, 87, 174, 65, 130, 25, 50, 100, 200, 141, 7, 14, 28, 56, 112, 224, 221, 167, 83, 166, 81, 162, 89, 178, 121, 242, 249, 239, 195, 155, 43, 86, 172, 69, 138, 9, 18, 36, 72, 144, 61, 122, 244, 245, 247, 243, 251, 235, 203, 139, 11, 22, 44, 88, 176, 125, 250, 233, 207, 131, 27, 54, 108, 216, 173, 71, 142];
    const fg:[u8;256] = [0, 1, 2, 26, 3, 51, 27, 199, 4, 224, 52, 239, 28, 105, 200, 76, 5, 101, 225, 15, 53, 142, 240, 130, 29, 194, 106, 249, 201, 9, 77, 114, 6, 139, 102, 48, 226, 37, 16, 34, 54, 148, 143, 219, 241, 19, 131, 70, 30, 182, 195, 126, 107, 40, 250, 186, 202, 155, 10, 121, 78, 229, 115, 167, 7, 192, 140, 99, 103, 222, 49, 254, 227, 153, 38, 180, 17, 146, 35, 137, 55, 209, 149, 207, 144, 151, 220, 190, 242, 211, 20, 93, 132, 57, 71, 65, 31, 67, 183, 164, 196, 73, 127, 111, 108, 59, 41, 85, 251, 134, 187, 62, 203, 95, 156, 160, 11, 22, 122, 44, 79, 213, 230, 173, 116, 244, 168, 88, 8, 113, 193, 248, 141, 129, 100, 14, 104, 75, 223, 238, 50, 198, 255, 25, 228, 166, 154, 120, 39, 185, 181, 125, 18, 69, 147, 218, 36, 33, 138, 47, 56, 64, 210, 92, 150, 189, 208, 206, 145, 136, 152, 179, 221, 253, 191, 98, 243, 87, 212, 172, 21, 43, 94, 159, 133, 61, 58, 84, 72, 110, 66, 163, 32, 46, 68, 217, 184, 124, 165, 119, 197, 24, 74, 237, 128, 13, 112, 247, 109, 162, 60, 83, 42, 158, 86, 171, 252, 97, 135, 178, 188, 205, 63, 91, 204, 90, 96, 177, 157, 170, 161, 82, 12, 246, 23, 236, 123, 118, 45, 216, 80, 175, 214, 234, 231, 232, 174, 233, 117, 215, 245, 235, 169, 81, 89, 176];
    let mut i:i32=0;

    if a == 0{
        return 0;
    }
    for i in 0..256
    {
        if gf[mlt(fg[a as usize] as u16, i) as usize] == 1{
           return i as u16; 
    }
    }
    println!("no return ");
    //  exit (1);
    0
}


// invert of integer
fn iinv( mut a:u16,  n:u16)->u16
{
    let mut d=0;
    let mut q=0;
    let mut t=0;
    let mut r=0;
    let mut x=0;
    let mut s=0;
    let mut gcd=0;

    x = 0;
    s = 1;

    d = n;
    while  a != 0
    {
        q = d / a;
        r = d % a;
        d = a;
        a = r;
        t = x - q * s;
        x = s;
        s = t;
    }
    gcd = d;

    ((x + n) % (n / d))
}


fn lot(mut z:[u8;32])->[u8;32]{
    let mut tmp:u8=0;

    for i in 0..32{    
        if i+1 < 32{
        tmp=z[i];
        z[i]=z[(i+1)];
        z[i+1]=tmp;
        }
    }
z
}

fn rot(mut z:[u8;32])->[u8;32]{
    let mut tmp:u8=0;

    for i in (0..32).rev(){
        //tmp=z[i];
        if (i)>0 {
        tmp=z[i];
        z[(i)%32]=z[(i-1)%32];
        z[(i-1)%32]=tmp;
        }
        if i == 0{
            z[0]=tmp;
        }
    }
    z
}

fn lot2(mut z:[u8;16])->[u8;16]{
    let mut tmp:u8=0;

    for i in 0..16{    
        if i+1 < 16{
        tmp=z[i];
        z[i]=z[(i+1)];
        z[i+1]=tmp;
        }
    }
z
}

fn rot2(mut z:[u8;16])->[u8;16]{
    let mut tmp:u8=0;

    for i in (0..16).rev(){
        //tmp=z[i];
        if (i)>0 {
        tmp=z[i];
        z[(i)%16]=z[(i-1)%16];
        z[(i-1)%16]=tmp;
        }
        if i == 0{
            z[0]=tmp;
        }
    }
    z
}

fn Lot(mut z:[u8;256])->[u8;256]{
    let mut tmp:u8=0;

    for i in 0..256{    
        if i+1 < 256{
        tmp=z[i];
        z[i]=z[(i+1)];
        z[i+1]=tmp;
        }
    }
z
}

fn Rot(mut z:[u8;256])->[u8;256]{
    let mut tmp:u8=0;

    for i in (0..256).rev(){
        //tmp=z[i];
        if (i)>0 {
        tmp=z[i];
        z[(i)%16]=z[(i-1)%256];
        z[(i-1)%16]=tmp;
        }
        if i == 0{
            z[0]=tmp;
        }
    }
    z
}

fn tenchi(m2:Array2<u8>)->Array2<u8>{
    let mut mat: Array2<u8> = Array2::zeros((16, 16));
    for i in 0..16{
        for j in 0..16{
            mat[[i,j]]=m2[[j,i]];
        }
    }

    mat
}

fn v2m(m:[u8;256])->Array2<u8>{
    let mut mat: Array2<u8> = Array2::zeros((16, 16));
    for i in 0..16{
        for j in 0..16{
            mat[[j,i]]=m[i*16+j];
        }
    }

    mat
}

fn v2t(m:[u8;256])->Array2<u8>{
    let mut mat: Array2<u8> = Array2::zeros((16, 16));
    for i in 0..16{
        for j in 0..16{
            mat[[j,i]]=m[i*16+j];
        }
    }

    mat
}

fn m2v(m2:Array2<u8>)->[u8;256]{
let mut r1:[u8;256]=[0;256];

for i in 0..16{
    for j in 0..16{
        r1[i*16+j]=m2[[j,i]];
    }
}

r1
}


fn shift(sf:Array2<u8>)->Array2<u8>{
    let mut v:[u8;16]=[0;16];
    let mut mat: Array2<u8> = Array2::zeros((16, 16));

    for j in 0..16{
    for i in 0..16{
        v[i]=sf[[j,i]];
    }
    for ii in 0..j{
    v=lot2(v);
    }
    for k in 0..16{
    mat[[j,k]]=v[k];
    }
}

mat
}

fn rev_shift(sf:Array2<u8>)->Array2<u8>{
    let mut v:[u8;16]=[0;16];
    let mut mat: Array2<u8> = Array2::zeros((16, 16));

    for j in 0..16{
    for i in 0..16{
        v[i]=sf[[j,i]];
    }
    for ii in 0..j{
    v=rot2(v);
    }
    for k in 0..16{
    mat[[j,k]]=v[k];
    }
}

mat
}


fn mulm(ma:Array2<u8>)->Array2<u8>{
    let mut nn:Array2<u8>=Array2::zeros((16,16));
    const  gf:[u8;256] = [0, 1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205, 135, 19, 38, 76, 152, 45, 90, 180, 117, 234, 201, 143, 3, 6, 12, 24, 48, 96, 192, 157, 39, 78, 156, 37, 74, 148, 53, 106, 212, 181, 119, 238, 193, 159, 35, 70, 140, 5, 10, 20, 40, 80, 160, 93, 186, 105, 210, 185, 111, 222, 161, 95, 190, 97, 194, 153, 47, 94, 188, 101, 202, 137, 15, 30, 60, 120, 240, 253, 231, 211, 187, 107, 214, 177, 127, 254, 225, 223, 163, 91, 182, 113, 226, 217, 175, 67, 134, 17, 34, 68, 136, 13, 26, 52, 104, 208, 189, 103, 206, 129, 31, 62, 124, 248, 237, 199, 147, 59, 118, 236, 197, 151, 51, 102, 204, 133, 23, 46, 92, 184, 109, 218, 169, 79, 158, 33, 66, 132, 21, 42, 84, 168, 77, 154, 41, 82, 164, 85, 170, 73, 146, 57, 114, 228, 213, 183, 115, 230, 209, 191, 99, 198, 145, 63, 126, 252, 229, 215, 179, 123, 246, 241, 255, 227, 219, 171, 75, 150, 49, 98, 196, 149, 55, 110, 220, 165, 87, 174, 65, 130, 25, 50, 100, 200, 141, 7, 14, 28, 56, 112, 224, 221, 167, 83, 166, 81, 162, 89, 178, 121, 242, 249, 239, 195, 155, 43, 86, 172, 69, 138, 9, 18, 36, 72, 144, 61, 122, 244, 245, 247, 243, 251, 235, 203, 139, 11, 22, 44, 88, 176, 125, 250, 233, 207, 131, 27, 54, 108, 216, 173, 71, 142];
    const  fg:[u8;256] = [0, 1, 2, 26, 3, 51, 27, 199, 4, 224, 52, 239, 28, 105, 200, 76, 5, 101, 225, 15, 53, 142, 240, 130, 29, 194, 106, 249, 201, 9, 77, 114, 6, 139, 102, 48, 226, 37, 16, 34, 54, 148, 143, 219, 241, 19, 131, 70, 30, 182, 195, 126, 107, 40, 250, 186, 202, 155, 10, 121, 78, 229, 115, 167, 7, 192, 140, 99, 103, 222, 49, 254, 227, 153, 38, 180, 17, 146, 35, 137, 55, 209, 149, 207, 144, 151, 220, 190, 242, 211, 20, 93, 132, 57, 71, 65, 31, 67, 183, 164, 196, 73, 127, 111, 108, 59, 41, 85, 251, 134, 187, 62, 203, 95, 156, 160, 11, 22, 122, 44, 79, 213, 230, 173, 116, 244, 168, 88, 8, 113, 193, 248, 141, 129, 100, 14, 104, 75, 223, 238, 50, 198, 255, 25, 228, 166, 154, 120, 39, 185, 181, 125, 18, 69, 147, 218, 36, 33, 138, 47, 56, 64, 210, 92, 150, 189, 208, 206, 145, 136, 152, 179, 221, 253, 191, 98, 243, 87, 212, 172, 21, 43, 94, 159, 133, 61, 58, 84, 72, 110, 66, 163, 32, 46, 68, 217, 184, 124, 165, 119, 197, 24, 74, 237, 128, 13, 112, 247, 109, 162, 60, 83, 42, 158, 86, 171, 252, 97, 135, 178, 188, 205, 63, 91, 204, 90, 96, 177, 157, 170, 161, 82, 12, 246, 23, 236, 123, 118, 45, 216, 80, 175, 214, 234, 231, 232, 174, 233, 117, 215, 245, 235, 169, 81, 89, 176];
    let mut me:[u8;256]=[0;256];
    let mut van=arr2(&[
        [2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17],
        [4,5,16,17,20,21,64,65,68,69,80,81,84,85,29,28],
        [8,15,64,85,120,107,58,115,146,221,231,186,127,36,205,193],
        [16,17,29,28,13,12,205,204,221,220,208,209,192,193,76,77],
        [32,51,116,108,46,36,38,226,1,215,169,116,244,59,180,233],
        [64,85,205,193,228,252,45,161,10,146,191,62,241,100,143,223],
        [128,255,19,226,98,206,117,192,68,79,87,43,199,38,24,174],
        [29,28,76,77,81,80,143,142,146,147,195,194,222,223,157,156],
        [58,36,45,100,251,173,12,138,221,68,125,179,64,145,37,169],
        [116,108,180,233,32,100,96,174,1,214,38,180,167,44,106,235],
        [232,180,234,106,192,33,39,183,10,153,181,151,164,185,238,253],
        [205,193,143,223,186,231,37,102,68,10,47,61,182,169,70,150],
        [135,94,6,132,187,143,53,113,146,78,217,60,74,89,20,3],
        [19,226,24,174,189,138,181,222,221,152,197,49,203,96,93,51],
        [38,59,96,44,169,145,193,96,1,1,85,96,150,26,185,36],
        [76,77,157,156,209,208,70,71,10,11,219,218,151,150,95,94]
        ]);
//exit(1);

    for i in 0..16{
        for j in 0..16{
            for k in 0..16{
                nn[[i,j]]^=gf[mlt(fg[van[[i,k]] as usize] as u16,fg[ma[[k,j]]as usize] as u16) as usize];
            }
        }
    }
nn
}


fn invm(ma:Array2<u8>)->Array2<u8>{
    let mut nn:Array2<u8>=Array2::zeros((16,16));
    const  gf:[u8;256] = [0, 1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205, 135, 19, 38, 76, 152, 45, 90, 180, 117, 234, 201, 143, 3, 6, 12, 24, 48, 96, 192, 157, 39, 78, 156, 37, 74, 148, 53, 106, 212, 181, 119, 238, 193, 159, 35, 70, 140, 5, 10, 20, 40, 80, 160, 93, 186, 105, 210, 185, 111, 222, 161, 95, 190, 97, 194, 153, 47, 94, 188, 101, 202, 137, 15, 30, 60, 120, 240, 253, 231, 211, 187, 107, 214, 177, 127, 254, 225, 223, 163, 91, 182, 113, 226, 217, 175, 67, 134, 17, 34, 68, 136, 13, 26, 52, 104, 208, 189, 103, 206, 129, 31, 62, 124, 248, 237, 199, 147, 59, 118, 236, 197, 151, 51, 102, 204, 133, 23, 46, 92, 184, 109, 218, 169, 79, 158, 33, 66, 132, 21, 42, 84, 168, 77, 154, 41, 82, 164, 85, 170, 73, 146, 57, 114, 228, 213, 183, 115, 230, 209, 191, 99, 198, 145, 63, 126, 252, 229, 215, 179, 123, 246, 241, 255, 227, 219, 171, 75, 150, 49, 98, 196, 149, 55, 110, 220, 165, 87, 174, 65, 130, 25, 50, 100, 200, 141, 7, 14, 28, 56, 112, 224, 221, 167, 83, 166, 81, 162, 89, 178, 121, 242, 249, 239, 195, 155, 43, 86, 172, 69, 138, 9, 18, 36, 72, 144, 61, 122, 244, 245, 247, 243, 251, 235, 203, 139, 11, 22, 44, 88, 176, 125, 250, 233, 207, 131, 27, 54, 108, 216, 173, 71, 142];
    const  fg:[u8;256] = [0, 1, 2, 26, 3, 51, 27, 199, 4, 224, 52, 239, 28, 105, 200, 76, 5, 101, 225, 15, 53, 142, 240, 130, 29, 194, 106, 249, 201, 9, 77, 114, 6, 139, 102, 48, 226, 37, 16, 34, 54, 148, 143, 219, 241, 19, 131, 70, 30, 182, 195, 126, 107, 40, 250, 186, 202, 155, 10, 121, 78, 229, 115, 167, 7, 192, 140, 99, 103, 222, 49, 254, 227, 153, 38, 180, 17, 146, 35, 137, 55, 209, 149, 207, 144, 151, 220, 190, 242, 211, 20, 93, 132, 57, 71, 65, 31, 67, 183, 164, 196, 73, 127, 111, 108, 59, 41, 85, 251, 134, 187, 62, 203, 95, 156, 160, 11, 22, 122, 44, 79, 213, 230, 173, 116, 244, 168, 88, 8, 113, 193, 248, 141, 129, 100, 14, 104, 75, 223, 238, 50, 198, 255, 25, 228, 166, 154, 120, 39, 185, 181, 125, 18, 69, 147, 218, 36, 33, 138, 47, 56, 64, 210, 92, 150, 189, 208, 206, 145, 136, 152, 179, 221, 253, 191, 98, 243, 87, 212, 172, 21, 43, 94, 159, 133, 61, 58, 84, 72, 110, 66, 163, 32, 46, 68, 217, 184, 124, 165, 119, 197, 24, 74, 237, 128, 13, 112, 247, 109, 162, 60, 83, 42, 158, 86, 171, 252, 97, 135, 178, 188, 205, 63, 91, 204, 90, 96, 177, 157, 170, 161, 82, 12, 246, 23, 236, 123, 118, 45, 216, 80, 175, 214, 234, 231, 232, 174, 233, 117, 215, 245, 235, 169, 81, 89, 176];
    let mut me:[u8;256]=[0;256];
    let mut inv=arr2(&[
        [ 77,240, 60, 82,  7,193,162, 86, 35,152, 75,172, 81,161,217,226],
        [175,124,143,135,174,152,138,216, 17, 90, 99,116,121,137,217,188],
        [249,217,194,104, 20,211, 43, 14,232,247, 57,132,236,246,126,145],
        [199, 96,228,206,231, 58,  3, 71, 21, 16, 17,182,196,222,126,237],
        [ 99,142,  6, 79, 53,201,227,150, 76, 42, 59,178,244, 30,198, 33],
        [137,139,245, 59,  9, 31,203, 88,171, 39, 19,112,220, 54,198, 69],
        [136, 31,222,168,235,141, 72,237,253,216,255,159,147, 28,105,160],
        [246,180, 58, 58, 99,138, 96,150,185, 19,215, 56,187, 52,105,223],
        [ 25,100,253,111,223,106,165, 81,223,250,214,129,160,223,250, 25],
        [137,116,204,180,152, 82,141, 53,129,219,254,214,136,247,250, 64],
        [ 61, 23,115, 62,141, 81,183, 90, 50,120, 12,  7,181, 32,245,221],
        [137,243,215,125,144,217,159, 65,163,124, 36,186,157,  8,245,204],
        [159,110,184,222,  7,245,230, 69,153, 17, 98,141,193,164, 33, 50],
        [ 53,255,201,113,213, 66,206,126, 18,255, 74,192,233,140, 33,161],
        [105,105,118,120,254,254,254,165, 38, 38, 38, 38, 38, 38, 38,232],
        [118,238,118,246,254,254,254,140, 38, 38, 38, 38, 38, 38, 38,215]
        ]);
//exit(1);

    for i in 0..16{
        for j in 0..16{
            for k in 0..16{
                nn[[i,j]]^=gf[mlt(fg[inv[[i,k]] as usize] as u16,fg[ma[[k,j]]as usize] as u16) as usize];
            }
        }
    }
nn
}


fn permute(u:[u8;32],mut u2:[u8;32] ,n:i32)->[u8;32]{
    let mut tmp:[u8;32]=[0;32];
    let mut nk:[u8;32]=[0;32];
    let mut nk2:[u8;32]=[0;32];

    for j in 0..n{
    for i in 0..32{
        tmp[i]=u[u2[i] as usize];
    }
    for i in 0..32{
        u2[i]=tmp[i];
    }
    }

    u2
}

fn rebirth(inv:[u8;32],mut nk:[u8;32],n:i32)->[u8;32]{
    let mut tmp:[u8;32]=[0;32];

    for j in 0..n{
    for i in 0..32{
        tmp[i]=inv[nk[i] as usize];
    }
    for k in 0..32{
        nk[k]=tmp[k];
    }
    }

    nk
}


fn enc(data: &String, a: &[u8; 256], mat: &Array2<u8>,seed2:&[u8]) -> String {
    /*
     * S-box transformation table
     */
    const S_BOX: [u8; 256] = [
        // 0     1     2     3     4     5     6     7     8     9     a     b     c     d     e     f
        0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76, // 0
        0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0, // 1
        0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15, // 2
        0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75, // 3
        0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84, // 4
        0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf, // 5
        0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8, // 6
        0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2, // 7
        0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73, // 8
        0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb, // 9
        0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79, // a
        0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08, // b
        0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a, // c
        0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e, // d
        0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf, // e
        0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16, // f
    ];


    const  gf:[u8;256] = [0, 1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205, 135, 19, 38, 76, 152, 45, 90, 180, 117, 234, 201, 143, 3, 6, 12, 24, 48, 96, 192, 157, 39, 78, 156, 37, 74, 148, 53, 106, 212, 181, 119, 238, 193, 159, 35, 70, 140, 5, 10, 20, 40, 80, 160, 93, 186, 105, 210, 185, 111, 222, 161, 95, 190, 97, 194, 153, 47, 94, 188, 101, 202, 137, 15, 30, 60, 120, 240, 253, 231, 211, 187, 107, 214, 177, 127, 254, 225, 223, 163, 91, 182, 113, 226, 217, 175, 67, 134, 17, 34, 68, 136, 13, 26, 52, 104, 208, 189, 103, 206, 129, 31, 62, 124, 248, 237, 199, 147, 59, 118, 236, 197, 151, 51, 102, 204, 133, 23, 46, 92, 184, 109, 218, 169, 79, 158, 33, 66, 132, 21, 42, 84, 168, 77, 154, 41, 82, 164, 85, 170, 73, 146, 57, 114, 228, 213, 183, 115, 230, 209, 191, 99, 198, 145, 63, 126, 252, 229, 215, 179, 123, 246, 241, 255, 227, 219, 171, 75, 150, 49, 98, 196, 149, 55, 110, 220, 165, 87, 174, 65, 130, 25, 50, 100, 200, 141, 7, 14, 28, 56, 112, 224, 221, 167, 83, 166, 81, 162, 89, 178, 121, 242, 249, 239, 195, 155, 43, 86, 172, 69, 138, 9, 18, 36, 72, 144, 61, 122, 244, 245, 247, 243, 251, 235, 203, 139, 11, 22, 44, 88, 176, 125, 250, 233, 207, 131, 27, 54, 108, 216, 173, 71, 142];
    const  fg:[u8;256] = [0, 1, 2, 26, 3, 51, 27, 199, 4, 224, 52, 239, 28, 105, 200, 76, 5, 101, 225, 15, 53, 142, 240, 130, 29, 194, 106, 249, 201, 9, 77, 114, 6, 139, 102, 48, 226, 37, 16, 34, 54, 148, 143, 219, 241, 19, 131, 70, 30, 182, 195, 126, 107, 40, 250, 186, 202, 155, 10, 121, 78, 229, 115, 167, 7, 192, 140, 99, 103, 222, 49, 254, 227, 153, 38, 180, 17, 146, 35, 137, 55, 209, 149, 207, 144, 151, 220, 190, 242, 211, 20, 93, 132, 57, 71, 65, 31, 67, 183, 164, 196, 73, 127, 111, 108, 59, 41, 85, 251, 134, 187, 62, 203, 95, 156, 160, 11, 22, 122, 44, 79, 213, 230, 173, 116, 244, 168, 88, 8, 113, 193, 248, 141, 129, 100, 14, 104, 75, 223, 238, 50, 198, 255, 25, 228, 166, 154, 120, 39, 185, 181, 125, 18, 69, 147, 218, 36, 33, 138, 47, 56, 64, 210, 92, 150, 189, 208, 206, 145, 136, 152, 179, 221, 253, 191, 98, 243, 87, 212, 172, 21, 43, 94, 159, 133, 61, 58, 84, 72, 110, 66, 163, 32, 46, 68, 217, 184, 124, 165, 119, 197, 24, 74, 237, 128, 13, 112, 247, 109, 162, 60, 83, 42, 158, 86, 171, 252, 97, 135, 178, 188, 205, 63, 91, 204, 90, 96, 177, 157, 170, 161, 82, 12, 246, 23, 236, 123, 118, 45, 216, 80, 175, 214, 234, 231, 232, 174, 233, 117, 215, 245, 235, 169, 81, 89, 176];
   
    let mut buf: [u8; 256] = [0; 256];
   // let mut byte = data.as_bytes();
    let mut byte = decode(&data).unwrap();
    //let seed2 = "kotobahairanai".as_bytes();

    let mut seed:[u8;32]=[0;32];
    //seed=p2(&seed2);
    let mut rng2: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed);
    println!("len = {}", byte.len());
    println!("origin: {}", str::from_utf8(data.as_bytes()).unwrap());
    let mut me: [u8; 256] = [11; 256];
    let cycle = rng2.gen_range(1..256);
    let mut count=0;
    let j = byte.len();
    let mut be=seed.clone();
    let mut it:[u8;256]=[0;256];
    //let mut result:[u8;256]=[17;256];
    //result=pappy(result);
    for i in 0..256{
    it[i]=a[i];
    }
    for i in 0..32{
        seed[i]=i as u8;
    }
   seed= shuffule(seed,32,&be);
    //println!("{:?}",mat);
    //exit(1);
    for _i in 0..3{
        be=p2(&be);
        println!("{:?}",be);
    }
    
    for i in 0..j {
        buf[i] = byte[i];
        //buf[i]^=be[(i+1)%32];
    }
let mut nk:[u8;32]=[0;32];
for i in 0..32{
    nk[i]=i as u8;
}
    //result=pappy(result);
    //println!("{:?}",result);

    for _k in 0..16 {
        //it=pappy(&it);
        //be=p2(&be);
       // buf[_k]^=be[_k];
       //println!("ii={:?}",&buf[0..j]);
       for _i in 0..j {
            buf[_i]^=be[nk[_i%32] as usize]; //gf[mlt(fg[be[_i%32] as usize] as u16,fg[buf[_i] as usize] as u16) as usize]; //+count;  
            //buf[_i]=gf[buf[_i] as usize];
            buf[_i] = S_BOX[((buf[_i] % 16) + (buf[_i] >> 4) * 16) as usize];
            buf[_i] = a[buf[_i] as usize] as u8;
            buf[_i] = mat[[  it[((16 * _k + _i))%256 as usize] as usize, (buf[_i as usize]) as usize]] ;
            
        }
        //seed=lot(seed);
        nk=permute(seed,nk,1);
        println!("{:?}",nk);
    }
//exit(1);

    println!("encrypted = {:?}", &buf[0..j]);

    let encoded = encode(&buf[0..j]);
    let enc = encoded.clone();
    println!("cipher text:");
    println!("{:?}", encoded);
    //exit(1);

    encoded
}

fn dec(encoded: &String, a: &[u8; 256], mat: &Array2<u8>,seed2:&[u8]) -> String {
    let mut buf: [u8; 256] = [0; 256];
    /*
     * Inverse S-box transformation table
     */
    const INV_S_BOX: [u8; 256] = [
        // 0     1     2     3     4     5     6     7     8     9     a     b     c     d     e     f
        0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb, // 0
        0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb, // 1
        0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e, // 2
        0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25, // 3
        0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92, // 4
        0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84, // 5
        0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06, // 6
        0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b, // 7
        0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73, // 8
        0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e, // 9
        0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b, // a
        0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4, // b
        0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f, // c
        0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef, // d
        0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61, // e
        0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d, // f
    ];

    const  gf:[u8;256] = [0, 1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205, 135, 19, 38, 76, 152, 45, 90, 180, 117, 234, 201, 143, 3, 6, 12, 24, 48, 96, 192, 157, 39, 78, 156, 37, 74, 148, 53, 106, 212, 181, 119, 238, 193, 159, 35, 70, 140, 5, 10, 20, 40, 80, 160, 93, 186, 105, 210, 185, 111, 222, 161, 95, 190, 97, 194, 153, 47, 94, 188, 101, 202, 137, 15, 30, 60, 120, 240, 253, 231, 211, 187, 107, 214, 177, 127, 254, 225, 223, 163, 91, 182, 113, 226, 217, 175, 67, 134, 17, 34, 68, 136, 13, 26, 52, 104, 208, 189, 103, 206, 129, 31, 62, 124, 248, 237, 199, 147, 59, 118, 236, 197, 151, 51, 102, 204, 133, 23, 46, 92, 184, 109, 218, 169, 79, 158, 33, 66, 132, 21, 42, 84, 168, 77, 154, 41, 82, 164, 85, 170, 73, 146, 57, 114, 228, 213, 183, 115, 230, 209, 191, 99, 198, 145, 63, 126, 252, 229, 215, 179, 123, 246, 241, 255, 227, 219, 171, 75, 150, 49, 98, 196, 149, 55, 110, 220, 165, 87, 174, 65, 130, 25, 50, 100, 200, 141, 7, 14, 28, 56, 112, 224, 221, 167, 83, 166, 81, 162, 89, 178, 121, 242, 249, 239, 195, 155, 43, 86, 172, 69, 138, 9, 18, 36, 72, 144, 61, 122, 244, 245, 247, 243, 251, 235, 203, 139, 11, 22, 44, 88, 176, 125, 250, 233, 207, 131, 27, 54, 108, 216, 173, 71, 142];
    const  fg:[u8;256] = [0, 1, 2, 26, 3, 51, 27, 199, 4, 224, 52, 239, 28, 105, 200, 76, 5, 101, 225, 15, 53, 142, 240, 130, 29, 194, 106, 249, 201, 9, 77, 114, 6, 139, 102, 48, 226, 37, 16, 34, 54, 148, 143, 219, 241, 19, 131, 70, 30, 182, 195, 126, 107, 40, 250, 186, 202, 155, 10, 121, 78, 229, 115, 167, 7, 192, 140, 99, 103, 222, 49, 254, 227, 153, 38, 180, 17, 146, 35, 137, 55, 209, 149, 207, 144, 151, 220, 190, 242, 211, 20, 93, 132, 57, 71, 65, 31, 67, 183, 164, 196, 73, 127, 111, 108, 59, 41, 85, 251, 134, 187, 62, 203, 95, 156, 160, 11, 22, 122, 44, 79, 213, 230, 173, 116, 244, 168, 88, 8, 113, 193, 248, 141, 129, 100, 14, 104, 75, 223, 238, 50, 198, 255, 25, 228, 166, 154, 120, 39, 185, 181, 125, 18, 69, 147, 218, 36, 33, 138, 47, 56, 64, 210, 92, 150, 189, 208, 206, 145, 136, 152, 179, 221, 253, 191, 98, 243, 87, 212, 172, 21, 43, 94, 159, 133, 61, 58, 84, 72, 110, 66, 163, 32, 46, 68, 217, 184, 124, 165, 119, 197, 24, 74, 237, 128, 13, 112, 247, 109, 162, 60, 83, 42, 158, 86, 171, 252, 97, 135, 178, 188, 205, 63, 91, 204, 90, 96, 177, 157, 170, 161, 82, 12, 246, 23, 236, 123, 118, 45, 216, 80, 175, 214, 234, 231, 232, 174, 233, 117, 215, 245, 235, 169, 81, 89, 176];
   
    let mut decoded = decode(&encoded).unwrap();
    let mut inv_P: [usize; 256] = [0; 256];
    let mut tmp: [u8; 256] = [11; 256];
    //let mut seed2=b"kotobahairanai";

    let mut seed:[u8;32]=[0;32];
    //seed=p2(seed2);
    let mut rng2: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed);
    let aa:String="kotobahairanai".to_string();
    //let v:Vec<u8>=aa.to_vec();
    let mut count=0;
    let cycle = rng2.gen_range(1..256);
    let mut it:[u8;256]=[0;256];


    println!("len = {}, {}", decoded.len(), cycle);

    for i in 0..256 {
        inv_P[a[i as usize] as usize] = i as usize;
    }
    let l = decoded.len();
    let _size: usize = 32;
    //let mut result:[u8;256]=[0;256];

    for i in 0..32{
        seed[i]=i as u8;
    }
    let mut be:[u8;32]=[0;32]; ////seed.clone();
    seed=shuffule(seed, 32, &be);

    let mut inv:[u8;32]=[0;32];
    for i in 0..32{
        inv[seed[i] as usize]=i as u8;
    }
/* 
    for i in 0..16{
        seed=lot(seed);
    }
    */
    let mut ee:[u8;32]=[0;32];
    for i in 0..32{
        ee[i]=i as u8;
    }
    ee=permute(seed,ee,16);

    for i in 0..3{
        be=p2(&be);

    }
    //result=pappy(result);
    //println!("{:?}",result);
    for i in 0..256{
    it[i]=a[i];
    }
    for j in (0..16) {
        // read hash digest
            //be=p2(&be);
            
            ee=rebirth(inv, ee, 1);
            //ee=rot(ee);
            //println!("{:?}",seed);
            
            //it=pappy(&it);
            //println!("aa={:?}",decoded);
            //println!("ie={:?}",be);
            for i in (0..l) {
            
            decoded[i] = mat[[ it[(16 * j + i)%256 as usize]  as usize, (decoded[i as usize]) as usize]];

            decoded[i] = (inv_P[decoded[i] as usize] as usize) as u8;

            //println!("dec {}", (decoded[i] % 16));
            decoded[i] = INV_S_BOX[(((decoded[i] % 16) + (decoded[i] >> 4) * 16) as usize)];
            //decoded[i]^=fg[decoded[i] as usize];
            decoded[i]^=be[ee[i%32] as usize]; //gf[mlt(oinv(be[i%32] as u16),fg[decoded[i] as usize] as u16) as usize];
            
        }

    }

    
  
    //println!("{:?}",decoded);
    //exit(1);

    for i in 0..l {
        buf[i] = decoded[i];
        //buf[i]^=be[(i+1)%32];
    }
    let v:Vec<u8>=vec![1,2,3];

    println!("plain text:");
    println!("decrypted = {:?}", &buf[0..l]);

    /*
    match String::from_utf8(buf.to_vec()) {
        Err(_why) => {
            println!("復号できませんでした");
            "baka".to_string()
        }
        Ok(str) => encode(&str[0..l])
    }
    */

    /* 
    // \*let ret =/
     match String::from_utf8(buf.to_vec()) {
        Err(_why) => {
        println!("復号できませんでした");
        b"baka".to_vec()
        }
        Ok(str) => str.as_bytes().to_vec()
        }
        //ret
        */
    //buf.to_vec()
    encode(&buf[0..l])
}

fn hmac(message: &[u8], key: [u8; 32]) -> Vec<u8> {
    let ipad: [u8; 32] = [0x36; 32];
    let opad: [u8; 32] = [0x5c; 32];
    //let m:&[u8]=message.as_bytes();
    let mut hasher = Keccak256::default();
    let mut k1: Vec<u8> = key.to_vec();
    let mut k2: Vec<u8> = key.to_vec();
    for i in 0..32 {
        k1[i] ^= opad[i];
        k2[i] ^= ipad[i];
    }
    let mut K1: Vec<u8> = vec![];
    let mut K2: Vec<u8> = vec![];

    K1.write_all(&k1).unwrap();
    K2.write_all(&k2).unwrap();
    K2.write_all(message).unwrap();
    //println!("{:?}",k2);
    //println!("{:?}",K2);
    //exit(1);

    hasher.update(K2);
    let result2 = hasher.finalize();
    K1.write(&result2.to_vec()).unwrap();
    let mut hasher = Keccak256::default();
    hasher.update(K1);
    let result: Vec<u8> = hasher.finalize().to_vec();
    //let be:String=String::from_utf8(result).unwrap();
    for x in &result {
        print!("{:0x}", x);
    }
    println!();

    result
}

fn v2u(bytes:&[u8])->Vec<u8>{
    let bytes_owned: Vec<u8> = bytes.to_owned(); // &[u8] -> Vec<u8>

    bytes_owned
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().fold("".to_owned(), |s, b| s + &format!("{:x}", b))
}

fn v2s(a: Vec<u8>) -> String {
    let b: String = String::from_utf8(a).unwrap();
    //assert_eq!("0".to_string(), b);
    b
}

fn s2v(a: String) -> Vec<u8> {
    let b: Vec<u8> = a.into_bytes();
    b
}

fn ae(cc:String,seed2:[u8;32])-> Vec<u8>{

    let mut gg = cc.clone();
    let mut dd: Vec<u8> = hmac(cc.as_bytes(), seed2);
    println!("d1={:?}", dd);
    //let e1=encode(dd);
    let d2: &[u8] = &dd;

    dd = hmac(d2, seed2);
    println!("d2={:?}", dd);
    //println!("encd_hash={:?}",dd);
    let mut f: Vec<u8> = vec![]; //dd; //(cc.as_bytes()).to_vec();
    f.write(&dd).unwrap();
    f.write(gg.as_bytes()).unwrap();
    //println!("f={:?}\ndd={:?}\n gg={:?}", f, dd, gg.as_bytes());
    //exit(1);

    for _i in 0..32 {
        dd[_i] = f[_i];
    }
    let tmp: &[u8] = &f[32..f.len()];
    //for i in 0..f.len()-32{
    let x: &[u8] = tmp;
    //}
    println!("msg={:?}", x);
    let mut w = hmac(x, seed2);
    println!("w1={:?}", w);
    //let e2=encode(w);
    let t: &[u8] = &w;
    w = hmac(t, seed2);
    println!("w2={:?}", w);
    //exit(1);
    let z: String = String::from_utf8(x.to_vec()).unwrap();
    
    x.to_vec()
}

fn v2vs(src:Vec<u8>)-> Vec<String>{
    // 整数型
    // 要素を整数型から文字列型変換
    let dst: Vec<String> = src.iter().map(|x| x.to_string()).collect();
    // 文字列型vectorを区切り文字で結合
    println!("{}", dst.join(" "));

    dst
}

//use ndarray::Array2;
fn main() {
    //let mut key:[u8;256]=[0;256];
    let mut data = String::new(); //from("日本語入力");
    let mut mat: Array2<u8> = Array2::zeros((256, 256));
    let mut sk: [u8; 256]=[0;256];
    let mut mat2: Array2<u8> = Array2::zeros((256, 256));

    let mut seed2: [u8; 32] = [17; 32];
    let mut rng2: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed2);
    let seedA: u64 = 1234567890;
    let seedB: u64 = 1234567890;
    let mut _rngA = rand_chacha::ChaCha20Rng::seed_from_u64(seedA);
    let mut _rngB = rand_chacha::ChaCha20Rng::seed_from_u64(seedB);
    let nonce: &[u8] = b"kotobahairanai";
    //let bytes: &[u8] = nonce.as_bytes();
    let mut seed =&p2(nonce); //rng2.gen_range(1..256);
    //seed=p2(&seed);
    let sk3=pappy(nonce);
    let sk2=pappy(&seed2);
    let n2=pappy(&sk3);
    let n3=pappy(&n2);
    let mut nk:[u8;32]=[0;32];
    let mut nk2:[u8;32]=[0;32];
    let mut tmp:[u8;32]=[0;32];
    let mut me:Array2<u8>=Array2::zeros((16,16));
    let mut inn:[u8;256]=[0;256];
    const gf:[u8;256] = [0, 1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205, 135, 19, 38, 76, 152, 45, 90, 180, 117, 234, 201, 143, 3, 6, 12, 24, 48, 96, 192, 157, 39, 78, 156, 37, 74, 148, 53, 106, 212, 181, 119, 238, 193, 159, 35, 70, 140, 5, 10, 20, 40, 80, 160, 93, 186, 105, 210, 185, 111, 222, 161, 95, 190, 97, 194, 153, 47, 94, 188, 101, 202, 137, 15, 30, 60, 120, 240, 253, 231, 211, 187, 107, 214, 177, 127, 254, 225, 223, 163, 91, 182, 113, 226, 217, 175, 67, 134, 17, 34, 68, 136, 13, 26, 52, 104, 208, 189, 103, 206, 129, 31, 62, 124, 248, 237, 199, 147, 59, 118, 236, 197, 151, 51, 102, 204, 133, 23, 46, 92, 184, 109, 218, 169, 79, 158, 33, 66, 132, 21, 42, 84, 168, 77, 154, 41, 82, 164, 85, 170, 73, 146, 57, 114, 228, 213, 183, 115, 230, 209, 191, 99, 198, 145, 63, 126, 252, 229, 215, 179, 123, 246, 241, 255, 227, 219, 171, 75, 150, 49, 98, 196, 149, 55, 110, 220, 165, 87, 174, 65, 130, 25, 50, 100, 200, 141, 7, 14, 28, 56, 112, 224, 221, 167, 83, 166, 81, 162, 89, 178, 121, 242, 249, 239, 195, 155, 43, 86, 172, 69, 138, 9, 18, 36, 72, 144, 61, 122, 244, 245, 247, 243, 251, 235, 203, 139, 11, 22, 44, 88, 176, 125, 250, 233, 207, 131, 27, 54, 108, 216, 173, 71, 142];
    let mut van=arr2(&[
        [2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17],
        [4,5,16,17,20,21,64,65,68,69,80,81,84,85,29,28],
        [8,15,64,85,120,107,58,115,146,221,231,186,127,36,205,193],
        [16,17,29,28,13,12,205,204,221,220,208,209,192,193,76,77],
        [32,51,116,108,46,36,38,226,1,215,169,116,244,59,180,233],
        [64,85,205,193,228,252,45,161,10,146,191,62,241,100,143,223],
        [128,255,19,226,98,206,117,192,68,79,87,43,199,38,24,174],
        [29,28,76,77,81,80,143,142,146,147,195,194,222,223,157,156],
        [58,36,45,100,251,173,12,138,221,68,125,179,64,145,37,169],
        [116,108,180,233,32,100,96,174,1,214,38,180,167,44,106,235],
        [232,180,234,106,192,33,39,183,10,153,181,151,164,185,238,253],
        [205,193,143,223,186,231,37,102,68,10,47,61,182,169,70,150],
        [135,94,6,132,187,143,53,113,146,78,217,60,74,89,20,3],
        [19,226,24,174,189,138,181,222,221,152,197,49,203,96,93,51],
        [38,59,96,44,169,145,193,96,1,1,85,96,150,26,185,36],
        [76,77,157,156,209,208,70,71,10,11,219,218,151,150,95,94]
        ]);

inn=m2v(van);
for i in 0..256{
    print!("{},",inn[i]);
}

me=v2m(inn);
println!("{:?}",me);

exit(1);

for i in 0..32{
    nk[i]=i as u8;
}
    nk=shuffule(nk,32, seed);
println!("ntt={:?}",nk);


for i in 0..32{
    nk2[nk[i] as usize]=i as u8;
}
for i in 0..32{
    tmp[i]=i as u8;
}
//tmp=nk.clone();
for i in 0..3{
tmp=permute(nk,tmp,1);
println!("nec={:?}",tmp);
}
//exit(1);
//tmp=nk;
for i in 0..3{
tmp=rebirth(nk2,tmp,1);
println!("inv={:?}",tmp);
}
//exit(1);
    

    for _j in 0..256 {
        for _i in 0..256 {
            sk[_i] = _i as u8;
        }
        //let _seed = rng2.gen::<u64>();
        //rng2.gen::<u64>(); // 32バイトシードで再現あり
        sk = random_shuffule(sk, 256, &seed2);
        for _k in 0..256 {
            mat[[_j, _k]] = sk[_k];
            //print!("{}, ", mat[[j, k]]);
        }
        //println!("");
    }
    /*
    for i in 0..32{
        seed2[i]=i as u8;
    }
    for i in 0..32{
        seed2=rot(seed2);
        println!("{:?}",seed2);
    }
    exit(1);
*/

    for _i in 0..256 {
        for _j in 0..256 {
            mat2[[_i, mat[[_i, _j]] as usize]] = _j as u8;
        }
    }

    for _i in 0..256 {
        sk[_i] = _i as u8;
    }
    //let seed = rng2.gen::<u64>();
    sk = random_shuffule(sk, 256, seed);


    println!("何か入力を");
    std::io::stdin().read_line(&mut data).ok();
    data = data.trim_end().to_owned();
    println!("{}", data);

    data=encode(data); //attention !!
    
    let mut cc:String=String::new();
    let mut l:String=String::new();
    let iv:&[u8]="aaaaaa".as_bytes();

    // encoded below
    for i in 0..1{
        cc = enc(&data, &sk, &mat, nonce);
    }

    println!(" ");

    // encoded above
    for i in 0..1{
    l=dec(&cc ,&sk, &mat2, nonce);
    }



 
/*
    let cc:String = ctr(&data,&sk,&mat);
    println!("{:?}",cc);
    let cc:String = ctr(&cc,&sk,&mat);
*/

let code=decode(&l).unwrap();

    println!("back to origin: {:?}",v2s(code)); 

 exit(1);
}
