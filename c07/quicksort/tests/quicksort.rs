use quicksort::quicksort;

#[test]
fn test_quicksort() {
    let mut values = vec![12, 1, 5, 0, 6, 2];
    quicksort(&mut values);
    assert_eq!(values, vec![0, 1, 2, 5, 6, 12]);

    let mut values = vec![1, 13, 5, 10, 6, 2, 0];
    quicksort(&mut values);
    assert_eq!(values, vec![0, 1, 2, 5, 6, 10, 13]);
}
