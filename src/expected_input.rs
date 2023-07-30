use crate::config::Config;

use mockall::automock;

#[derive(Debug)]
pub struct ExpectedInput;

impl ExpectedInput {
    pub fn new(_config: &Config) -> ExpectedInput {
        return ExpectedInput {};
    }
}

#[automock]
pub trait ExpectedInputTrait {
    fn get_string(&self, len: usize) -> String;
}

impl ExpectedInputTrait for ExpectedInput {
    fn get_string(&self, len: usize) -> String {
        let s = String::from("hello world I hope it works now, because I've spent some time trying to fix it and it should work now. I don't know why it didn't work before, but it should work now.");
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
        let config = Config {
            duration: 30,
            numbers: false,
        };
        let expected_input = ExpectedInput::new(&config);

        assert_eq!(expected_input.get_string(12), "hello world ");
    }
}
