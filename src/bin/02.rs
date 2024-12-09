advent_of_code::solution!(2);

/*
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
*/
pub fn parse_lists(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line: &str| {
            line.split_whitespace()
                .filter_map(|s: &str| s.parse::<i32>().ok())
                .collect()
        })
        .collect()
}

pub fn is_safe_report(level: &[i32]) -> bool {
    let mut increasing: bool = true;
    let mut decreasing: bool = true;

    for i in 1..level.len() {
        match (level[i] - level[i - 1]).cmp(&0) {
            std::cmp::Ordering::Greater => increasing = false,
            std::cmp::Ordering::Less => decreasing = false,
            _ => {}
        }
    }

    if !increasing && !decreasing {
        return false;
    }

    for i in 1..level.len() {
        let diff = (level[i] - level[i - 1]).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }

    true
}

// This approach came from reddit
// but reimplemented
fn is_dampener_safe(level: Vec<i32>) -> bool {
    for i in 0..level.len() {
        let mut level_copy = level.clone();
        level_copy.remove(i);

        if is_safe_report(&level_copy) {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let levels: Vec<Vec<i32>> = parse_lists(input);
    let safe_count: usize = levels.iter().filter(|level: &&Vec<i32>| is_safe_report(level)).count();

    Some(safe_count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let levels: Vec<Vec<i32>> = parse_lists(input);
    let safe_count: usize = levels.iter().filter(|level: &&Vec<i32>| is_dampener_safe(level.to_vec())).count();

    Some(safe_count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
