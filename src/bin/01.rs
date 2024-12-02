advent_of_code::solution!(1);

use std::collections::HashMap;

// input is a file containing two space-separated lists of numbers on each line
pub fn part_one(input: &str) -> Option<u32> {
    // iterate over the input lines
    let lines = input.lines();
    let mut sum = 0;
    let mut left_numbers: Vec<u32> = Vec::new();
    let mut right_numbers: Vec<u32> = Vec::new();

    for line in lines {
        // split each line into two numbers
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        left_numbers.push(numbers[0]);
        right_numbers.push(numbers[1]);
    }

    left_numbers.sort();
    right_numbers.sort();

    // sum the distances
    for i in 0..left_numbers.len() {
        let difference = right_numbers[i].abs_diff(left_numbers[i]);
        sum += difference;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut similarity_score = 0;
    let mut left_numbers: Vec<u32> = Vec::new();
    let mut right_numbers: Vec<u32> = Vec::new();
    let mut right_counts: HashMap<u32, u32> = HashMap::new();

    for line in lines {
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        left_numbers.push(numbers[0]);
        right_numbers.push(numbers[1]);
        *right_counts.entry(numbers[1]).or_insert(0) += 1;
    }

    for left_number in left_numbers {
        let count = right_counts.get(&left_number).unwrap_or(&0);
        similarity_score += left_number * count;
    }

    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
