use magnus::{function, prelude::*, Error, Ruby};

fn hello(subject: String) -> String {
    format!("Hello from Rust, {subject}!")
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("SampleGemRust")?;
    module.define_singleton_method("hello", function!(hello, 1))?;
    Ok(())
}

#[test]
fn test_hello() {
    assert_eq!(hello("world".to_string()), "Hello from Rust, world!");
}
