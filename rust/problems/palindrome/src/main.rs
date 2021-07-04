fn main() {
    println!("Hello, world!");
}

// TODO: Make the `is_palindrome` function work for non english words too.
fn is_palindrome(text: String) -> bool {
    let half = text.len() / 2;

    text.bytes().take(half).eq(text.bytes().rev().take(half))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_test() {
        assert!(is_palindrome("kayak".to_string()));
        assert!(is_palindrome("radar".to_string()));

        assert!(!is_palindrome("not a palindrome".to_string()));
    }
}
