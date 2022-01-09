//! # rustdoc-example
//!
//! A simple project demonstrating the use of rustdoc with the function [`mult`].
//!
//! # Example
//!
//! ----
//! use rustdoc_example::mult;
//! assert_eq!(mult(10, 10), 100);
//! ----

#![warn(missing_docs)]

/// Returns the product of `a` and `b`.
pub fn mult(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 * 2, mult(2, 2));
    }
}
