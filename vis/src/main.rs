#![allow(non_snake_case)]

use base64::{decode, encode};
use rand::{Rng, SeedableRng};
//use rand_chacha::ChaCha8Rng;
use std::{str, process::exit};

/*
    Fisher-Yates shuffle による方法
    配列の要素をランダムシャッフルする
*/
fn random_shuffule(mut array: [u8; 256], size: u16) -> [u8; 256] {
    let _i: usize;
    let mut a: usize;
    let mut b: usize;
    let seed2: [u8; 32] = [1;32]; 
    let mut rng2: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed2);
    let seed: u64 = 1;
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let mut _c: usize;
    let mut it: usize; // genはRng traitに定義されている
    let mut be;

    for _i in (1..size).rev() {
        a = (_i) as usize;
        _c = (rng.gen_range(1..256)) as usize; //暗号理論的に安全だが初期値が小さい、再現あり
        //b=c as usize;
        be = rng2.gen::<u8>() as usize; // 32バイトシードで再現あり
        it =(rand::thread_rng().gen_range(1..256) % _i) as usize; //毎回変わる
        b = it; //be&_c;
        // ソートするキーの型
        (array[a], array[b]) = (array[b], array[a])
    }

    array
}

fn enc(data: &String, a: [u8; 256],mat:&Array2<u8>) -> String {
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
        0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16 // f
    ];
    let gf: [i32; 256] = [
        0, 1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205, 135, 19, 38, 76, 152, 45, 90, 180,
        117, 234, 201, 143, 3, 6, 12, 24, 48, 96, 192, 157, 39, 78, 156, 37, 74, 148, 53, 106, 212,
        181, 119, 238, 193, 159, 35, 70, 140, 5, 10, 20, 40, 80, 160, 93, 186, 105, 210, 185, 111,
        222, 161, 95, 190, 97, 194, 153, 47, 94, 188, 101, 202, 137, 15, 30, 60, 120, 240, 253,
        231, 211, 187, 107, 214, 177, 127, 254, 225, 223, 163, 91, 182, 113, 226, 217, 175, 67,
        134, 17, 34, 68, 136, 13, 26, 52, 104, 208, 189, 103, 206, 129, 31, 62, 124, 248, 237, 199,
        147, 59, 118, 236, 197, 151, 51, 102, 204, 133, 23, 46, 92, 184, 109, 218, 169, 79, 158,
        33, 66, 132, 21, 42, 84, 168, 77, 154, 41, 82, 164, 85, 170, 73, 146, 57, 114, 228, 213,
        183, 115, 230, 209, 191, 99, 198, 145, 63, 126, 252, 229, 215, 179, 123, 246, 241, 255,
        227, 219, 171, 75, 150, 49, 98, 196, 149, 55, 110, 220, 165, 87, 174, 65, 130, 25, 50, 100,
        200, 141, 7, 14, 28, 56, 112, 224, 221, 167, 83, 166, 81, 162, 89, 178, 121, 242, 249, 239,
        195, 155, 43, 86, 172, 69, 138, 9, 18, 36, 72, 144, 61, 122, 244, 245, 247, 243, 251, 235,
        203, 139, 11, 22, 44, 88, 176, 125, 250, 233, 207, 131, 27, 54, 108, 216, 173, 71, 142,
    ];

    let mut buf: [u8; 256] = [0; 256];
    let byte = data.as_bytes();
    let seed2: [u8; 32] = [1;32]; 
    let mut rng2: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed2);
    let seed: u64 = 1;
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);

    println!("len = {}",byte.len());
    println!("origin: {}", str::from_utf8(data.as_bytes()).unwrap());
    let mut me:[u8;256]=[0;256];

    let j = byte.len();
    for i in 0..j{
        buf[i]=byte[i];
        //buf[i]^=(rng.gen_range(1..256)) as u8;

    }
    for k in 0..16{
    for i in 0..j {
        
        //buf[i]^=i as u8;
        buf[i]=S_BOX[((buf[i] % 16) + (buf[i] >> 4) * 16) as usize];
        //buf[i]^=rng2.gen::<u8>() as u8; // 32バイトシードで再現あり
        //buf[i]^=i as u8;
        me[i] = a[ buf[i] as usize] as u8;
        
        buf[i]=mat[[a[i] as usize as usize, me[i] as usize]] as u8;
        //buf[i] ^= i as u8;//a[i];    
    }
    }

    println!("encryptod = {:?}", &buf[0..j]);

    let encoded = encode(&buf[0..j]);

    println!("cipher text:");
    println!("{}", encoded);

    encoded
}

