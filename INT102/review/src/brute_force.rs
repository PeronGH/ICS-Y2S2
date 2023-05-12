pub fn selection_sort<T: PartialOrd>(arr: &mut [T]) -> &mut [T] {
    // For each element in the list
    for i in 0..arr.len() {
        // Assume the first element is the smallest
        let mut min_index = i;
        // Check every element following the assumed smallest
        for j in (i + 1)..arr.len() {
            // If an element is smaller than the assumed smallest
            if arr[j] < arr[min_index] {
                // This is the new smallest element
                min_index = j;
            }
        }
        // Swap the smallest element with the first element
        arr.swap(i, min_index);
    }
    arr
}
