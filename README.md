# Donkeytype

a _very_ minimalistic _cli_ typing test.

![gif demonstraiting how the program works](https://github.com/radlinskii/donkeytype/assets/26116041/4c2a1b6d-e70e-4631-8438-9259cc780a36)

## How it works

When the program is run you will see the expected input displayed at the top of your terminal window.
This text is a placeholder, and this is the input that you should write when the test is started.
Now you should write this text as fast as you can.
If you make a mistake you can press `backspace` to delete a single character,
or press `backspace` while holding `Option`/`Ctrl` to delete a whole word.

On the bottom-right corner is a help message saying that to start the test you need to press `'e'` (enter the test) or leave by pressing `'q'`
When test is running you can see how much time you have left in bottom-left corner.

You can pause the test by pressing `<Esc>`, to resume it press `'e'` again.

WPM (words per minute) score is calculated as amount of typed characters divided by 5 (word), divided by the duration normalized to 60 seconds (minute).

## Usage

### Installation

For now there is no deployment environment setup.
You can clone the repo, and run the main program with default configuration using cargo:

```shell
cargo run
```

To view the history of results in a bar chart you can run:

```shell
cargo run -- history
```

<img width="1426" alt="picture demonstraiting bar chart with history data" src="https://github.com/radlinskii/donkeytype/assets/26116041/352c68fc-28a3-4ea2-8800-d74b8d759ddd">

To see all available options run:

```shell
cargo run -- --help
```

> So far it was only tested on MacOS.
> Needs testing on Linux
> Not supporting Windows yet (different file paths)

### Configuration

For now there are only three options that are read from config.
Configuration will grow when more features are added (_different modes_, _different languages_, _configuring colors_).

Default config looks like this:

| name              | default value               | type in JSON | description                                                                            |
| ----------------- | --------------------------- | ------------ | -------------------------------------------------------------------------------------- |
| `duration`        | `30`                        | number       | duration of the test in seconds                                                        |
| `numbers`         | `false`                     | boolean      | flag indicating if numbers should be inserted in expected input                        |
| `numbers_ratio`   | `0.05` (if numbers=TRUE)    | number       | ratio for putting numbers in the test                                                  |
| `uppercase`       | `false`                     | boolean      | flag indicating if uppercase letters should be inserted in expected input              |
| `uppercase_ratio` | `0.15`                      | boolean      | ratio for putting uppercase letters in test                                            |
| `dictionary_path` | `None` (builtin dictionary) | string       | path to file with dictionary words to sample from while creating test's expected input |
| `save_results`    | `true`                      | boolean      | flag indicating if results should be saved to a file                                   |
| `dictionary_path` | `"src/dict/words.txt"`      | string       | dictionary words to sample from while creating test's expected input                   |

> NOTE: If provided `numbers_ratio` is not between `0` to `1.0`, Default `numbers_ratio = 0.05` will be used.
> Same happens with `uppercase_ratio`.

You can provide this config as options when running the program like so:

```shell
cargo run -- --duration 60 --numbers true --uppercase true
```

To get all the available options run

```shell
cargo run -- --help
```

You can also put all the options inside config file in `~/.config/donkeytype/donkeytype-config.json`:

```json
{
    "duration": 60,
    "dictionary_path": "/usr/share/dict/words",
    "numbers": true,
    "numbers_ratio": 0.1,
    "uppercase": true,
    "uppercase_ratio": 0.3
    "colors": {
        "correct_match_fg": "green",
        "correct_match_bg": "white",
        "incorrect_match_fg": "#ff00ff"
        "incorrect_match_bg": "#0f000f"
    }
}
```

> Providing config in a file also supports passing custom color values.

## Development

### Prerequisites

You need to have [rust](https://www.rust-lang.org/) installed to run & develop this program locally.

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

## License

MIT.
See [LICENSE](./LICENSE)
