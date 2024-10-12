
use std::collections::HashMap;

fn mode(numbers: &Vec<u32>) -> Option<u32> {
    let mut mode_map: HashMap<u32, u32> = HashMap::new();

    // Count occurrences of each number
    for &v in numbers.iter() {
        let count = mode_map.entry(v).or_insert(0);
        *count += 1;
    }

    // Variables to track the maximum frequency and corresponding key (mode)
    let mut max_value = 0;
    let mut max_key = None;

    // Find the key with the maximum value
    for (&key, &value) in mode_map.iter() {
        if value > max_value {
            max_value = value;
            max_key = Some(key);  // Store the key of the maximum value
        }
    }

    max_key  // Return the mode, or None if the vector is empty
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 2, 3, 4, 4, 4];
    if let Some(mode_of_numbers) = mode(&numbers) {
        println!("The mode is: {}", mode_of_numbers);
    } else {
        println!("No mode found");
    }
}