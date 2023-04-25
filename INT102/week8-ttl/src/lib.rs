pub fn counting_sort(arr: &mut [usize]) {
    // Step 1: Find the maximum value in the input array
    let max_val = match arr.iter().max() {
        Some(&max) => max,
        None => return,
    };

    // Step 2: Create count array
    let mut count_arr = vec![0; max_val + 1];

    // Step 3: Count occurrences of each number
    for &num in arr.iter() {
        count_arr[num] += 1;
    }

    // Step 4: Compute cumulative sum
    for i in 1..count_arr.len() {
        count_arr[i] += count_arr[i - 1];
    }

    // Step 5: Create output array
    let mut output_arr = vec![0; arr.len()];

    // Step 6: Place numbers into output array
    for &num in arr.iter().rev() {
        output_arr[count_arr[num] - 1] = num;
        count_arr[num] -= 1;
    }

    // Step 7: Copy sorted elements back to input array
    arr.clone_from_slice(&output_arr);
}

pub mod horspool {
    use std::collections::HashMap;

    // Step 1: Create shift table
    pub fn create_shift_table(pattern: &str) -> HashMap<char, usize> {
        let m = pattern.len();
        pattern
            .chars()
            .enumerate()
            .take(m - 1)
            .map(|(i, ch)| (ch, m - i - 1)) // Step 1.1: Compute the shift for each character in the pattern
            .collect() // Step 1.2: Collect shifts into a HashMap
    }

    pub fn search(text: &str, pattern: &str) -> Option<usize> {
        let shift_table = create_shift_table(pattern);
        let (n, m) = (text.len(), pattern.len());

        let mut i = 0;
        while i <= n - m {
            // Step 2: Compare pattern and text from the current position
            if pattern
                .chars()
                .zip(text[i..].chars())
                .all(|(p_ch, t_ch)| p_ch == t_ch)
            {
                // Step 3: If all characters match, return the position
                return Some(i);
            }

            // Step 4: If there's a mismatch, shift the pattern using the shift table
            let next_char = text
                .chars()
                .nth(i + m - 1)
                .unwrap_or_else(|| pattern.chars().next().unwrap());
            i += shift_table.get(&next_char).unwrap_or(&m);
        }

        // Step 5: If the pattern goes beyond the end of the text, return None (no match found)
        None
    }
}
