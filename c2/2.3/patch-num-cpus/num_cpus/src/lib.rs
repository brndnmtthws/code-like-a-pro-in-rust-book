pub fn get() -> usize {
    100 // Return some arbitrary value, for test purposes
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
