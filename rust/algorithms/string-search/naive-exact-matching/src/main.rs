fn main() {}

fn matches(text: String, pattern: String) -> bool {
    for i in 0..(text.len() - pattern.len() + 1) {
        let mut matched = true;
        for j in 0..pattern.len() {
            if text.as_bytes()[i + j] != pattern.as_bytes()[j] {
                matched = false;
                break;
            }
        }

        if matched {
            return true;
        }
    }

    return false;
}

fn get_occurrences(text: String, pattern: String) -> Vec<usize> {
    let mut occurrences = Vec::new();

    for i in 0..(text.len() - pattern.len() + 1) {
        let mut matched = true;

        for j in 0..pattern.len() {
            if text.as_bytes()[i + j] != pattern.as_bytes()[j] {
                matched = false;
                break;
            }
        }

        if matched {
            occurrences.push(i);
        }
    }
    occurrences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_test() {
        assert!(matches(
            "A very long string with a word".to_string(),
            "word".to_string()
        ));
        assert!(!matches(
            "A very long string with a".to_string(),
            "word".to_string()
        ));
    }

    #[test]
    fn get_occurrences_test() {
        let expected = vec![26, 32];
        let result = get_occurrences(
            "A very long string with a wordy word".to_string(),
            "word".to_string(),
        );
        assert_eq!(result, expected);

        let expected = vec![];
        let result = get_occurrences("A very long string with a ".to_string(), "word".to_string());
        assert_eq!(result, expected);
    }
}
