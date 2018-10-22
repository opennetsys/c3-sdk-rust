#[cfg(test)]
mod tests {
    extern crate c3_sdk;

    use self::c3_sdk::hashutil;

    #[test]
    fn it_works() {
        assert_eq!(hashutil::generate_hash().len(), 64);
        assert_eq!(hashutil::hash_bytes("hello world".to_owned().into_bytes()), "0ac561fac838104e3f2e4ad107b4bee3e938bf15f2b15f009ccccd61a913f017".to_owned().into_bytes());
        assert_eq!(hashutil::hash_to_hex_string("hello world".to_owned().into_bytes()), "0x0ac561fac838104e3f2e4ad107b4bee3e938bf15f2b15f009ccccd61a913f017");
        assert_eq!(hashutil::is_equal("0x1234".to_string(), "foo".to_owned().into_bytes()), false);
        assert_eq!(hashutil::is_equal("0x0ac561fac838104e3f2e4ad107b4bee3e938bf15f2b15f009ccccd61a913f017".to_string(), "hello world".to_owned().into_bytes()), true);
    }
}
