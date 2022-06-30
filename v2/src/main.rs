#![allow(non_snake_case)]
use base64::{decode, encode};
use rand::prelude::*;
//use rand::rngs::adapter::ReseedingRng;
use rand::{Rng, SeedableRng};
//use rand_chacha::ChaCha20Core;
use ndarray::arr2;
use ndarray::prelude::*;
use ndarray::Array2;
use sha3::Sha3_256;
use sha3::{Digest, Keccak256};
use std::io::Write;
//use std::slice::ArrayChunks;
use std::{process::exit, str};

const gf: [u8; 256] = [
    0, 1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205, 135, 19, 38, 76, 152, 45, 90, 180, 117, 234, 201, 143, 3, 6,
    12, 24, 48, 96, 192, 157, 39, 78, 156, 37, 74, 148, 53, 106, 212, 181, 119, 238, 193, 159, 35, 70, 140, 5, 10, 20,
    40, 80, 160, 93, 186, 105, 210, 185, 111, 222, 161, 95, 190, 97, 194, 153, 47, 94, 188, 101, 202, 137, 15, 30, 60,
    120, 240, 253, 231, 211, 187, 107, 214, 177, 127, 254, 225, 223, 163, 91, 182, 113, 226, 217, 175, 67, 134, 17, 34,
    68, 136, 13, 26, 52, 104, 208, 189, 103, 206, 129, 31, 62, 124, 248, 237, 199, 147, 59, 118, 236, 197, 151, 51,
    102, 204, 133, 23, 46, 92, 184, 109, 218, 169, 79, 158, 33, 66, 132, 21, 42, 84, 168, 77, 154, 41, 82, 164, 85,
    170, 73, 146, 57, 114, 228, 213, 183, 115, 230, 209, 191, 99, 198, 145, 63, 126, 252, 229, 215, 179, 123, 246, 241,
    255, 227, 219, 171, 75, 150, 49, 98, 196, 149, 55, 110, 220, 165, 87, 174, 65, 130, 25, 50, 100, 200, 141, 7, 14,
    28, 56, 112, 224, 221, 167, 83, 166, 81, 162, 89, 178, 121, 242, 249, 239, 195, 155, 43, 86, 172, 69, 138, 9, 18,
    36, 72, 144, 61, 122, 244, 245, 247, 243, 251, 235, 203, 139, 11, 22, 44, 88, 176, 125, 250, 233, 207, 131, 27, 54,
    108, 216, 173, 71, 142,
];
const fg: [u8; 256] = [
    0, 1, 2, 26, 3, 51, 27, 199, 4, 224, 52, 239, 28, 105, 200, 76, 5, 101, 225, 15, 53, 142, 240, 130, 29, 194, 106,
    249, 201, 9, 77, 114, 6, 139, 102, 48, 226, 37, 16, 34, 54, 148, 143, 219, 241, 19, 131, 70, 30, 182, 195, 126,
    107, 40, 250, 186, 202, 155, 10, 121, 78, 229, 115, 167, 7, 192, 140, 99, 103, 222, 49, 254, 227, 153, 38, 180, 17,
    146, 35, 137, 55, 209, 149, 207, 144, 151, 220, 190, 242, 211, 20, 93, 132, 57, 71, 65, 31, 67, 183, 164, 196, 73,
    127, 111, 108, 59, 41, 85, 251, 134, 187, 62, 203, 95, 156, 160, 11, 22, 122, 44, 79, 213, 230, 173, 116, 244, 168,
    88, 8, 113, 193, 248, 141, 129, 100, 14, 104, 75, 223, 238, 50, 198, 255, 25, 228, 166, 154, 120, 39, 185, 181,
    125, 18, 69, 147, 218, 36, 33, 138, 47, 56, 64, 210, 92, 150, 189, 208, 206, 145, 136, 152, 179, 221, 253, 191, 98,
    243, 87, 212, 172, 21, 43, 94, 159, 133, 61, 58, 84, 72, 110, 66, 163, 32, 46, 68, 217, 184, 124, 165, 119, 197,
    24, 74, 237, 128, 13, 112, 247, 109, 162, 60, 83, 42, 158, 86, 171, 252, 97, 135, 178, 188, 205, 63, 91, 204, 90,
    96, 177, 157, 170, 161, 82, 12, 246, 23, 236, 123, 118, 45, 216, 80, 175, 214, 234, 231, 232, 174, 233, 117, 215,
    245, 235, 169, 81, 89, 176,
];

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

const Sbox: [[u8; 16]; 8] = [
    [3, 8, 15, 1, 10, 6, 5, 11, 14, 13, 4, 2, 7, 0, 9, 12],
    [15, 12, 2, 7, 9, 0, 5, 10, 1, 11, 14, 8, 6, 13, 3, 4],
    [8, 6, 7, 9, 3, 12, 10, 15, 13, 1, 14, 4, 0, 11, 5, 2],
    [0, 15, 11, 8, 12, 9, 6, 3, 13, 1, 2, 4, 10, 7, 5, 14],
    [1, 15, 8, 3, 12, 0, 11, 6, 2, 5, 4, 10, 9, 14, 7, 13],
    [15, 5, 2, 11, 4, 10, 9, 12, 0, 3, 14, 8, 13, 6, 7, 1],
    [7, 2, 12, 5, 8, 4, 6, 11, 14, 9, 1, 15, 13, 3, 10, 0],
    [1, 13, 15, 0, 14, 8, 2, 11, 7, 4, 12, 10, 9, 3, 5, 6],
];
//let InvS:usize;16;8=
const InvS: [[u8; 16]; 8] = [
    [13, 3, 11, 0, 10, 6, 5, 12, 1, 14, 4, 7, 15, 9, 8, 2],
    [5, 8, 2, 14, 15, 6, 12, 3, 11, 4, 7, 9, 1, 13, 10, 0],
    [12, 9, 15, 4, 11, 14, 1, 2, 0, 3, 6, 13, 5, 8, 10, 7],
    [0, 9, 10, 7, 11, 14, 6, 13, 3, 5, 12, 2, 4, 8, 15, 1],
    [5, 0, 8, 3, 10, 9, 7, 14, 2, 12, 11, 6, 4, 15, 13, 1],
    [8, 15, 2, 9, 4, 1, 13, 14, 11, 6, 5, 3, 7, 12, 10, 0],
    [15, 10, 1, 13, 5, 3, 6, 0, 4, 9, 14, 7, 2, 12, 8, 11],
    [3, 0, 6, 13, 9, 14, 15, 8, 5, 12, 11, 7, 10, 1, 4, 2],
];

