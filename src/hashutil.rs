extern crate hex;
extern crate sha2;
extern crate rand;
extern crate byteorder;

use std::mem;
use self::byteorder::{LittleEndian, WriteBytesExt};
use self::rand::Rng;
use self::sha2::Sha512Trunc256;
use self::sha2::Digest;
use hexutil;

pub fn generate_hash() -> Vec<u8> {
    let mut hasher = Sha512Trunc256::new();

    let rand_int : u64 = rand::thread_rng().gen();

    let mut bs = [0u8; mem::size_of::<u64>()];
    bs.as_mut().write_u64::<LittleEndian>(rand_int).expect("unable to write");

    hasher.input(bs);

    return Vec::from(format!("{:x}", hasher.result()));
}

pub fn hash_bytes(data: Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha512Trunc256::new();

    hasher.input(data);

    return Vec::from(format!("{:x}", hasher.result()));
}

pub fn hash_to_hex_string(data: Vec<u8>) -> String {
    let mut hasher = Sha512Trunc256::new();

    hasher.input(data);

    let result = hasher.result();
    return hexutil::add_prefix(format!("{:x}", result));
}

pub fn is_equal(hex_str: String, bytes: Vec<u8>) -> bool {
    return hex_str.to_lowercase() == hash_to_hex_string(bytes);
}
