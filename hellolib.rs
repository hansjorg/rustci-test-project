#[link(name = "hellolib", vers = "0.1")];
#[crate_type = "lib"];

pub fn hello_string() -> &str {
    return "Hello world?";
}

#[test]
pub fn test_lib() {
    assert_eq!("Hello world?", hello_string());
}
