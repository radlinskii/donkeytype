use crate::config::Config;

#[derive(Debug)]
pub struct ExpectedInput {}

impl ExpectedInput {
    pub fn new(_config: &Config) -> ExpectedInput {
        return ExpectedInput {};
    }

    pub fn to_string(&self) -> String {
        return String::from("hello world ");
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

        assert_eq!(expected_input.to_string(), "hello world ");
    }
}
