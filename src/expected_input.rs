use crate::config;

#[derive(Debug)]
pub struct ExpectedInput<'a>(&'a str);

impl<'a> ExpectedInput<'a> {
    pub fn new(_config: config::Config) -> ExpectedInput<'a> {
        let expected_input_str = "hello world";

        return ExpectedInput(expected_input_str);
    }

    pub fn to_str(&self) -> &str {
        return self.0;
    }
}

#[cfg(test)]
mod tests {
    use crate::config;
    use crate::expected_input;

    #[test]
    fn to_str() {
        let expected_input = expected_input::ExpectedInput::new(config::Config {
            duration: 30,
            numbers: false,
        });

        assert_eq!(expected_input.to_str(), "hello world");
    }
}
