pub fn selection_sort<T: Ord>(arr: &mut [T]) -> &mut [T] {
    for i in 0..arr.len() {
        if let Some((min_i, _)) = arr.iter().enumerate().skip(i).min_by_key(|(_, v)| *v) {
            arr.swap(i, min_i)
        }
    }
    arr
}

pub fn bubble_sort<T: Ord>(arr: &mut [T]) -> &mut [T] {
    for i in 0..arr.len() {
        for j in 1..arr.len() - i {
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
            }
        }
    }
    arr
}

pub fn insertion_sort<T: Ord>(arr: &mut [T]) -> &mut [T] {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
    arr
}
