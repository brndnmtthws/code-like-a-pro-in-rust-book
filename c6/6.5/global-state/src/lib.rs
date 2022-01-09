#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use std::sync::atomic::{AtomicI32, Ordering};
    lazy_static! {
        static ref COUNT: AtomicI32 = AtomicI32::new(0);
    }

    #[test]
    fn test_count() {
        COUNT.fetch_add(1, Ordering::SeqCst);
    }
}
