pub fn alternating_chars(string: &str) -> String {
    let mut i: i64 = 0;
    let mut alts = String::from("");
    for c in string.chars() {
        if i == 0 {
            alts.push(c);
        }

        i = (i + 1) % 2;
    }

    alts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alternating_chars_returns_alternate_characters() {
        let alternates = alternating_chars("abcde");
        assert_eq!(alternates, "ace");

        let alternates = alternating_chars("bcdef");
        assert_eq!(alternates, "bdf");
    }

    #[test]
    fn alternating_chars_works_with_empty_string() {
        assert_eq!(alternating_chars(""), "");
    }
}
