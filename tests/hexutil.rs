#[cfg(test)]
mod tests {
    extern crate c3;

    use self::c3::hexutil;
    use std::string::String;

    #[test]
    fn it_works() {
        assert_eq!(hexutil::encode_string(String::from("hello")), "0x68656c6c6f");
        assert_eq!(hexutil::encode_string(String::from("123")), "0x313233");
        assert_eq!(hexutil::decode_string(String::from("0x313233")), "123".to_owned().into_bytes());
        assert_eq!(hexutil::encode_to_string("hello".to_owned().into_bytes()), "0x68656c6c6f");
        assert_eq!(hexutil::decode_to_string("68656c6c6f".to_owned().into_bytes()), "hello");
        assert_eq!(hexutil::encode_bytes("hello".to_owned().into_bytes()), "68656c6c6f".to_owned().into_bytes());
        assert_eq!(hexutil::decode_bytes("68656c6c6f".to_owned().into_bytes()), "hello".to_owned().into_bytes());
        assert_eq!(hexutil::strip_prefix(String::from("0x123")), "123");
        assert_eq!(hexutil::strip_prefix(String::from("123")), "123");
        assert_eq!(hexutil::strip_prefix(String::from("0x")), "");
        assert_eq!(hexutil::add_prefix(String::from("123")), "0x123");
        assert_eq!(hexutil::add_prefix(String::from("0x123")), "0x123");
    }
}
