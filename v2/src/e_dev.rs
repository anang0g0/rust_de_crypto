#![allow(non_snake_case)]

use base64::{decode, encode};
use rand::prelude::*;
use rand::rngs::adapter::ReseedingRng;
use rand::rngs::OsRng;
use rand::{Rng, SeedableRng};
use rand_chacha::rand_core::RngCore;
use rand_chacha::ChaCha20Core;
use std::{process::exit, str};

/*
    Fisher-Yates shuffle による方法
    配列の要素をランダムシャッフルする
*/
fn random_shuffule(mut array: [u8; 256], size: u16, seed: u64) -> [u8; 256] {
    let _i: usize;
    let mut a: usize;
    let mut b: usize;
    //let seed2: [u8; 32] = [17;32];
    //let mut rng2: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed2);
    //let seed: u64 = 1;
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);

    for _i in (1..size).rev() {
        a = (_i) as usize;
        let b = rng.gen::<u8>() % _i as u8; // 32バイトシードで再現あり
        (array[a], array[b as usize]) = (array[b as usize], array[a])
    }

    array
}

fn enc(data: &String, a: &[u8; 256], mat: &Array2<u8>) -> String {
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
    let byte = data.as_bytes();
    let mut seed2: [u8; 32] = [17; 32];
    // お好みの乱数で
    //let seed: u64 = 1;
    //let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let mut rng2: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed2);
    let mut Q:[u8;256]=[0;256];
    let mut inv_Q:[u8;256]=[0;256];
    
    println!("len = {}", byte.len());
    println!("origin: {}", str::from_utf8(data.as_bytes()).unwrap());
    let mut me: [u8; 256] = [0; 256];
    let cycle = rng2.gen_range(1..255);

    let j = byte.len();
    if j>256 {
        println!("message size is over");
        exit(1);
    }
    for i in 0..j {
        buf[i] = byte[i];
        buf[i]^=(rng.gen_range(1..256)) as u8;
    }
    for k in 0..16 {
        for i in 0..j {
            buf[i] = S_BOX[((buf[i] % 16) + (buf[i] >> 4) * 16) as usize];
            me[i] = a[buf[i] as usize] as u8;
            buf[i] = mat[[a[(16 * k + i) % cycle] as usize, me[i] as usize]] as u8;
        }
    }

    println!("encryptod = {:?}", &buf[0..j]);

    let encoded = encode(&buf[0..j]);

    println!("cipher text:");
    println!("{}", encoded);

    encoded
}

fn router(mut o:[u8;256],mut r:[u8;256])->[u8;256]{
    let mut inv_o:[u8;256]=[0;256];
    let mut s:[u8;256]=[0;256];

    
    for i in 0..256{
        inv_o[o[i] as usize]=i as u8;
    }
    for i in 0..256{
        s[i]=inv_o[r[o[i] as usize] as usize];
    }
    for i in 0..256{
        o[i]=s[i];
    }
    
    o
}

use ndarray::Array2;
fn main() {
    //let mut key:[u8;256]=[0;256];
    let mut data = String::new(); //from("日本語入力");
    let mut mat: Array2<u8> = Array2::zeros((256, 256));
    let mut sk: [u8; 256] = [0; 256];
    //let mut _it: Array2<u8> = Array2::zeros((256, 256));
    let mut mat2: Array2<u8> = Array2::zeros((256, 256));
    let mut _i: usize;
    let mut _j: usize;
    let mut seed: u64 = 1234567890;
    let seed2: [u8; 32] = [17; 32];
    let mut rng2: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed2);

    for j in 0..256 {
        for _i in 0..256 {
            sk[_i] = _i as u8;
        }
        let seed = rng2.gen::<u64>();
        //rng2.gen::<u64>(); // 32バイトシードで再現あり
        sk = random_shuffule(sk, 256, seed);
        for k in 0..256 {
            mat[[j, k]] = sk[k];
            print!("{}, ", mat[[j, k]]);
        }
        println!("");
    }

    for i in 0..256 {
        for j in 0..256 {
            mat2[[i, mat[[i, j]] as usize]] = j as u8;
        }
    }
    //exit(1);

    for _i in 0..256 {
        sk[_i] = _i as u8;
    }
    sk = random_shuffule(sk, 256, seed);
    let sk2 = sk.clone();
    println!("{:?}",&sk);

    let mut tmp:[u8;256]=[0;256];
    for i in 0..256{
       tmp[i]=sk[sk[i] as usize];
    }
    println!("\n\ntwice {:?}",tmp);
    //exit(1);

    println!("何か入力を");
    std::io::stdin().read_line(&mut data).ok();
    data = data.trim_end().to_owned();
    println!("{}", data);

    let cc = enc(&data, &sk, &mat);
    println!(" ");
    //let l = dec(cc, &sk2, &mat2);

    println!("back to origin: {}", cc);
}

