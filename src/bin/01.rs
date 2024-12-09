use std::collections::HashMap;

advent_of_code::solution!(1);

// 3   4
// 4   3
// 2   5
// 1   3
// 3   9
// 3   3
pub fn parse_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    // Left: [3, 4, 2, 1, 3, 3]
    // Right: [4, 3, 5, 3, 9, 3]
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        left.push(parts[0].parse::<u32>().unwrap());
        right.push(parts[1].parse::<u32>().unwrap());
    }

    (left, right)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse_lists(input);

    left.sort();
    right.sort();

    let total_distance: u32 = left.iter().zip(right.iter())
        .map(|(&left_num, &right_num)| (right_num as i32 - left_num as i32).abs() as u32)
        .sum();

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut left, right) = parse_lists(input);

    let mut freq_map: HashMap<u32, u32> = HashMap::new();
    for num in right.iter() {
        *freq_map.entry(*num).or_insert(0) += 1;
    }

    left.sort_unstable();
    let mut total_similarity: u32 = 0;

    for &left_num in &left {
        if let Some(count) = freq_map.get(&left_num) {
            total_similarity += left_num * count;
        }
    }

    Some(total_similarity)
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
