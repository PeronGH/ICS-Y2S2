use std::cmp::Ordering::*;

pub fn merge_sort<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
    match arr.len() {
        0 | 1 => arr.to_vec(),
        _ => {
            let (left, right) = arr.split_at(arr.len() / 2);
            merge(&merge_sort(left), &merge_sort(right))
        }
    }
}

fn merge<T: Ord + Clone>(arr1: &[T], arr2: &[T]) -> Vec<T> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            merged.push(arr1[i].clone());
            i += 1;
        } else {
            merged.push(arr2[j].clone());
            j += 1;
        }
    }

    merged.extend_from_slice(&arr1[i..]);
    merged.extend_from_slice(&arr2[j..]);

    merged
}

pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> bool {
    if arr.is_empty() {
        return false;
    }
    let mid = arr.len() / 2;
    match arr[mid].cmp(target) {
        Equal => true,
        Less => binary_search(&arr[mid + 1..], target),
        Greater => binary_search(&arr[..mid], target),
    }
}
