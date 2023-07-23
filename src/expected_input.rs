use crate::config;

#[derive(Debug)]
pub struct ExpectedInput<'a>(&'a str); // tuple struct

impl<'a> ExpectedInput<'a> {
    pub fn new(_config: config::Config) -> ExpectedInput<'a> {
        let expected_input_str = "hello world";

        return ExpectedInput(expected_input_str);
    }

    pub fn to_str(&self) -> &str {
        return self.0;
    }
}
