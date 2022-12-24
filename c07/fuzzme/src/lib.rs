pub fn parse_integer(s: &str) -> Option<i32> {
    // checks if string contains _only_ digits using a regular expression,
    // including negative numbers
    use regex::Regex;
    // will match a string with 1-10 digits, prefixed by an option `-`
    let re = Regex::new(r"^-?\d{1,10}$").expect("parsing regex failed");
    if re.is_match(s) {
        Some(s.parse().expect("Parsing failed"))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_integer;

    #[test]
    fn test_parse_integer() {
        assert_eq!(parse_integer("10149").unwrap(), 10149);
    }
}
