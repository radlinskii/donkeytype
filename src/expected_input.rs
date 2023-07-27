use crate::config::Config;

#[derive(Debug)]
pub struct ExpectedInput<'a>(&'a str);

impl<'a> ExpectedInput<'a> {
    pub fn new(_config: &Config) -> ExpectedInput<'a> {
        let expected_input_str = "hello world";

        return ExpectedInput(expected_input_str);
    }

    pub fn to_str(&self) -> &str {
        return self.0;
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

        assert_eq!(expected_input.to_str(), "hello world");
    }
}
