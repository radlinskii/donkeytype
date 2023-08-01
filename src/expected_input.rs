use crate::config::Config;

use mockall::automock;

#[derive(Debug)]
pub struct ExpectedInput;

impl ExpectedInput {
    pub fn new(_config: &Config) -> Self {
        return Self {};
    }
}

#[automock]
pub trait ExpectedInputInterface {
    fn get_string(&self, len: usize) -> String;
}

impl ExpectedInputInterface for ExpectedInput {
    fn get_string(&self, len: usize) -> String {
        let s = String::from("hello world! I hope it works now. I don't know why it didn't work before, but it should work now.");
        let s = s.repeat(len / s.len() + 1);
        let (s, _) = s.split_at(len);

        return s.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_expected_input_should_correctly_convert_to_str() {
        let config = Config::default();
        let expected_input = ExpectedInput::new(&config);

        assert_eq!(expected_input.get_string(12), "hello world!");
    }
}
