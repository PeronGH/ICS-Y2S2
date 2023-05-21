use std::collections::HashMap;

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
    // index -> element in input
    // value -> should be put in n-th place
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

// Step 1: Create shift table
fn create_shift_table(pattern: &str) -> HashMap<char, usize> {
    let m = pattern.len();
    pattern
        .chars()
        .enumerate()
        .map(|(i, ch)| (ch, m - i - 1)) // Step 1.1: Compute the shift for each character in the pattern
        .collect() // Step 1.2: Collect shifts into a HashMap
}

pub fn horspool_search(text: &str, pattern: &str) -> Option<usize> {
    let (n, m) = (text.len(), pattern.len());
    if n < m {
        return None;
    }

    // Step 2: Generate the shift table for the given pattern
    let shift_table = create_shift_table(pattern);

    // Step 3: Initialize skip and comparison count variables
    let mut skip = 0;

    // Step 4: Iterate through the text, comparing with the pattern
    while skip <= n - m {
        // Step 4.1: Check if the pattern starts at the current position in the text
        if text
            .chars()
            .skip(skip)
            .zip(pattern.chars())
            .all(|(a, b)| a == b)
        {
            // Step 4.1.1: If the pattern is found, return the position and comparison count
            return Some(skip);
        }

        // Step 4.2: Update the skip value based on the shift table
        skip += shift_table
            .get(&text.chars().skip(skip).nth(m - 1).unwrap())
            .unwrap_or(&m);
    }

    // Step 5: If the pattern is not found, return None and the comparison count
    None
}
