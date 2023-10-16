use anyhow::{Context, Result};

use crate::runner::Stats;

pub fn split_by_char_index(string: &str, char_index: usize) -> (&str, &str) {
    string
        .char_indices()
        .nth(char_index)
        .map(|(index, _)| (&string[..index], &string[index..]))
        .unwrap_or((string, ""))
}
// create function which takes input of state struct and duration and create csv with headers and
// state fields
pub fn dump_data_to_csv(state: Stats, duration: u64) -> Result<String> {
    let csv_path = dirs::home_dir()
        .context("Unable to get home directory")?
        .join(".config")
        .join("donkeytype")
        .join("results.csv");
    // create csv path if it doesn't exist
    if !csv_path.exists() {
        std::fs::create_dir_all(csv_path.parent().unwrap())
            .context("failed to create csv file to dump data.")?;
    }
    let csv_file = String::from(csv_path.to_str().unwrap());

    let mut writer = csv::Writer::from_path(csv_path)?;
    writer.write_record(&[
        "WPM",
        "RAW_ACCURACY",
        "RAW_VALID_CHARACTERS_COUNT",
        "RAW_MISTAKES",
        "RAW_TYPED_CHARACTERS",
        "ACCURACY_AFTER_CORRECTIONS",
        "VALID_CHARACTERS_AFTER_CORRECTIONS",
        "MISTAKES_AFTER_CORRECTIONS",
        "TYPED_CHARACTERS_AFTER_CORRECTIONS",
        "DURATION_IN_SECONDS",
    ])?;
    writer
        .write_record(&[
            state.wpm.to_string(),
            state.raw_accuracy.to_string(),
            state.raw_valid_characters_count.to_string(),
            state.raw_mistakes_count.to_string(),
            state.raw_typed_characters_count.to_string(),
            state.accuracy.to_string(),
            state.valid_characters_count.to_string(),
            state.mistakes_count.to_string(),
            state.typed_characters_count.to_string(),
            duration.to_string(),
        ])
        .context("failed to write records")?;
    writer.flush().context("Unable to save data to csv file")?;
    Ok(csv_file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work_with_non_ascii_chars() {
        let (first_part, second_part) = split_by_char_index("Բարեւ Ձեզ", 5);

        assert_eq!((first_part, second_part), ("Բարեւ", " Ձեզ"));
    }
}
