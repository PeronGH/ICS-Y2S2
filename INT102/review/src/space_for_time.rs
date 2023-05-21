pub fn counting_sort(input_arr: &[usize]) -> Vec<usize> {
    let max_val = match input_arr.iter().max() {
        Some(&max) => max,
        None => return vec![],
    };

    let mut count_arr = vec![0; max_val + 1];

    // Create a frequency array
    for &num in input_arr {
        count_arr[num] += 1;
    }

    // Convert to cumulative sum
    for i in 1..count_arr.len() {
        count_arr[i] += count_arr[i - 1];
    }

    let mut output_arr = vec![0; input_arr.len()];

    for &num in input_arr.iter().rev() {
        output_arr[count_arr[num] - 1] = num;
        count_arr[num] -= 1;
    }

    output_arr
}
