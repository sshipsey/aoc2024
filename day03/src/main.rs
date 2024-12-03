use utils::input::read_to_string;
fn main() {
    let memory = read_to_string("day03/input.txt").expect("Failed to parse input file");
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
            if first_num.len() > 3 {
                first_num = String::from("");
                parsing_first_num = false;
            }
            if chars[idx] == ',' {
                parsing_first_num = false;
                parsing_second_num = true;
                idx += 1;
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
                println!("{} * {}", first_num, second_num);
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
    println!("{}", sum);
}

// 168787836 low
