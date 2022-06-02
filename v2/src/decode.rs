#![allow(non_snake_case)]

use base64::{decode, encode};
use rand::prelude::*;
use rand::rngs::adapter::ReseedingRng;
use rand::rngs::OsRng;
use rand::{Rng, SeedableRng};
use rand_chacha::rand_core::RngCore;
use rand_chacha::ChaCha20Core;
use std::{process::exit, str};
use sha3::{Digest, Keccak256};


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
        let b: usize = (rng.gen::<u8>() % _i as u8) as usize; // 32バイトシードで再現あり
        (array[a], array[b]) = (array[b], array[a])
    }

    array
}

fn dec(encoded: String, a: &[u8; 256], mat: &Array2<u8>) -> String {
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

    let mut decoded = decode(&encoded).unwrap();
    let mut inv_P: [usize; 256] = [0; 256];
    let mut seed2: [u8; 32] = [17; 32];
    let mut rng2: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed2);
    let mut tmp: [u8; 256] = [0; 256];
    let seed: u64 = 1;
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let cycle = rng2.gen_range(1..256);
    println!("len = {}", decoded.len());

    for i in 0..256 {
        inv_P[a[i as usize] as usize] = i as usize;
    }
    let l = decoded.len();
    for j in (0..16).rev() {
        for i in 0..l {
            decoded[i] = mat[[a[(16 * j + i) % cycle] as usize, decoded[i] as usize]];
            tmp[i] = (inv_P[decoded[i] as usize] as usize) as u8;

            //println!("dec {}", (decoded[i] % 16));
            decoded[i] = INV_S_BOX[(((tmp[i] % 16) + (tmp[i] >> 4) * 16) as usize)];
        }
    }
    for i in 0..l {
        buf[i] = decoded[i];
        //buf[i]^=(rng.gen_range(1..256)) as u8;
    }
    //for i in l..256 {
    //    buf[i]^=(rng.gen_range(1..256)) as u8;
    //}
    let mut hasher = Keccak256::new();
    
    // write input message
    hasher.update(buf);

    // read hash digest
    let result = hasher.finalize();
    println!("\n\n{:0x}",result);
    
    println!(" ");

    println!("plain text:");
    println!("decrypted = {:?}", &buf[0..l]);
    match String::from_utf8(buf.to_vec()) {
        Err(_why) => {
            println!("復号できませんでした");
            "へげええええ！".to_string()
        }
        Ok(str) => str,
    }
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
    let mut seedA: u64 = 1234567890;
    let mut seed: u64 = 1234567890;
    let seed2: [u8; 32] = [17; 32];
    let mut rng2: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed2);
    let seedB: u64 = 1234567890;
    let mut rngA = rand_chacha::ChaCha20Rng::seed_from_u64(seedA);
    let mut rngB = rand_chacha::ChaCha20Rng::seed_from_u64(seedB);

    // create a SHA3-256 object


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

    println!("何か入力を");
    std::io::stdin().read_line(&mut data).ok();
    data = data.trim_end().to_owned();
    println!("{}", data);

    //let cc = enc(&data, &sk, &mat);
    println!(" ");
    let l = dec(data, &sk2, &mat2);

    println!("back to origin: {}", l);
}