const rs: [[u8; 64]; 8] = [
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
        32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
        60, 61, 62, 63, 64, 65, 66,
    ],
    [
        5, 16, 17, 20, 21, 64, 65, 68, 69, 80, 81, 84, 85, 29, 28, 25, 24, 13, 12, 9, 8, 93, 92, 89, 88, 77, 76, 73,
        72, 116, 117, 112, 113, 100, 101, 96, 97, 52, 53, 48, 49, 36, 37, 32, 33, 105, 104, 109, 108, 121, 120, 125,
        124, 41, 40, 45, 44, 57, 56, 61, 60, 205, 204, 201,
    ],
    [
        15, 64, 85, 120, 107, 58, 115, 146, 221, 231, 186, 127, 36, 205, 193, 191, 181, 228, 252, 166, 184, 107, 47,
        185, 251, 223, 143, 61, 107, 38, 115, 70, 21, 145, 208, 193, 134, 115, 110, 179, 168, 89, 80, 169, 166, 127,
        39, 101, 59, 161, 237, 139, 193, 182, 166, 12, 26, 245, 241, 127, 125, 45, 161, 191,
    ],
    [
        17, 29, 28, 13, 12, 205, 204, 221, 220, 208, 209, 192, 193, 76, 77, 92, 93, 81, 80, 65, 64, 129, 128, 145, 144,
        156, 157, 140, 141, 180, 181, 164, 165, 169, 168, 185, 184, 121, 120, 105, 104, 100, 101, 116, 117, 248, 249,
        232, 233, 229, 228, 245, 244, 53, 52, 37, 36, 40, 41, 56, 57, 143, 142, 159,
    ],
    [
        51, 116, 108, 46, 36, 38, 226, 1, 215, 169, 116, 244, 59, 180, 233, 17, 94, 32, 100, 255, 169, 132, 28, 38,
        172, 235, 106, 51, 160, 3, 150, 108, 235, 26, 150, 15, 145, 116, 36, 28, 94, 150, 223, 132, 223, 77, 132, 167,
        124, 180, 100, 36, 230, 44, 32, 193, 223, 46, 59, 185, 190, 96, 174, 55,
    ],
    [
        85, 205, 193, 228, 252, 45, 161, 10, 146, 191, 62, 241, 100, 143, 223, 47, 107, 186, 231, 115, 58, 252, 33, 59,
        242, 150, 70, 56, 252, 96, 161, 217, 12, 15, 195, 223, 7, 161, 237, 127, 39, 145, 208, 38, 115, 241, 97, 168,
        44, 102, 251, 86, 223, 110, 115, 80, 89, 166, 182, 241, 245, 37, 102, 47,
    ],
    [
        255, 19, 226, 98, 206, 117, 192, 68, 79, 87, 43, 199, 38, 24, 174, 148, 67, 189, 138, 84, 33, 200, 30, 36, 83,
        51, 93, 234, 6, 156, 56, 141, 137, 193, 173, 26, 245, 248, 58, 122, 216, 85, 102, 77, 61, 224, 131, 187, 103,
        247, 45, 242, 180, 184, 105, 161, 15, 48, 97, 39, 42, 181, 222, 81,
    ],
];

const MDS4: [[u8; 4]; 4] = [[1, 1, 1, 1], [2, 3, 4, 5], [4, 5, 16, 17], [8, 15, 64, 85]];
const inv4: [[u8; 4]; 4] = [
    [208, 230, 157, 192],
    [107, 38, 93, 192],
    [104, 93, 39, 192],
    [210, 157, 231, 192],
];
const MDS8: [[u8; 8]; 8] = [
    [1, 1, 1, 1, 1, 1, 1, 1],
    [2, 3, 4, 5, 6, 7, 8, 9],
    [4, 5, 16, 17, 20, 21, 64, 65],
    [8, 15, 64, 85, 120, 107, 58, 115],
    [16, 17, 29, 28, 13, 12, 205, 204],
    [32, 51, 116, 108, 46, 36, 38, 226],
    [64, 85, 205, 193, 228, 252, 45, 161],
    [128, 255, 19, 226, 98, 206, 117, 192],
];

const inv8: [[u8; 8]; 8] = [
    [155, 199, 157, 23, 40, 195, 56, 28],
    [25, 15, 174, 11, 207, 223, 36, 28],
    [77, 99, 255, 181, 252, 124, 92, 23],
    [175, 189, 1, 162, 203, 107, 75, 23],
    [85, 57, 44, 113, 174, 78, 149, 183],
    [243, 177, 127, 198, 194, 249, 34, 183],
    [177, 221, 132, 171, 137, 137, 137, 188],
    [118, 67, 26, 23, 53, 53, 53, 188],
];

