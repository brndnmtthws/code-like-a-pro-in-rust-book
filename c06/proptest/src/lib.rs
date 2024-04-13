pub fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    proptest! {
        #[test]
        #[should_panic(expected = "attempt to add with overflow")]
        fn test_add(a: i64, b: i64) {
            assert_eq!(add(a, b), a+b);
        }
    }
}
