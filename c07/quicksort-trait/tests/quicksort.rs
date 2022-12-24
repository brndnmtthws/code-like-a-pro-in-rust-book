#[test]
fn test_quicksort() {
    use quicksort_trait::quicksort;

    let mut values = vec![12, 1, 5, 0, 6, 2];
    quicksort(&mut values);
    assert_eq!(values, vec![0, 1, 2, 5, 6, 12]);

    let mut values = vec![1, 13, 5, 10, 6, 2, 0];
    quicksort(&mut values);
    assert_eq!(values, vec![0, 1, 2, 5, 6, 10, 13]);
}

#[test]
fn test_quicksort_trait() {
    use quicksort_trait::Quicksort;

    let mut values = vec![12, 1, 5, 0, 6, 2];
    values.quicksort();
    assert_eq!(values, vec![0, 1, 2, 5, 6, 12]);

    let mut values = vec![1, 13, 5, 10, 6, 2, 0];
    values.quicksort();
    assert_eq!(values, vec![0, 1, 2, 5, 6, 10, 13]);
}
