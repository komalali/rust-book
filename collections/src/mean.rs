use std::collections::HashMap;

pub fn calculate_mean(int_list: &[i32]) -> f32 {
    let len = int_list.len() as f32;
    let sum = int_list.iter().sum::<i32>() as f32;
    sum / len
}

pub fn calculate_median(int_list: &mut [i32]) -> f32 {
    int_list.sort_unstable();
    let length = int_list.len();
    let median = if length % 2 == 0 {
        let midpoint_round_down = length /2;
        ((int_list[midpoint_round_down] + int_list[midpoint_round_down + 1]) / 2) as f32
    } else {
        int_list[length / 2] as f32
    };
    median
}

pub fn calculate_mode(int_list: &[i32]) -> i32 {
    let mut number_occurrences = HashMap::new();
    for &item in int_list {
        let count = number_occurrences.entry(item).or_insert(0);
        *count += 1;
    }

    let mut mode= 0;
    let mut highest_occurrence = 0;
    for (key, val) in number_occurrences.iter() {
        if &highest_occurrence < val {
            mode = *key;
            highest_occurrence = *val;
        }
    }

    mode
}