fn dec(encoded: String, a: [u8; 256],mat:&Array2<u8>) -> String {
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
        0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d  // f
    ];
    let fg: [i32; 256] = [
        0, 1, 2, 26, 3, 51, 27, 199, 4, 224, 52, 239, 28, 105, 200, 76, 5, 101, 225, 15, 53, 142,
        240, 130, 29, 194, 106, 249, 201, 9, 77, 114, 6, 139, 102, 48, 226, 37, 16, 34, 54, 148,
        143, 219, 241, 19, 131, 70, 30, 182, 195, 126, 107, 40, 250, 186, 202, 155, 10, 121, 78,
        229, 115, 167, 7, 192, 140, 99, 103, 222, 49, 254, 227, 153, 38, 180, 17, 146, 35, 137, 55,
        209, 149, 207, 144, 151, 220, 190, 242, 211, 20, 93, 132, 57, 71, 65, 31, 67, 183, 164,
        196, 73, 127, 111, 108, 59, 41, 85, 251, 134, 187, 62, 203, 95, 156, 160, 11, 22, 122, 44,
        79, 213, 230, 173, 116, 244, 168, 88, 8, 113, 193, 248, 141, 129, 100, 14, 104, 75, 223,
        238, 50, 198, 255, 25, 228, 166, 154, 120, 39, 185, 181, 125, 18, 69, 147, 218, 36, 33,
        138, 47, 56, 64, 210, 92, 150, 189, 208, 206, 145, 136, 152, 179, 221, 253, 191, 98, 243,
        87, 212, 172, 21, 43, 94, 159, 133, 61, 58, 84, 72, 110, 66, 163, 32, 46, 68, 217, 184,
        124, 165, 119, 197, 24, 74, 237, 128, 13, 112, 247, 109, 162, 60, 83, 42, 158, 86, 171,
        252, 97, 135, 178, 188, 205, 63, 91, 204, 90, 96, 177, 157, 170, 161, 82, 12, 246, 23, 236,
        123, 118, 45, 216, 80, 175, 214, 234, 231, 232, 174, 233, 117, 215, 245, 235, 169, 81, 89,
        176,
    ];

    let mut decoded = decode(&encoded).unwrap();
    let mut inv_P: [usize; 256] = [0; 256];
    let seed2: [u8; 32] = [1;32]; 
    let mut rng2: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed2);
    let mut tmp:[u8;256]=[0;256];
    let seed: u64 = 1;
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    
    println!("len = {}",decoded.len());

    for i in 0..256 {
        inv_P[a[i as usize] as usize] = i as usize;
    }

    for j in (0..16).rev() {
        for i in 0..decoded.len() {
            //for k in 0..256
            //
            //decoded[i] = decoded[i] ^ i  as u8; //a[i] as u8;
            decoded[i]=mat[[a[i] as usize,decoded[i] as usize]];

            tmp[i] = (inv_P[decoded[i] as usize] as usize) as u8;

            //println!("dec {}", (decoded[i] % 16));
            decoded[i] = INV_S_BOX[(((tmp[i] % 16) + (tmp[i] >> 4) * 16) as usize)];
           //decoded[i]^=i as u8;
        }
    }
    for i in 0..decoded.len() {
        buf[i] = decoded[i];
        //buf[i]^=(rng.gen_range(1..256)) as u8;
    }

    println!("plain text:");
    println!("decrypted = {:?}", &buf[0..decoded.len()]);
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
    let mut a: [u8; 256] = [0; 256];
    let mut _it: Array2<u8> = Array2::zeros((256, 256));
    let mut mat2:Array2<u8>=Array2::zeros((256,256));
    let mut _i: usize;
    let mut _j: usize;
    let seed2: [u8; 32] = [1;32]; 
    let mut rng2: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed2);

    for j in 0..256{
        for _i in 0..256 {
            a[_i] = _i as u8;
        }
        a = random_shuffule(a, 256);
        for k in 0..256{
    mat[[j,k]]=a[k];
    }
}
for i in 0..256{
    for j in 0..256{
        print!("{},",mat[[i,j]]);
    }
    println!("");
}
for i in 0..256{
    for j in 0..256{
        mat2[[i,mat[[i,j]] as usize]]=j as u8;
    }
}
//exit(1);

    /*
        for _j in 1..32{
        for _i in 1..256{
        {
        a[_i]= _i as u8;
        }
        //exit(1);
        a=random_shuffule(a,256);

        for _i in 1..256{
            print!("{},",a[_i]);
        }
        println!("\n");
        it[[_j,_i]] =a[_i];
        }
    */

        for _i in 0..256 {
            a[_i] = _i as u8;
        }
        a = random_shuffule(a, 256);
        let u=a.clone();
    
    println!("何か入力を");
    std::io::stdin().read_line(&mut data).ok();
    data = data.trim_end().to_owned();
    println!("{}", data);

    

    let cc = enc(&data, a, &mat);
    println!(" ");
    let l = dec(cc, u, &mat2);


    println!("back to origin: {}", l);
}
