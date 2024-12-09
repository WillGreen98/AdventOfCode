advent_of_code::solution!(4);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    // This function maps each direction to the corresponding row and column changes
    fn to_delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::UpLeft => (-1, -1),
            Direction::UpRight => (-1, 1),
            Direction::DownLeft => (1, -1),
            Direction::DownRight => (1, 1),
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
}

pub fn check_target_direction(grid: &[Vec<char>], target: &str, start_row: usize, start_col: usize, direction: &Direction) -> bool {

}

pub fn find_word(grid: &[Vec<char>], word: &str) -> u32 {

}

pub fn check_x_mas(grid: &[Vec<char>], row: usize, col: usize) -> bool {

}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = parse_input(input); // Parse the grid directly
    let result: u32 = find_word(&grid, "MAS"); // Pass the grid to find_word

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = parse_input(input); // Parse the grid
    let result: u32 = find_x_mas(&grid); // Count the X-MAS patterns

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
