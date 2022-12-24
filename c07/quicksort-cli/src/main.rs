use std::env;

fn main() {
    use quicksort_proptest::Quicksort;

    let mut values: Vec<i64> = env::args()
        .skip(1)
        .map(|s| s.parse::<i64>().expect(&format!("{s}: bad input: ")))
        .collect();

    values.quicksort();

    println!("{values:?}");
}
