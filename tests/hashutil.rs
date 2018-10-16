#[cfg(test)]
mod tests {
    extern crate c3;
    use self::c3::hashutil;
    use std::string::String;

    #[test]
    fn it_works() {
        assert_eq!(hashutil::encode_string(String::from("hello")), "0x68656c6c6f");
        assert_eq!(hashutil::strip_prefix(String::from("0x123")), "123");
        assert_eq!(hashutil::strip_prefix(String::from("123")), "123");
        assert_eq!(hashutil::strip_prefix(String::from("0x")), "");
        assert_eq!(hashutil::add_prefix(String::from("123")), "0x123");
        assert_eq!(hashutil::add_prefix(String::from("0x123")), "0x123");
    }
}
