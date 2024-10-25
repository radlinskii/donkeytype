# Donkeytype

a _very_ minimalistic typing test TUI app.

![gif demonstrating how the program works](https://github.com/user-attachments/assets/8142340c-44db-4b79-926b-53332169daaf)

## How it Works

When you run the program, you'll find the expected input displayed at the top of your terminal window. This text serves as a placeholder and represents what you should type when the test begins. Your goal is to type this text as quickly as possible. If you make a mistake, you can press the `backspace` key to delete a single character or hold down `Option`/`Ctrl` and press `backspace` to delete an entire word.

In the top-right corner of the screen, a helpful message prompts you to start the test by pressing `'s'` (to start the test) or exit by pressing `'q'`.

While the test is running, you'll be able to monitor the time remaining in the top-left corner of the screen.

To pause the test, simply press `<Esc>`. To resume, press `'s'` again.

Your WPM (words per minute) score is calculated based on the number of typed characters divided by 5 (word), and normalized to a 60-second timeframe (minute).

> It has been successfully tested on `MacOS`, `Linux` and `Windows`

## Usage

### Installation

Go to [the latest release](https://github.com/radlinskii/donkeytype/releases/latest), download the compressed binary and unpack it locally.
Then to run the main program with default configuration simply run the executable binary in your terminal:

```shell
./donkeytype
```

You can move the binary to e.g. `~/.local/bin` folder (or any other folder added to your $PATH) to run it from anywhere:

```shell
mv ~/Downloads/donkeytype ~/.local/bin/donkeytype
donkeytype --version
```

By default `donkeytype` saves results of tests to `~/.local/share/donkeytype/donkeytype-results.csv` on **Linux** and **MacOS**, and `C:\Users\{Username}\AppData\Local\donkeytype\donkeytype-results.csv` on **Windows**.

To view the history of results in a bar chart you can run:

```shell
./donkeytype history
```

<img width="1426" alt="picture demonstrating bar chart with history data" src="https://github.com/user-attachments/assets/c96c4311-8ab7-4874-bf98-35648c541a0c">

To see all available options run:

```shell
./donkeytype --help
```

### Configuration

For now there are only three options that are read from config.
Configuration will grow when more features are added (_different modes_, _different languages_, _configuring colors_).

Default config looks like this:

| name              | default value               | type in JSON | description                                                                                                                                                                                                         |
| ----------------- | --------------------------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `duration`        | `30`                        | number       | duration of the test in seconds                                                                                                                                                                                     |
| `numbers`         | `false`                     | boolean      | flag indicating if numbers should be inserted in expected input                                                                                                                                                     |
| `numbers_ratio`   | `0.05` (if numbers=true)    | number       | ratio for putting numbers in the test                                                                                                                                                                               |
| `symbols`         | `false`                     | boolean      | flag indicating if symbols should be inserted in expected input                                                                                                                                                     |
| `symbols_ratio`   | `0.10` (if symbols=true)    | number       | ratio for putting symbols in the test                                                                                                                                                                               |
| `uppercase`       | `false`                     | boolean      | flag indicating if uppercase letters should be inserted in expected input                                                                                                                                           |
| `uppercase_ratio` | `0.15` (if uppercase=true)  | boolean      | ratio for putting uppercase letters in test                                                                                                                                                                         |
| `dictionary_path` | `None` (builtin dictionary) | string       | path to file with dictionary words to sample from while creating test's expected input                                                                                                                              |
| `save_results`    | `true`                      | boolean      | flag indicating if results should be saved to a file ( `~/.local/share/donkeytype/donkeytype-results.csv` on Linux and MacOS, and `C:\Users\{Username}\AppData\Local\donkeytype\donkeytype-results.csv` on Windows) |

NOTE: If provided `numbers_ratio` is not between `0` to `1.0`, default `numbers_ratio = 0.15` will be used. Same happens with `uppercase_ratio` and `symbols_ratio`.

You can provide this config as options when running the program like so:

```shell
./donkeytype --duration 60 --numbers true --uppercase true
```

To get all the available options run

```shell
./donkeytype --help
```

You can also put all the options inside config file in `~/.config/donkeytype/donkeytype-config.json`:

```json
{
    "duration": 60,
    "dictionary_path": "/usr/share/dict/words",
    "numbers": true,
    "numbers_ratio": 0.1,
    "uppercase": true,
    "uppercase_ratio": 0.3,
    "colors": {
        "correct_match_fg": "green",
        "correct_match_bg": "white",
        "incorrect_match_fg": "#ff00ff",
        "incorrect_match_bg": "#0f000f"
    }
}
```

> Providing config in a file also supports passing custom color values.

## Development

### Prerequisites

You need to have [rust toolchain](https://www.rust-lang.org/) installed locally to develop this program.

### Getting started

Use cargo to compile and run local repository with:

```
cargo run
```

To pass configuration options pass them via cargo to underlying program using `--`:

```
cargo run -- --duration 60
```

### Guidelines

Try cover your changes with unit tests whenever possible.
Before opening a PR run locally `rustfmt` to format your changes and make sure tests are passing with `cargo test`.

## Contributing

Thank you for considering contributing to the project.

### Suggesting a Feature or Enhancement

If you have an idea for a new feature or enhancement, please share it. Follow these steps to suggest a feature:

1. Check if your feature idea has already been proposed in the issue tracker.
2. If it's not already there, open a new issue and describe the feature you'd like to see, why it's needed, and how you envision it working.

### Codebase contribution

To submit a contribution, follow these general steps:

1. Create your own fork of the code repository.
2. Make the desired changes in your fork.
3. Ensure your changes adhere to the code style and quality standards.

### Hacktoberfest 2023

If you found this repo because of [Hacktoberfest 2023](https://hacktoberfest.com/), make sure you familiarize yourself with [participation rules for contributors](https://hacktoberfest.com/participation/#contributors).

## Uninstalling

If you want to remove `donkeytype` from your system you simply remove the executable binary from wherever you've downloaded it to.

### MacOS & Linux

Additionally to remove the history of results run:

```shell
rm -rf ~/.local/share/donkeytype
```

and if you've created a configuration file remove it too:

```shell
rm -rf ~/.config/donkeytype
```

### Windows

On Windows delete `C:\Users\{Username}\AppData\Local\donkeytype` folder to get rid of history results and configuration file.

## License

MIT.
See [LICENSE](./LICENSE)
