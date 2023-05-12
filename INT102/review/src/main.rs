mod brute_force;

fn main() {
    let unsorted_arr = [7, 5, 3, 2, 6, 8, 9, 4, 2];

    let arr_for_selection_sort = &mut unsorted_arr.clone();
    brute_force::selection_sort(arr_for_selection_sort);
    println!("After Selection Sort: {:?}", arr_for_selection_sort);

    let arr_for_bubble_sort = &mut unsorted_arr.clone();
    brute_force::bubble_sort(arr_for_bubble_sort);
    println!("After Bubble Sort: {:?}", arr_for_bubble_sort);

    let arr_for_insertion_sort = &mut unsorted_arr.clone();
    brute_force::insertion_sort(arr_for_insertion_sort);
    println!("After Insertion Sort: {:?}", arr_for_insertion_sort);
}
