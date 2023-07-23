mod config;
mod expected_input;
mod runner;

fn main() {
    let config = config::Config::new();
    let expected_input = expected_input::ExpectedInput::new(config);
    let mut runner = runner::Runner::new(expected_input);

    runner.run();
}
