mod brute_force;
use brute_force::BruteForceSorting;

fn main() {
    let unsorted_arr = [7, 5, 3, 2, 6, 8, 9, 4, 2];

    println!(
        "After Selection Sort: {:?}",
        unsorted_arr.clone().selection_sort()
    );

    println!(
        "After Bubble Sort: {:?}",
        unsorted_arr.clone().bubble_sort()
    );

    println!(
        "After Insertion Sort: {:?}",
        unsorted_arr.clone().insertion_sort()
    );
}
