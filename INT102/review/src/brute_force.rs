pub fn selection_sort<T: Ord>(arr: &mut [T]) -> &mut [T] {
    for i in 0..arr.len() {
        if let Some((min_i, ..)) = arr.iter().enumerate().skip(i).min_by_key(|(.., v)| *v) {
            arr.swap(i, min_i);
        }
    }
    arr
}
