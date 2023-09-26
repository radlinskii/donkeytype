# Donkeytype

a _very_ minimalistic _cli_ typing test.

### Todo

-   [ ] docs
-   [ ] add CONTRIBUTING.md
-   [ ] save results to csv
-   [ ] read results from csv and display them
-   [ ] deployment to homebrew?
-   [ ] count wpm only in words that were typed correctly
-   [ ] handle alt+backspace to delete last word from input
-   [ ] support uppercase letters at the beginning of words
-   [ ] support numbers
-   [ ] support symbols
-   [ ] support configuring color scheme
-   [ ] figure out a way to mock tui::widgets::Widget to test if expected text is rendered
-   [ ] handle unicode characters (e.g. `ł`, `ń`, `ü`, `ß`)
-   [ ] add support for more languages
-   [ ] add integration tests

## How it works

When run the program you will see the expected input displayed at the top of your terminal window.
This text is a placeholder, and this is the input that you should write when the test is started.

On the bottom-right corner there is a help message saying that to start the test you need to press `'e'` (enter the test) or leave by pressing `'q'`
When test is running you can see how much time you have left in bottom-left corner.

You can pause the test by pressinng <Esc>, to resume it press `'e'` again.

WPM (words per minute) is calculated as amount of typed characters divided by 5 (word), divided by the duration normalized to 60 seconds (minute).

## Usage

### Installation

For now there is now deployment environment setup.
You can clone the repo, and run the main program with cargo:

```shell
cargo run
```

To start the program with default config.

### Configuring

For now there are only two options that are read from config.
Configuration will grow when more features are added (different modes, different languages, configuring colors).

Default config looks like this:

| name            | default value        | type in JSON | description                                                          |
| --------------- | -------------------- | ------------ | -------------------------------------------------------------------- |
| duration        | 30                   | number       | duration of the test in seconds                                      |
| dictionary_path | "src/dict/words.txt" | string       | dictionary words to sample from while creating test's expected input |

You can provide this config as options when running the program like so:

```shell
cargo run -- --duration 60
```

or put them in a config file in `~/.config/donkeytype/donkeytype-config.json`:

```json
{
    "duration": 60
}
```

To get all the available options run

```shell
cargo run -- --help
```

## Development

### Prerequisites

You need to have [rust](https://www.rust-lang.org/) installed to run & develop this program locally.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md)

## License

MIT.
See [LICENSE](./LICENSE)
