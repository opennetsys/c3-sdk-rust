extern crate hex;
extern crate regex;
extern crate buffer;
extern crate num_bigint;

use std::str;
use self::num_bigint::{BigInt, Sign};

pub fn encode_string(s: String) -> String {
    return add_prefix(hex::encode(s).to_lowercase());
}

pub fn decode_string(hex_str: String) -> Vec<u8> {
    return hex::decode(strip_prefix(hex_str)).unwrap();
}

pub fn encode_to_string(buf: Vec<u8>) -> String {
    let bufstr = match str::from_utf8(&buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    return add_prefix(hex::encode(bufstr)).to_lowercase();
}

pub fn decode_to_string(buf: Vec<u8>) -> String {
    let decoded = hex::decode(buf).unwrap();

    let bufstr = match str::from_utf8(&decoded) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    return bufstr.to_string();
}

pub fn encode_bytes(src: Vec<u8>) -> Vec<u8> {
    return strip_prefix(encode_to_string(src)).to_owned().into_bytes()
}

pub fn decode_bytes(src: Vec<u8>) -> Vec<u8> {
    let bufstr = match str::from_utf8(&src) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    return hex::decode(bufstr).unwrap();
}

pub fn encode_big_int(bn: BigInt) -> Vec<u8> {
    // TODO: make it work
    //return add_prefix(BigInt::toBase(16)).to_lowercase();
    let _ = bn;
    return vec![0];
}

pub fn decode_big_int(hex_str: String) -> BigInt {
    // TODO: make it work
    //let hex_bytes = hex::decode(strip_prefix(hex_str)).unwrap();
    //return BigInt::new(Sign::Plus, hex_bytes);
    let _ = hex_str;
    return BigInt::new(Sign::Plus, vec![0]);
}

pub fn strip_prefix(s: String) -> String {
    let re = regex::Regex::new(r"^0x").unwrap();
    return re.replace_all(&s, "").to_string();
}

pub fn add_prefix(s: String) -> String {
    return format!("0x{}", strip_prefix(s));
}
