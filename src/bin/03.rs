use regex::Regex;

advent_of_code::solution!(3);

#[derive(Debug)]
pub enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

// Part 1 and 2 - valid instruction extractor
pub fn extract_instructions(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(?P<dont>don't\()|(?P<do>do\()").unwrap();
    let mut instructions = Vec::new();

    for cap in re.captures_iter(input) {
        if let Some(x) = cap.get(1) {
            if let Some(y) = cap.get(2) {
                if let (Ok(x_num), Ok(y_num)) = (x.as_str().parse(), y.as_str().parse()) {
                    instructions.push(Instruction::Mul(x_num, y_num));
                }
            }
        } else if let Some(_) = cap.name("dont") {
            println!("Found 'don't' instruction");
            instructions.push(Instruction::Dont);
        } else if let Some(_) = cap.name("do") {
            println!("Found 'do' instruction");
            instructions.push(Instruction::Do);
        }
    }

    println!("Extracted {:#?} instructions", &instructions);
    instructions
}


/// Part 1 multiply X and Y and return the sum of their product.
pub fn compute_sum_of_products(mul_instructions: &[(u32, u32)]) -> u32 {
    mul_instructions.iter().map(|(x, y)| x * y).sum()
}

pub fn compute_sum_with_conditions(instructions: &[Instruction]) -> u32 {
    let mut enabled = true;
    let mut sum = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Mul(x, y) if enabled => {
                let product = x * y;
                println!("Multiplying {} * {} = {}", x, y, product);
                sum += product;
            }
            Instruction::Do => {
                enabled = true;
                println!("Enabled multiplication");
            }
            Instruction::Dont => {
                enabled = false;
                println!("Disabled multiplication");
            }
            _ => {}
        }
    }

    println!("Final sum: {}", sum);
    sum
}
pub fn part_one(input: &str) -> Option<u32> {
       let instructions = extract_instructions(input);
       let mul_instructions: Vec<(u32, u32)> = instructions
           .into_iter()
           .filter_map(|instruction| {
               if let Instruction::Mul(x, y) = instruction {
                   Some((x, y))
               } else {
                   None
               }
           })
           .collect();

       Some(compute_sum_of_products(&mul_instructions))
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions = extract_instructions(input);

    // Pass all instructions (Mul, Do, and Dont) to compute_sum_with_conditions
    Some(compute_sum_with_conditions(&instructions))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
