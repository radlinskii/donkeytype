pub fn split_by_char_index(string: &str, char_index: usize) -> (&str, &str) {
    string
        .char_indices()
        .nth(char_index)
        .map(|(index, _)| (&string[..index], &string[index..]))
        .unwrap_or((string, ""))
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
