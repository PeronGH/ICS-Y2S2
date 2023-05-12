pub fn selection_sort<T: Ord>(arr: &mut [T]) -> &mut [T] {
    for i in 0..arr.len() {
        if let Some((min_i, _)) = arr.iter().enumerate().skip(i).min_by_key(|(_, v)| *v) {
            arr.swap(i, min_i)
        }
    }
    arr
}

pub fn bubble_sort<T: Ord>(arr: &mut [T]) -> &mut [T] {
    for _ in 0..arr.len() {
        for i in 1..arr.len() {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
            }
        }
    }
    arr
}
