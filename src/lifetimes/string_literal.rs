struct MyStruct {}
impl MyStruct {
    //default lifetime of string literal is static
    fn get_string_literal() -> &str {
        "evren"
    }
    fn get_string_literal_lifetime() -> &'static str {
        "okcu"
    }
}
