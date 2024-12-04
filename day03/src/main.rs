use utils::input::read_to_string;
fn main() {
    let memory = read_to_string("day03/input.txt").expect("Failed to parse input file");

    println!("Part 1: {}", part1(memory.clone()));
    println!("Part 2: {}", part2(memory.clone()));
}

fn part1(memory: String) -> i32 {
    let substring = "mul(";

    let chars: Vec<_> = memory.chars().collect();
    let mut idx = 0;
    let mut parsing_first_num = false;
    let mut parsing_second_num = false;
    let mut first_num = String::from("");
    let mut second_num = String::from("");
    let mut sum = 0;

    while idx < memory.len() {
        if parsing_first_num {
            if chars[idx].is_numeric() {
                first_num.push(chars[idx]);
            }

            if chars[idx] == ',' {
                parsing_first_num = false;
                parsing_second_num = true;
                idx += 1;
            }

            if first_num.len() > 3 || (!chars[idx].is_numeric() && chars[idx] != ',') {
                first_num = String::from("");
                parsing_first_num = false;
            }
        }

        if parsing_second_num {
            if chars[idx].is_numeric() {
                second_num.push(chars[idx]);
            }

            if (!chars[idx].is_numeric() && !(chars[idx] == ')')) || second_num.len() > 3 {
                first_num = String::from("");
                second_num = String::from("");
                parsing_second_num = false;
            }

            if chars[idx] == ')' && first_num.len() > 0 && second_num.len() > 0 {
                parsing_second_num = false;
                sum = sum
                    + (first_num.parse::<i32>().expect("failed to parse")
                        * second_num.parse::<i32>().expect("failed to parse"));
                first_num = String::from("");
                second_num = String::from("");
            }
        }

        if !parsing_first_num
            && !parsing_second_num
            && idx < memory.len() - 4
            && memory[idx..].starts_with(substring)
        {
            parsing_first_num = true;
            idx += 3;
        }
        idx += 1;
    }
    sum
}

fn part2(memory: String) -> i32 {
    let substring = "mul(";

    let chars: Vec<_> = memory.chars().collect();
    let mut idx = 0;
    let mut parsing_first_num = false;
    let mut parsing_second_num = false;
    let mut first_num = String::from("");
    let mut second_num = String::from("");
    let mut sum = 0;
    let mut is_enabled = true;

    while idx < memory.len() {
        if parsing_first_num {
            if chars[idx].is_numeric() {
                first_num.push(chars[idx]);
            }

            if chars[idx] == ',' {
                parsing_first_num = false;
                parsing_second_num = true;
                idx += 1;
            }

            if first_num.len() > 3 || (!chars[idx].is_numeric() && chars[idx] != ',') {
                first_num = String::from("");
                parsing_first_num = false;
            }
        }

        if parsing_second_num {
            if chars[idx].is_numeric() {
                second_num.push(chars[idx]);
            }

            if (!chars[idx].is_numeric() && !(chars[idx] == ')')) || second_num.len() > 3 {
                first_num = String::from("");
                second_num = String::from("");
                parsing_second_num = false;
            }

            if chars[idx] == ')' && first_num.len() > 0 && second_num.len() > 0 {
                parsing_second_num = false;
                sum = sum
                    + (first_num.parse::<i32>().expect("failed to parse")
                        * second_num.parse::<i32>().expect("failed to parse"));
                first_num = String::from("");
                second_num = String::from("");
            }
        }

        if !parsing_first_num
            && !parsing_second_num
            && idx < memory.len() - 4
            && memory[idx..].starts_with(substring)
            && is_enabled
        {
            parsing_first_num = true;
            idx += 3;
        }

        if idx < memory.len() - 7 && memory[idx..].starts_with("don't()") && is_enabled {
            is_enabled = false;
            idx += 6;
        }

        if idx < memory.len() - 4 && memory[idx..].starts_with("do()") && !is_enabled {
            is_enabled = true;
            idx += 3;
        }

        idx += 1;
    }
    sum
}
