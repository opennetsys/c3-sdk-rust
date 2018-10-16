//use std::string::String;
extern crate hex;
extern crate regex;
extern crate buffer;

pub fn encode_string(str: String) -> String {
    return add_prefix(hex::encode(str).to_lowercase());
}

/*
pub fn decode_string(hex_str: String) -> buffer::Buffer {
    //vec![0; num_points]
    let (result, err) = hex::decode(hex_str);
    return result;
    //hex_str = strip_prefix(hex_str);
    //return Util::hex2ByteArray($string);
}
pub fn encode_to_string(buf: buffer::Buffer) {
//return tolower(add_prefix(byteArray2Hex(buf)));
}

pub fn encode_bytes(src: buffer::Buffer) {
//return hex2ByteArray(byteArray2Hex(src));
}

pub fn decode_bytes(src: buffer:Buffer) {
//return :hex2ByteArray(:byteArray2String(src));
}

pub fn encode_big_int(bn) {
//return tolower(add_prefix(bn::toBase(16)));
}

pub fn decode_big_int(hex_string: String) {
//return BN(strip_prefix(hex_str), 'hex');
}
*/

pub fn strip_prefix(str: String) -> String {
    let re = regex::Regex::new(r"^0x").unwrap();
    return re.replace_all(&str, "").to_string();
}

pub fn add_prefix(str: String) -> String {
    return format!("0x{}", strip_prefix(str));
}
