pub trait Quicksort {
    fn quicksort(&mut self) {}
}

impl<T: std::cmp::PartialOrd + Clone> Quicksort for [T] {
    fn quicksort(&mut self) {
        quicksort(self);
    }
}

pub fn quicksort<T: std::cmp::PartialOrd + Clone>(slice: &mut [T]) {
    if slice.len() < 2 {
        return;
    }
    let (left, right) = partition(slice);
    quicksort(left);
    quicksort(right);
}

fn partition<T: std::cmp::PartialOrd + Clone>(
    slice: &mut [T],
) -> (&mut [T], &mut [T]) {
    let pivot_value = slice[slice.len() - 1].clone();
    let mut pivot_index = 0;
    for i in 0..slice.len() {
        if slice[i] <= pivot_value {
            slice.swap(i, pivot_index);
            pivot_index += 1;
        }
    }
    if pivot_index < slice.len() - 1 {
        slice.swap(pivot_index, slice.len() - 1);
    }

    slice.split_at_mut(pivot_index - 1)
}

#[cfg(test)]
mod tests {
    use crate::{partition, quicksort};

    #[test]
    fn test_partition() {
        let mut values = vec![0, 1, 2, 3];
        assert_eq!(
            partition(&mut values),
            (vec![0, 1, 2].as_mut_slice(), vec![3].as_mut_slice())
        );

        let mut values = vec![0, 1, 2, 4, 3];
        assert_eq!(
            partition(&mut values),
            (vec![0, 1, 2].as_mut_slice(), vec![3, 4].as_mut_slice())
        );
    }

    #[test]
    fn test_quicksort() {
        let mut values = vec![1, 5, 0, 6, 2];
        quicksort(&mut values);
        assert_eq!(values, vec![0, 1, 2, 5, 6]);

        let mut values = vec![1, 5, 10, 6, 2, 0];
        quicksort(&mut values);
        assert_eq!(values, vec![0, 1, 2, 5, 6, 10]);
    }
}
