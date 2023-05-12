pub trait BruteForceSorting<T: Ord> {
    fn selection_sort(&mut self) -> &mut [T];
    fn bubble_sort(&mut self) -> &mut [T];
    fn insertion_sort(&mut self) -> &mut [T];
}

impl<T: Ord> BruteForceSorting<T> for [T] {
    fn selection_sort(&mut self) -> &mut [T] {
        selection_sort(self);
        self
    }

    fn bubble_sort(&mut self) -> &mut [T] {
        bubble_sort(self);
        self
    }

    fn insertion_sort(&mut self) -> &mut [T] {
        insertion_sort(self);
        self
    }
}

pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        if let Some((min_i, _)) = arr.iter().enumerate().skip(i).min_by_key(|(_, e)| *e) {
            arr.swap(i, min_i)
        }
    }
}

pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 1..arr.len() - i {
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
            }
        }
    }
}

pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

pub fn linear_search<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {
    arr.iter().position(|e| e == target)
}
