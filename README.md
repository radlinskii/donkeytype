# Donkeytype

a _very_ minimalistic _cli_ typing test.

![donkeytype demonstration](https://github.com/radlinskii/donkeytype/assets/26116041/ecd835f5-e50b-4bc6-aea4-75f9ecde5de7)

## How it works

When run the program you will see the expected input displayed at the top of your terminal window.
This text is a placeholder, and this is the input that you should write when the test is started.

On the bottom-right corner there is a help message saying that to start the test you need to press `'e'` (enter the test) or leave by pressing `'q'`
When test is running you can see how much time you have left in bottom-left corner.

You can pause the test by pressinng <Esc>, to resume it press `'e'` again.

WPM (words per minute) score is calculated as amount of typed characters divided by 5 (word), divided by the duration normalized to 60 seconds (minute).

## Usage

### Installation

For now there is no deployment environment setup.
You can clone the repo, and run the main program with cargo:

```shell
cargo run
```

To start the program with default config.

> So far it was only tested on MacOS.

### Configuring

For now there are only three options that are read from config.
Configuration will grow when more features are added (_different modes_, _different languages_, _configuring colors_).

Default config looks like this:

| name              | default value          | type in JSON | description                                                          |
| ----------------- | ---------------------- | ------------ | -------------------------------------------------------------------- |
| `duration`        | `30`                   | number       | duration of the test in seconds                                      |
| `numbers`         | `false`                | boolean      | flag indicating if numbers should be inserted in expected input      |
| `numbers_ratio`   | `0.05` if numbers=TRUE | number       | ratio for putting numbers in the test                                |
| `dictionary_path` | `"src/dict/words.txt"` | string       | dictionary words to sample from while creating test's expected input |
 
`NOTE: If provided numbers_ratio is not between 0 to 1.0, Default numbers_ratio = 0.05 will be used.`



You can provide this config as options when running the program like so:

```shell
cargo run -- --duration 60 --dictionary-path "/usr/share/dict/words" --numbers true
```

or put them in a config file in `~/.config/donkeytype/donkeytype-config.json`:

```json
{
    "duration": 60,
    "dictionary_path": "/usr/share/dict/words",
    "numbers": false
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

Thank you for considering contributing to the project.

### Suggesting a Feature or Enhancement

If you have an idea for a new feature or enhancement, please share it. Follow these steps to suggest a feature:

1. Check if your feature idea has already been proposed in the issue tracker.
2. If it's not already there, open a new issue and describe the feature you'd like to see, why it's needed, and how you envision it working.

### Codebase contribution

To submit a contribution, follow these general steps:

1. Create your own fork of the code repository.
2. Make the desired changes in your fork.
3. Ennsure your changes adhere to the code style and quality standards.

### Hacktoberfest 2023

If you found this repo because of [Hacktoberfest 2023](https://hacktoberfest.com/), make sure you familiarize yourself with [participation rules for contributors](https://hacktoberfest.com/participation/#contributors).

## License

MIT.
See [LICENSE](./LICENSE)