const MDS16: [[u8; 16]; 16] = [
    [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
    [4, 5, 16, 17, 20, 21, 64, 65, 68, 69, 80, 81, 84, 85, 29, 28],
    [8, 15, 64, 85, 120, 107, 58, 115, 146, 221, 231, 186, 127, 36, 205, 193],
    [16, 17, 29, 28, 13, 12, 205, 204, 221, 220, 208, 209, 192, 193, 76, 77],
    [32, 51, 116, 108, 46, 36, 38, 226, 1, 215, 169, 116, 244, 59, 180, 233],
    [
        64, 85, 205, 193, 228, 252, 45, 161, 10, 146, 191, 62, 241, 100, 143, 223,
    ],
    [128, 255, 19, 226, 98, 206, 117, 192, 68, 79, 87, 43, 199, 38, 24, 174],
    [29, 28, 76, 77, 81, 80, 143, 142, 146, 147, 195, 194, 222, 223, 157, 156],
    [58, 36, 45, 100, 251, 173, 12, 138, 221, 68, 125, 179, 64, 145, 37, 169],
    [116, 108, 180, 233, 32, 100, 96, 174, 1, 214, 38, 180, 167, 44, 106, 235],
    [
        232, 180, 234, 106, 192, 33, 39, 183, 10, 153, 181, 151, 164, 185, 238, 253,
    ],
    [205, 193, 143, 223, 186, 231, 37, 102, 68, 10, 47, 61, 182, 169, 70, 150],
    [135, 94, 6, 132, 187, 143, 53, 113, 146, 78, 217, 60, 74, 89, 20, 3],
    [19, 226, 24, 174, 189, 138, 181, 222, 221, 152, 197, 49, 203, 96, 93, 51],
    [38, 59, 96, 44, 169, 145, 193, 96, 1, 1, 85, 96, 150, 26, 185, 36],
    [76, 77, 157, 156, 209, 208, 70, 71, 10, 11, 219, 218, 151, 150, 95, 94],
];

const inv16: [[u8; 16]; 16] = [
    [77, 240, 60, 82, 7, 193, 162, 86, 35, 152, 75, 172, 81, 161, 217, 226],
    [
        175, 124, 143, 135, 174, 152, 138, 216, 17, 90, 99, 116, 121, 137, 217, 188,
    ],
    [
        249, 217, 194, 104, 20, 211, 43, 14, 232, 247, 57, 132, 236, 246, 126, 145,
    ],
    [199, 96, 228, 206, 231, 58, 3, 71, 21, 16, 17, 182, 196, 222, 126, 237],
    [99, 142, 6, 79, 53, 201, 227, 150, 76, 42, 59, 178, 244, 30, 198, 33],
    [137, 139, 245, 59, 9, 31, 203, 88, 171, 39, 19, 112, 220, 54, 198, 69],
    [
        136, 31, 222, 168, 235, 141, 72, 237, 253, 216, 255, 159, 147, 28, 105, 160,
    ],
    [246, 180, 58, 58, 99, 138, 96, 150, 185, 19, 215, 56, 187, 52, 105, 223],
    [
        25, 100, 253, 111, 223, 106, 165, 81, 223, 250, 214, 129, 160, 223, 250, 25,
    ],
    [
        137, 116, 204, 180, 152, 82, 141, 53, 129, 219, 254, 214, 136, 247, 250, 64,
    ],
    [61, 23, 115, 62, 141, 81, 183, 90, 50, 120, 12, 7, 181, 32, 245, 221],
    [
        137, 243, 215, 125, 144, 217, 159, 65, 163, 124, 36, 186, 157, 8, 245, 204,
    ],
    [159, 110, 184, 222, 7, 245, 230, 69, 153, 17, 98, 141, 193, 164, 33, 50],
    [
        53, 255, 201, 113, 213, 66, 206, 126, 18, 255, 74, 192, 233, 140, 33, 161,
    ],
    [105, 105, 118, 120, 254, 254, 254, 165, 38, 38, 38, 38, 38, 38, 38, 232],
    [118, 238, 118, 246, 254, 254, 254, 140, 38, 38, 38, 38, 38, 38, 38, 215],
];

const Mix: [[u8; 4]; 4] = [
    [0x2, 0x3, 0x1, 0x1],
    [0x1, 0x2, 0x3, 0x1],
    [0x1, 0x1, 0x2, 0x3],
    [0x3, 0x1, 0x1, 0x2],
];

const iMix: [[u8; 4]; 4] = [
    [0xe, 0xb, 0xd, 0x9],
    [0x9, 0xe, 0xb, 0xd],
    [0xd, 0x9, 0xe, 0xb],
    [0xb, 0xd, 0x9, 0xe],
];

const rcon: [u32; 11] = [
    0x00000000, /* invalid */
    0x00000001, /* x^0 */
    0x00000002, /* x^1 */
    0x00000004, /* x^2 */
    0x00000008, /* x^3 */
    0x00000010, /* x^4 */
    0x00000020, /* x^5 */
    0x00000040, /* x^6 */
    0x00000080, /* x^7 */
    0x0000001B, /* x^4 + x^3 + x^1 + x^0 */
    0x00000036, /* x^5 + x^4 + x^2 + x^1 */
];

const Nb: usize = 64;
const K: usize = 8;
const E: usize = 16;
const N: usize = 256;
const BIT: usize = 8;

fn testbit(bit: i32, i: i32) -> bool {
    ((bit >> i) & 1) != 0 //as bool
}
/*
    Fisher-Yates shuffle による方法
    配列の要素をランダムシャッフルする
*/
fn random_shuffule(mut array: [u8; N], size: u16, seed: &[u8]) -> [u8; N] {
    //let _i: usize;
    let mut _a: usize;
    let mut _b: usize;
    let mut sead: [u8; 32] = [123; 32];
    for i in 0..seed.len() {
        sead[i] = seed[i];
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
    let mut sead: [u8; 32] = [123; 32];
    for i in 0..seed.len() {
        sead[i] = seed[i];
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

fn any_length_hash(a: &[u8]) -> [u8; N] {
    // create a SHA3-256 object
    let mut count = 0;
    let mut buf: [u8; 32] = [0; 32];
    let mut u2: [u8; N] = [0; N];

    for _i in 0..a.len() {
        buf[_i % 32] ^= a[_i];
    }

    for _i in 0..8 {
        let mut hasher = Sha3_256::default();
        //me=hasher.clone();
        hasher.update(buf);
        // read hash digest
        let mut result = hasher.finalize();

        for _i in 0..32 {
            buf[_i] ^= result[_i];
            u2[count % N] = result[_i];
            //print!("{},",result[i]);
            count = count + 1;
        }
        //println!("");
    }

    u2
}

fn perm_32(a: &[u8]) -> [u8; 32] {
    // create a SHA3-256 object
    let mut count = a.len();
    let mut buf: [u8; 32] = [0; 32];
    let mut u2: [u8; N] = [0; N];

    for _i in 0..count {
        buf[_i % 32] ^= a[_i];
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
    /*
       let mut hasher = Keccak256::default();
       hasher.update(K1);
       let result: Vec<u8> = hasher.finalize().to_vec();
       //let be:String=String::from_utf8(result).unwrap();
       for x in &result {
           print!("{:0x}", x);
       }
       println!();
    */
    let mut o: &[u8] = &buf;
    //println!("oh={:x}",o);

    buf
}

fn mlt(x: u16, y: u16) -> u16 {
    if x == 0 || y == 0 {
        // println!("is 0");
        return 0;
    }

    ((x + y - 2) % (N - 1) as u16) + 1
}

fn mltn(mut n: u16, mut x: u16) -> u16 {
    let mut ret = 1;
    while n > 0 {
        if (n % 2) == 1 {
            ret = mlt(ret, x) // n の最下位bitが 1 ならば x^(2^i) をかける
        }
        x = mlt(x, x);
        n = (n >> 1) // n を1bit 左にずらす
    }

    ret
}

//有限体の元の逆数
fn oinv(a: u16) -> u16 {
    let mut i: i32 = 0;

    if a == 0 {
        return 0;
    }

    ((N - fg[a as usize] as usize) % (N - 1) + 1) as u16
}

// invert of integer
fn iinv(mut a: u16, n: u16) -> u16 {
    let mut d = 0;
    let mut q = 0;
    let mut t = 0;
    let mut r = 0;
    let mut x = 0;
    let mut s = 0;
    let mut gcd = 0;

    x = 0;
    s = 1;

    d = n;
    while a != 0 {
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

fn lot(mut z: [u8; 32]) -> [u8; 32] {
    let mut tmp: u8 = 0;

    for i in 0..32 {
        if i + 1 < 32 {
            tmp = z[i];
            z[i] = z[(i + 1)];
            z[i + 1] = tmp;
        }
    }
    z
}

fn rot(mut z: [u8; 32]) -> [u8; 32] {
    let mut tmp: u8 = 0;

    for i in (0..32).rev() {
        //tmp=z[i];
        if (i) > 0 {
            tmp = z[i];
            z[(i) % 32] = z[(i - 1) % 32];
            z[(i - 1) % 32] = tmp;
        }
        if i == 0 {
            z[0] = tmp;
        }
    }
    z
}

fn lot2(mut z: [u8; E]) -> [u8; E] {
    let mut tmp: u8 = 0;

    for i in 0..E {
        if i + 1 < E {
            tmp = z[i];
            z[i] = z[(i + 1)];
            z[i + 1] = tmp;
        }
    }
    z
}

fn rot2(mut z: [u8; E]) -> [u8; E] {
    let mut tmp: u8 = 0;

    for i in (0..E).rev() {
        //tmp=z[i];
        if (i) > 0 {
            tmp = z[i];
            z[(i) % E] = z[(i - 1) % E];
            z[(i - 1) % E] = tmp;
        }
        if i == 0 {
            z[0] = tmp;
        }
    }
    z
}

fn Lot(mut z: [u8; N]) -> [u8; N] {
    let mut tmp: u8 = 0;

    for i in 0..N {
        if i + 1 < N {
            tmp = z[i];
            z[i] = z[(i + 1)];
            z[i + 1] = tmp;
        }
    }
    z
}

fn Rot(mut z: [u8; N]) -> [u8; N] {
    let mut tmp: u8 = 0;

    for i in (0..N).rev() {
        //tmp=z[i];
        if (i) > 0 {
            tmp = z[i];
            z[(i) % E] = z[(i - 1) % N];
            z[(i - 1) % E] = tmp;
        }
        if i == 0 {
            z[0] = tmp;
        }
    }
    z
}

fn v2m(m: [u8; N]) -> Array2<u8> {
    let mut mat: Array2<u8> = Array2::zeros((E, E));
    let mut kt:[u8;32]=[0;32]; //xorshift256();
    unsafe{
        xx = 0x180ec6d33cfd0aba;
        yy = 0xd5a61266f0c9392c;
        zz = 0xa9582618e03fc9aa;
        ww = 0x39abdc4529b1661c;  // 全ゼロ以外の値
        }    
    for i in 0..E {
        kt=xorshift256();
        for j in 0..E {
            mat[[i, j]] = m[i * E + j]^kt[j];
        }
    }

    mat
}

fn v2b(mut t: [u8; BIT]) -> [u8; BIT] {
    let mut tmp: [u8; BIT] = [0; BIT];
    let mut j = 0;

    while j < BIT {
        for i in 0..BIT {
            tmp[j] = (tmp[j] << 1);
            tmp[j] ^= t[i] % 2;
            t[i] = (t[i] >> 1);
        }
        j += 1;
    }
    tmp
}

fn b2v(u: [u8; BIT]) -> [u8; BIT] {
    let mut tmp: [u8; BIT] = [0; BIT];
    let mut f = 0;

    for i in 0..BIT {
        for j in 0..BIT {
            f = u[j];
            tmp[i] ^= ((u[j] >> (BIT - i - 1) & 1) << j);
            //print!("`{}",(u[j]>>(4-i-1))&1);
        }
    }

    tmp
}

fn mta(m: Array2<u8>) -> Array2<u8> {
    let mut g: Array2<u8> = Array2::zeros((BIT, BIT));

    for i in 0..BIT {
        for j in 0..BIT {
            for k in 0..BIT {
                g[[i, j]] ^= gf[mlt(fg[m[[i, k]] as usize] as u16, fg[MDS8[k][j] as usize] as u16) as usize];
            }
        }
    }
    //println!("{:?}",g);
    //exit(1);

    g
}

fn atm(m: Array2<u8>) -> Array2<u8> {
    let mut g: Array2<u8> = Array2::zeros((BIT, BIT));

    for i in 0..BIT {
        for j in 0..BIT {
            for k in 0..BIT {
                g[[i, j]] ^= gf[mlt(fg[m[[i, k]] as usize] as u16, fg[inv8[k][j] as usize] as u16) as usize];
            }
        }
    }
    g
}

fn a2b(mut a: [u8; N]) -> [u8; N] {
    let mut g: [u8; BIT] = [0; BIT];
    let mut tmp: [u8; BIT] = [0; BIT];
    let mut f: [u8; N] = [0; N];

    for i in 0..N / BIT {
        for j in 0..BIT {
            tmp[j] = a[i * BIT + j];
        }
        println!("tmp={:?}", tmp);
        tmp = v2b(tmp);
        println!("tmp={:?}", tmp);
        for k in 0..BIT {
            tmp[k] = bite(tmp[k] as usize, k);
        }
        println!("tmp={:?}", tmp);

        for j in 0..BIT {
            //    g[j] = 0;
            for k in 0..BIT {
                g[j] ^= gf[mlt(fg[tmp[k] as usize] as u16, fg[MDS8[k][j] as usize] as u16) as usize];
            }
        }

        for j in 0..BIT {
            f[i * BIT + j] = tmp[j];
        }
    }
    f
}

fn b2a(mut a: [u8; N]) -> [u8; N] {
    let mut g: [u8; BIT] = [0; BIT];
    let mut tmp: [u8; BIT] = [0; BIT];
    let mut f: [u8; N] = [0; N];

    for i in 0..N / BIT {
        for j in 0..BIT {
            tmp[j] = a[i * BIT + j];
        }

        for j in 0..BIT {
            //g[j] = 0;
            for k in 0..BIT {
                g[j] ^= gf[mlt(fg[tmp[k] as usize] as u16, fg[inv8[k][j] as usize] as u16) as usize];
            }
        }

        for k in 0..BIT {
            tmp[k] = u2(tmp[k] as u8, k);
        }
        tmp = b2v(tmp);
        println!("inV={:?}", tmp);
        //exit(1);
        for j in 0..BIT {
            f[i * BIT + j] = tmp[j];
        }
    }
    f
}

fn m2b(m: Array2<u8>) -> Array2<u8> {
    let mut mat: Array2<u8> = Array2::zeros((BIT, BIT));
    let mut tmp: [u8; BIT] = [0; BIT];

    for jj in 0..BIT {
        for i in 0..BIT {
            tmp[i] = m[[jj, i]];
        }
        tmp = v2b(tmp);
        for k in 0..BIT {
            tmp[k] = bite(tmp[k] as usize, jj);
        }
        for i in 0..BIT {
            mat[[jj, i]] = tmp[i];
        }
    }
    //
    mat = mta(mat);
    //println!("{:?}",mat);
    //exit(1);

    mat
}

fn b2m(m: Array2<u8>) -> Array2<u8> {
    let mut mat: Array2<u8> = Array2::zeros((BIT, BIT));
    let mut tmp: [u8; BIT] = [0; BIT];
    let mut tmp2: [u8; N] = [0; N];

    mat = atm(m);

    for jj in 0..BIT {
        for i in 0..BIT {
            tmp[i] = mat[[jj, i]];
        }
        for k in 0..BIT {
            tmp[k] = u2(tmp[k], jj);
        }
        tmp = b2v(tmp);
        for i in 0..BIT {
            mat[[jj, i]] = tmp[i];
        }
    }
    //println!("{:?}",mat);
    //exit(1);
    mat
}

fn v2t(m: [u8; N]) -> Array2<u8> {
    let mut mat: Array2<u8> = Array2::zeros((E, E));
    for i in 0..E {
        for j in 0..E {
            mat[[j, i]] = m[i * E + j];
        }
    }

    mat
}

fn m2v(m2: Array2<u8>) -> [u8; N] {
    let mut r1: [u8; N] = [0; N];
    let mut kt:[u8;32]=[0;32]; //xorshift256();

    unsafe{
        xx = 0x180ec6d33cfd0aba;
        yy = 0xd5a61266f0c9392c;
        zz = 0xa9582618e03fc9aa;
        ww = 0x39abdc4529b1661c;  // 全ゼロ以外の値
        }    
    for i in 0..E {
        kt=xorshift256();
        for j in 0..E {
            r1[i * E + j] = m2[[i, j]]^kt[j];
        }
    }

    r1
}

fn shift(sf: Array2<u8>) -> Array2<u8> {
    let mut v: [u8; E] = [0; E];
    let mut mat: Array2<u8> = Array2::zeros((E, E));

    for j in 0..E {
        for i in 0..E {
            v[i] = sf[[j, i]];
        }
        for ii in 0..j {
            v = lot2(v);
        }
        for k in 0..E {
            mat[[j, k]] = v[k];
        }
    }

    mat
}

fn rev_shift(sf: Array2<u8>) -> Array2<u8> {
    let mut v: [u8; E] = [0; E];
    let mut mat: Array2<u8> = Array2::zeros((E, E));

    for j in 0..E {
        for i in 0..E {
            v[i] = sf[[j, i]];
        }
        for ii in 0..j {
            v = rot2(v);
        }
        for k in 0..E {
            mat[[j, k]] = v[k];
        }
    }

    mat
}

struct DummyRng {
    state: u64,
}
impl DummyRng {
    fn new(seed: u64) -> Self {
        DummyRng { state: seed }
    }
    fn next(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 2;
        self.state = x;
        x
    }
}

// make the (K,K)-MDS matrix
fn van2() {
    let i: u16;
    let j: u16;
    let mut vb: Array2<u8> = Array2::zeros((K, N));
    //let mut vb:Array2<u16>= arr2(&([[8,16]]));
    println!("van der");

    for i in 0..(N) {
        vb[[0, i]] = 1;
    }
    //#pragma omp parallel for private(i, j)
    for i in 0..K {
        for j in 0..68 {
            vb[[i, j]] = gf[mltn(i as u16, fg[j as usize] as u16) as usize];
            print!("{},", vb[[i, j]]);
        }
        print!("\n");
    }
}

fn mulm(ma: Array2<u8>) -> Array2<u8> {
    let mut nn: Array2<u8> = Array2::zeros((E, E));
    //let mut me: [u8; 256] = [0; 256];
    //exit(1);

    for i in 0..E {
        for j in 0..E {
            for k in 0..E {
                nn[[i, j]] ^= gf[mlt(fg[MDS16[i][k] as usize] as u16, fg[ma[[k, j]] as usize] as u16) as usize];
            }
        }
    }
    println!("{:?}", nn);
    //exit(1);

    nn
}

fn invm(ma: Array2<u8>) -> Array2<u8> {
    let mut nn: Array2<u8> = Array2::zeros((E, E));

    //let mut me: [u8; 256] = [0; 256];

    //exit(1);

    for i in 0..E {
        for j in 0..E {
            for k in 0..E {
                nn[[i, j]] ^= gf[mlt(fg[inv16[i][k] as usize] as u16, fg[ma[[k, j]] as usize] as u16) as usize];
            }
        }
    }
    nn
}

fn permute(u: [u8; 32], mut u2: [u8; 32], n: i32) -> [u8; 32] {
    let mut tmp: [u8; 32] = [0; 32];
    let mut nk: [u8; 32] = [0; 32];
    let mut nk2: [u8; 32] = [0; 32];

    for j in 0..n {
        for i in 0..32 {
            tmp[i] = u[u2[i] as usize];
        }
        for i in 0..32 {
            u2[i] = tmp[i];
        }
    }

    u2
}

fn rebirth(inv1: [u8; 32], mut nk: [u8; 32], n: i32) -> [u8; 32] {
    let mut tmp: [u8; 32] = [0; 32];

    for j in 0..n {
        for i in 0..32 {
            tmp[i] = inv1[nk[i] as usize];
        }
        for k in 0..32 {
            nk[k] = tmp[k];
        }
    }

    nk
}

fn addround(mut key: [u64; 8], l: i32) -> [u64; 8] {
    let mut round: [u64; 8] = [0; 8];

    for i in 0..l {
        for j in 0..8 {
            key[j] = key[(j + 7) % 8] ^ key[j];
        }
        for j in 0..8 {
            round[j] ^= key[j];
        }
    }

    round
}

fn llu2b(mut l: u64) -> [u8; 8] {
    let mut uu: [u8; 8] = [0; 8];
    let mut count = 0;
    while l > 0 {
        uu[count] = (l % N as u64) as u8;
        count = count + 1;
        l = (l >> 8);
    }

    uu
}

fn gmult(mut a: u8, mut b: u8) -> u8 {
    let mut c: u8 = 0;
    let mut i: u8;
    let mut msb: u8;

    for i in 0..8 {
        if (b & 1) == 1 {
            c ^= a;
        }
        msb = a & 0x80;
        a <<= 1;
        if (msb == 1) {
            a ^= 0x1b;
        }
        b >>= 1;
    }

    c
}

fn b2w(b: [u8; 4]) -> u32 {
    let mut temp: u32 = 0;

    for i in 0..4 {
        temp = (temp << 8);
        temp ^= b[i] as u32;
    }
    temp
}

fn w2b(mut w: u32) -> [u8; 4] {
    let mut tmp: [u8; 4] = [0; 4];

    for i in 0..4 {
        tmp[i] = (tmp[i] << 8);
        tmp[i] ^= (w % N as u32) as u8;
        w = (w >> 8);
    }
    tmp
}

fn rot_word(word: u32) -> u32 {
    /* a3 a2 a1 a0 -> a0 a3 a2 a1 */
    (word << 24 | word >> 8)
}

fn add_round_key(state: [u8; E] /*4*Nb*/, mut w: [u32; 4] /*Nb*(Nr+1)*/) -> [u32; 4] {
    let mut i: i32;
    let mut m: u32 = 0;

    let mut s: [u32; 4] = [0; 4]; //state;
    for i in 0..4 {
        for j in 0..4 {
            m = (m << 8);
            m ^= state[i * 4 + j] as u32;
        }
        s[i] = m;
        m = 0;
    }
    for i in 0..4 {
        s[i] ^= w[i];
    }

    s
}

fn sub_word(word: u32) -> u32 {
    let mut val: u32 = word;
    let mut p: [u8; 4] = [0; 4];
    for i in 0..4 {
        p[i] = (val % N as u32) as u8;
        val = (val >> 8)
    } //&val;
    p[0] = S_BOX[p[0] as usize];
    p[1] = S_BOX[p[1] as usize];
    p[2] = S_BOX[p[2] as usize];
    p[3] = S_BOX[p[3] as usize];

    val
}

fn key_expansion(key: [u32; 8] /*Nk*/, mut w: [u32; 60] /*Nb*(Nr+1)*/) -> [u32; 60] {
    let i: i32;
    let Nr: u8 = 14;
    let Nk: u8 = 8;
    for i in 0..8 {
        w[i] = key[i];
    }

    //memcpy(w, key, Nk*4);
    for i in 8..60 {
        let mut temp: u32 = w[i - 1];
        if i % 8 == 0 {
            println!("aii={}", i);
            temp = sub_word(rot_word(temp)) ^ rcon[(i) / 8];
        } else if (6 < 8 && i % 14 == 4) {
            temp = sub_word(temp);
        }
        w[i] = w[i - 8] ^ temp;
    }

    w
}

fn bite(a: usize, i: usize) -> u8 {
    let mut u: [u8; 2] = [0; 2];
    //let mut Sbox: Array2<u8> = Array2::zeros((8, 16));
    let mut t = 0;

    u[0] = Sbox[i % 8][a % E];
    u[1] = Sbox[i % 8][(a >> 4)];
    t = (u[1] << 4) ^ u[0];
    t
}

fn u2(ua: u8, i: usize) -> u8 {
    let mut r: [u8; 2] = [0; 2];
    let mut e = 0;

    r[0] = InvS[i % 8][(ua % E as u8) as usize];
    r[1] = InvS[i % 8][(ua >> 4) as usize];

    e = (r[1] << 4) ^ r[0];

    e
}

// inverse matrix
/*
fn matinv(za:Array2<u8>, en:i32)
{

  // unsigned short a[F][F];     //={{1,2,0,1},{1,1,2,0},{2,0,1,1},{1,2,1,1}}; //入力用の配列
  let mut inv_a:Array2<u8>=Array2::zeros(N,N); //[N][N]={0};   //ここに逆行列が入る
  let mut buf:i32=0;           //一時的なデータを蓄える
 let count=0;           //カウンタ
  // MTX a={0};
  unsigned short c[N][N] = {0};
  MTX z = {0};
  // unsigned short cc[N][N] = {0};

 unsigned short bb[N][N]={0};


for(i=0;i<8;i++){
  for(j=0;j<8;j++){
  bb[i][j]=za.x[i][j];
    printf("%d,",bb[i][j]);
  }
  printf("\n");
}
//exit(1);


printf("n=%d\n",en);
 en=8;
lab:
  for (i = 0; i < en; i++)
  {
    for (j = 0; j < en; j++)
      c[i][j] = bb[i][j]; //=b[i][j];
  }

  //memcpy(bb,bb[,sizeof(bb));

  //単位行列を作る
  for (i = 0; i < en; i++)
  {
    for (j = 0; j < en; j++)
    {
      inv_a[i][j] = (i == j) ? 1 : 0;
    }
  }
  //掃き出し法
  //#pragma omp parallel for num_threads(omp_get_max_threads()) //private(i,j,k)
  for (i = 0; i < en; i++)
  {
    buf = gf[oinv(bb[i][i])];
    for (j = 0; j < en; j++)
    {
      bb[i][j] = gf[mlt(fg[buf], fg[bb[i][j]])];
      inv_a[i][j] = gf[mlt(fg[buf], fg[inv_a[i][j]])];
    }
    for (j = 0; j < en; j++)
    {
      if (i != j)
      {
        buf = bb[j][i];
        for (k = 0; k < en; k++)
        {
          bb[j][k] ^= gf[mlt(fg[bb[i][k]], fg[buf])];
          inv_a[j][k] ^= gf[mlt(fg[inv_a[i][k]], fg[buf])];
        }
      }
    }
  }
for(i=0;i<en;i++){
  for(j=0;j<en;j++)
  printf("%3d,",inv_a[i][j]);
  printf("\n");
}
  memset(bb, 0, sizeof(bb));
  //検算
  //  #pragma omp parallel for num_threads(omp_get_max_threads()) //private(i,j,k)
  for (i = 0; i < en; i++)
  {
    for (j = 0; j < en; j++)
    {
      for (k = 0; k < en; k++)
        bb[i][j] ^= gf[mlt(fg[c[i][k]], fg[inv_a[k][j]])];

      printf("^%d,", bb[i][j]);
    }
    printf("\n");
  }

  int flg = 0;
  for (i = 0; i < en; i++)
  {
    //   printf("%d",b[i][i]);
    // printf("==\n");
    if (bb[i][i] == 1)
    {
      // printf("baka");
      //    exit(1);
      flg++;
    }
  }

printf("flg==%d\n",flg);
//exit(1);
  // printf("\n\n逆行列を出力\n");
  for (i = 0; i < en; i++)
  {
    count = 0;
    for (j = 0; j < en; j++)
    {
      if (inv_a[i][j] == 0)
        count++;
      if (count == en)
      {
        printf("\nbaka\n\n");
        goto lab;
      }
      printf(" %d", inv_a[i][j]);
      z.x[i][j] = inv_a[i][j];
    }
     printf("\n");
  }

if(flg==en){
  count = 0;

  for (i = 0; i < en; i++)
  {
    for (j = 0; j < en; j++)
    {
      if (bb[i][j] == 0 && i != j)
        count++;
    }
  }
  if (flg == en && (en * en - en) == count){
    return z;
  }
    //
}

}
 */

/*
fn u3(a:[u8;256])->[u8;512]{
    let uu:[u8;512]=[0;512];
    let t:[u8;2]=[0;2];
    i=0;
    while i < 256 {
        t=bite(a[i]);
        uu[i]=t[0];
        uu[i+1]=t[1];
        i+=2;
    }
    uu
}
 */

fn round(mut buf: [u8; N], be: [u8; 32], a: &[u8; N], mut nk: [u8; 32], mat: &Array2<u8>, _k: usize) -> [u8; N] {
    for _i in 0..N {
        buf[_i] ^= be[nk[_i % 32] as usize];
        buf[_i] = S_BOX[((buf[_i] % 16) + (buf[_i] >> 4) * 16) as usize];
        buf[_i] = a[buf[_i] as usize] as u8;
        buf[_i] = mat[[a[(16 * _k + _i) % N as usize] as usize, (buf[_i as usize]) as usize]];
    }
    buf
}

fn invs(decoded: Vec<u8>) {
    let mut buf: [u8; N] = [0; N];

    for i in 0..N {
        buf[i] = decoded[i];
    }
}

static mut xx:u64 = 0x180ec6d33cfd0aba;
static mut yy:u64 = 0xd5a61266f0c9392c;
static mut zz:u64 = 0xa9582618e03fc9aa;
static mut ww:u64 = 0x39abdc4529b1661c;  // 全ゼロ以外の値。種。
fn xorshift256()->[u8;32] {
    
    let mut w:u64=0;
    let mut x:u64=0;
    let mut y:u64=0;
    let mut z:u64=0;
    unsafe{
    let mut t:u64 = xx ^ (xx << 11);
    xx = yy; yy = zz; zz = ww;
    ww = (ww ^ (ww >> 19)) ^ (t ^ (t >> 8));
    w=ww;
    z=zz;
    y=yy;
    x=xx;
}
//println!("{:?},{:?},{:?},{:?}",x,y,z,w);
//exit(1);
let mut u:[u8;32]=[0;32];
for i in 0..8{
    u[i]=(w%(256 as u64)) as u8;
    w=(w>>8);
}

for i in 0..8{
    u[i+8]=(x%(256 as u64)) as u8;
    x=(x>>8);
}
for i in 0..8{
    u[i+16]=(y%(256 as u64)) as u8;
    y=(y>>8);
}
for i in 0..8{
    u[i+24]=(z%(256 as u64)) as u8;
    z=(z>>8);
}

    u
  }
  
// 変数はすべて64ビット整数とする
fn xorshift128plus()->[u8;16]{
  let mut state0:u64 = 123456789;
  let mut state1:u64 = 362436069;    
  let mut buf:[u8;16]=[0;16];
  let mut tt:u64=0;
  let mut x:u64=123456789;
  let mut y:u64=362436069;
  let mut z:u64=521288629;
  let  w:u64=88675123;
  let  t:u64=(x^(x<<11));
  x=y;
  y=z;
  z=w;
 tt= (w^(w>>19))^(t^(t>>8));
 println!("tt={:?}",tt);
 for i in 0..16{
    buf[i]=(tt%256) as u8;
    tt=(tt>>8);
 }

 buf
}


fn enc(data: &String, a: &[u8; N], mat: &Array2<u8>, seed2: [u8; 32]) -> String {
    /*
     * S-box transformation table
     */
    // s-box from serpent

    let mut buf: [u8; N] = [0; N];
    // let mut byte = data.as_bytes();
    let mut byte = decode(&data).unwrap();
    //let seed2 = "kotobahairanai".as_bytes();

    let mut seed = seed2.clone();
    let mut bb: [u8; 8] = [17; 8];
    //seed=p2(&seed2);
    let mut rng2: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed);
    println!("len = {}", byte.len());
    println!("origin: {}", str::from_utf8(data.as_bytes()).unwrap());
    let mut mat3: Array2<u8> = Array2::zeros((E, E));
    //let mut aia: Array2<u8> = Array2::zeros((E, E));
    let mut mat2: Array2<u8> = Array2::zeros((E, E));

    //let mut me: [u8; N] = [11; N];
    let cycle = rng2.gen_range(1..N);
    let mut count = 0;
    let j = byte.len();
    let mut be = seed2.clone();
    let mut it: [u8; N] = [0; N];



    //for i in 0..N {
    //    it[i] = a[i];
    //}
    for i in 0..32 {
        seed[i] = i as u8;
    }
    seed = shuffule(seed, 32, &be);
    //println!("{:?}",mat);
    //exit(1);
    for _i in 0..3 {
        be = perm_32(&be);
        println!("{:?}", be);
    }
    
    //byteは参照型なので配列に置き換える
    for i in 0..j {
        buf[i] = byte[i];
    }

    let mut trim: [u8; 8] = [0; 8];

    let mut nk: [u8; 32] = [0; 32];
    for i in 0..32 {
        nk[i] = i as u8;
    }
    let mut kt:[u8;16]=[0;16];
    let mut beef: [u8; 64] = [0; 64];
    //let mut _k:usize;
    for _k in 0..16 {
    
        mat3 = v2m(buf);

        mat3 = shift(mat3);
        mat3 = mulm(mat3);
        buf = m2v(mat3);
        buf = a2b(buf);
        nk = permute(seed, nk, 1);
        println!("{:?}", nk);
        buf = round(buf, be, a, nk, &mat, _k);
        for i in 0..N / 8 {
            for j in 0..8 {
                trim[j] = buf[i * 8 + j];
                trim[j] = bite(trim[j] as usize, i);
            }
            //trim = v2b(trim);
            trim = schedule(trim);
            for j in 0..8 {
                buf[i * 8 + j] = trim[j];
            }
        }

        println!("{:?}", buf);


        for o in 0..8 {
            bb[o] = seed2[(_k + o) % 32];
        }
        for o in 0..N / 64 {
            bb = aha(bb);
            beef = expand(bb);

            for p in 0..64 {
                buf[o * 64 + p] ^= beef[p];
            }
        }
    }
    //exit(1);

    println!("encrypted = {:?}", &buf[0..N]); //j]);

    let encoded = encode(&buf[0..N]); //j]);
    let enc = encoded.clone();
    println!("cipher text:");
    println!("{:?}", encoded);
    //exit(1);

    encoded
}

fn dec(encoded: &String, a: &[u8; N], mat: &Array2<u8>, seed2: [u8; 32]) -> String {
    let mut buf: [u8; N] = [0; N];

    let mut decoded = decode(&encoded).unwrap();
    let mut inv_P: [usize; N] = [0; N];
    //let mut tmp: [u8; N] = [11; N];
    //let mut seed2=b"kotobahairanai";
    let mut t2: [u8; N] = [0; N];
    let mut seed = seed2.clone();
    let mut bb: [u8; 8] = [17; 8];
    //seed=p2(seed2);
    let mut rng2: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed);
    //let aa: String = "kotobahairanai".to_string();
    //let v:Vec<u8>=aa.to_vec();
    let mut count = 0;
    let cycle = rng2.gen_range(1..N);
    let mut it: [u8; N] = [0; N];
    let mut mat2: Array2<u8> = Array2::zeros((16, 16));
    //let mut xount: i32 = 0;

    println!("len = {}, {}", decoded.len(), cycle);

    for i in 0..N {
        inv_P[a[i as usize] as usize] = i as usize;
    }
    let l = decoded.len();
    println!("{}", l);
    //exit(1);

    let _size: usize = 32;
    //let mut result:[u8;256]=[0;256];

    for i in 0..32 {
        seed[i] = i as u8;
    }
    let mut be = seed2.clone();
    seed = shuffule(seed, 32, &be);

    let mut inv2: [u8; 32] = [0; 32];
    for i in 0..32 {
        inv2[seed[i] as usize] = i as u8;
    }
    /*
    for i in 0..16{
        seed=lot(seed);
    }
    */
    let mut ee: [u8; 32] = [0; 32];
    for i in 0..32 {
        ee[i] = i as u8;
    }
    ee = permute(seed, ee, 16);

    for i in 0..3 {
        be = perm_32(&be);
    }

    let mut trim: [u8; 8] = [0; 8];
    let mut beef: [u8; 64] = [0; 64];
    //exit(1);
    for j in (0..16) {
        // read hash digest
        //be=p2(&be);

        for o in 0..8 {
            bb[o] = seed2[((15 - j) + o) % 32]; //beef[o];
        }
        for o in 0..N / 64 {
            bb = aha(bb);
            beef = expand(bb);

            for p in 0..64 {
                decoded[o * 64 + p] ^= beef[p];
            }
        }

        let mut trim: [u8; 8] = [0; 8];
        for i in 0..N / 8 {
            for j in 0..8 {
                trim[j] = decoded[i * 8 + j];
            }
            trim = invsche(trim);
            //trim = b2v(trim);

            for j in 0..8 {
                trim[j] = u2(trim[j], i);
                decoded[i * 8 + j] = trim[j];
            }
        }
        //t2=invs(decoded);
        for i in (0..l) {
            decoded[i] = mat[[it[(16 * j + i) % N as usize] as usize, (decoded[i as usize]) as usize]];
            decoded[i] = (inv_P[decoded[i] as usize] as usize) as u8;
            decoded[i] = INV_S_BOX[(((decoded[i] % 16) + (decoded[i] >> 4) * 16) as usize)];
            decoded[i] ^= be[ee[i % 32] as usize];
            t2[i] = decoded[i];
        }
        let mut buff: [u8; N] = [0; N];
        ee = rebirth(inv2, ee, 1);


        t2 = b2a(t2);
  

        mat2 = v2m(t2);

        //

        println!("ppp{:?}", mat2);
        //exit(1);
        //mat2=b2m(mat2);
        mat2 = invm(mat2);

        println!("xxx{:?}", mat2);
        //exit(1);

        mat2 = rev_shift(mat2);

        t2 = m2v(mat2);
    
        for ii in 0..l {
            decoded[ii] = t2[ii];
        }
        println!("~~~{:?}", decoded);

    }

    //println!("{:?}",decoded);
    //exit(1);
    let mut om = 0;
    for i in 0..l {
        buf[i] = decoded[i];
        if (buf[i] > 0) {
            om = om + 1;
        }
    }
    println!("{:?}", &buf[0..om]);
    //exit(1);
    //buf[i]^=be[(i+1)%32];

    let v: Vec<u8> = vec![1, 2, 3];

    println!("plain text:");
    println!("decrypted = {:?}", &buf[0..om]);

    match String::from_utf8(buf.to_vec()) {
        Err(_why) => {
            println!("復号できませんでした");
            "baka".to_string()
        }
        Ok(str) => encode(&str[0..om]),
    }
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

fn mkiv(nonce: &[u8], key: [u8; 32]) -> Vec<u8> {
    //let m:&[u8]=message.as_bytes();
    let mut hasher = Keccak256::default();
    let mut k1: Vec<u8> = key.to_vec();
    let mut K1: Vec<u8> = vec![];
    let mut K2: Vec<u8> = vec![];
    let mut IV: Vec<u8> = vec![];

    K1.write_all(&k1).unwrap();
    K1.write_all(nonce).unwrap();

    hasher.update(K1);
    let result = hasher.finalize();
    K2.write(&result.to_vec()).unwrap();
    //  let result2: Vec<u8> = hasher.finalize().to_vec();
    let mut hasher = Keccak256::default();
    hasher.update(K2);
    let result2 = hasher.finalize();
    IV.write(&result2.to_vec()).unwrap();

    for IV in &result {
        print!("{:0x}", &IV);
    }
    println!();

    IV
}

fn aha(d: [u8; 8]) -> [u8; 8] {
    let mut hasher = Sha3_256::new();
    let mut v: [u8; 8] = [0; 8];
    hasher.update(d);
    let result = hasher.finalize();
    //v.write(&result.to_vec()).unwrap();
    for i in 0..8 {
        v[i] = result[i];
        print!("{:0x}", v[i]);
    }
    println!();

    v
}

fn v2u(bytes: &[u8]) -> Vec<u8> {
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

fn ae(cc: String, seed2: [u8; 32]) -> Vec<u8> {
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

fn v2vs(src: Vec<u8>) -> Vec<String> {
    // 整数型
    // 要素を整数型から文字列型変換
    let dst: Vec<String> = src.iter().map(|x| x.to_string()).collect();
    // 文字列型vectorを区切り文字で結合
    println!("{}", dst.join(" "));

    dst
}
use std::fs::File;
use std::io::{BufReader, Read};

fn fread() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = BufReader::new(File::open("./README.md")?);
    let mut buf = [0; 32];

    loop {
        match reader.read(&mut buf)? {
            0 => break,
            n => {
                let buf = &buf[..n];
                println!("{:?}", buf);
            }
        }
    }

    Ok(())
}

fn vmul(a: Vec<u8>, ma: Array2<u8>, k: i32) -> Vec<u8> {
    let mut b: Vec<u8> = Vec::new();

    for i in 0..k {
        for j in 0..k {
            b[i as usize] ^= gf[mlt(
                fg[ma[[i as usize, j as usize]] as usize] as u16,
                fg[a[j as usize] as usize] as u16,
            ) as usize] as u8;
        }
    }

    b
}

fn mmat(a: Array2<u8>, b: Array2<u8>, l: i32) {
    let mut tmp: Array2<u8> = Array2::zeros((E, E));
    let mut i: usize;
    let mut j: usize;
    let mut k: usize;

    //#pragma omp parallel for num_threads(4) // private(i,j,k)
    for i in 0..l {
        for j in 0..l {
            for k in 0..l {
                tmp[[i as usize, j as usize]] ^= gf[mlt(
                    fg[a[[i as usize, k as usize]] as usize] as u16,
                    fg[b[[k as usize, j as usize]] as usize] as u16,
                ) as usize] as u8;
            }
        }
    }

    for i in 0..l {
        for j in 0..l {
            print!(" {},", tmp[[i as usize, j as usize]]);
        }
        println!("");
    }
}

fn schedule(v: [u8; BIT]) -> [u8; 8] {
    let mut trim: [u8; 8] = [0; 8];
    let mut s: [u8; 8] = [0; 8];

    trim = v2b(v);
    for i in 0..BIT {
        trim[i] = S_BOX[(((trim[i] % 16) as usize + ((trim[i] >> 4) * 16) as usize) as usize)];
    }

    for i in 0..BIT {
        s[i] = 0;
        for j in 0..BIT {
            s[i] ^= gf[mlt(fg[trim[j] as usize] as u16, fg[MDS8[j][i] as usize] as u16) as usize]
        }
    }

    s
}

fn b2b(mut buff: [u8; N], ii: i32) -> [u8; N] {
    let mut buf: [u8; 8] = [0; 8];

    for i in 0..N / 8 {
        for j in 0..8 {
            buf[j] = buff[i * 8 + j]
        }
        buf = v2b(buf);
        for j in 0..8 {
            buff[i * 8 + j] = bite(buf[j] as usize, ii as usize);
        }
    }

    buff
}

fn bb2(mut buff: [u8; N], ii: i32) -> [u8; N] {
    let mut buf: [u8; 8] = [0; 8];

    for i in 0..N / 8 {
        for j in 0..8 {
            buf[j] = u2(buff[i * 8 + j], ii as usize);
        }
        buf = b2v(buf);
        for j in 0..8 {
            buff[i * 8 + j] = buf[j];
        }
    }

    buff
}

fn invsche(v: [u8; BIT]) -> [u8; 8] {
    let mut trim: [u8; 8] = [0; 8];
    let mut s: [u8; 8] = [0; 8];

    for i in 0..BIT {
        s[i] = 0;
        for j in 0..BIT {
            s[i] ^= gf[mlt(fg[v[j] as usize] as u16, fg[inv8[j][i] as usize] as u16) as usize]
        }
    }

    for i in 0..BIT {
        trim[i] = INV_S_BOX[(((s[i] % 16) as usize + ((s[i] >> 4) * 16) as usize) as usize)];
    }
    trim = b2v(trim);

    trim
}

fn expand(mut key: [u8; 8]) -> [u8; 64] {
    let mut exkey: Vec<u8> = Vec::new();
    let mut tmp: [u8; 64] = [0; 64];
    exkey = schedule(key).to_vec();

    for j in 0..64 {
        tmp[j] = 0;
        for i in 0..8 {
            tmp[j] ^= gf[mlt(fg[exkey[i] as usize] as u16, fg[rs[i][j] as usize] as u16) as usize];
        }
    }
    tmp
}


fn main() {
    //let mut key:[u8;256]=[0;256];
    let mut data = String::new(); //from("日本語入力"svan());
    let mut mat: Array2<u8> = Array2::zeros((N, N));
    let mut sk: [u8; N] = [0; N];
    let mut mat2: Array2<u8> = Array2::zeros((N, N));
    //let mut buf: [u8; N] = [0; N];
    let mut seed2: [u8; 32] = [17; 32];
    let mut rng2: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed2);
    let nonce: &[u8] = ("kotobahairanai").as_bytes();
    //let bytes: &[u8] = nonce.as_bytes();
    let mut seed = perm_32(nonce); //rng2.gen_range(1..256);
    let mut cc: String = String::new();
    let mut l: String = String::new();
    let iv: &[u8] = "aaaaaa".as_bytes();
    let mut kt:[u8;32]=[0;32];

    unsafe{
        xx = 0x180ec6d33cfd0aba;
        yy = 0xd5a61266f0c9392c;
        zz = 0xa9582618e03fc9aa;
        ww = 0x39abdc4529b1661c;  // 全ゼロ以外の値
        }    
    for i in 0..10{
        kt=xorshift256();
        println!("{:?}",kt);
    }
    unsafe{
        xx = 0x180ec6d33cfd0aba;
        yy = 0xd5a61266f0c9392c;
        zz = 0xa9582618e03fc9aa;
        ww = 0x39abdc4529b1661c;  // 全ゼロ以外の値
        }
        for i in 0..10{
        kt=xorshift256();
        println!("{:?}",kt);
    }
    //exit(1);

    //van2();
    //exit(1);
    for _j in 0..N {
        for _i in 0..N {
            sk[_i] = _i as u8;
        }
        sk = random_shuffule(sk, N as u16, &seed2);
        for _k in 0..N {
            mat[[_j, _k]] = sk[_k];
        }
    }
    //秘密鍵
    for i in 0..N {
        sk[i] = (i + 1) as u8;
    }

    for _i in 0..N {
        for _j in 0..N {
            mat2[[_i, mat[[_i, _j]] as usize]] = _j as u8;
        }
    }

    for _i in 0..N {
        sk[_i] = _i as u8;
    }
    sk = random_shuffule(sk, N as u16, &seed);

    println!("何か入力を");
    std::io::stdin().read_line(&mut data).ok();
    data = data.trim_end().to_owned();
    println!("{}", data);

    data = encode(data); //attention !!
                         // encoded below
    cc = enc(&data, &sk, &mat, seed);
    // encoded above
    l = dec(&cc, &sk, &mat2, seed);


    let code = decode(&l).unwrap();

    println!("back to origin: {:?}", v2s(code));

    exit(1);
}
