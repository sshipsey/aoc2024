use utils::input::read_lines;

fn main() {
    let lines = read_lines("day02/input.txt").expect("Failed to parse input file");
    let rows: Vec<Vec<i32>> = lines
        .iter()
        .map(|row_str| {
            row_str
                .split(" ")
                .map(|num| num.parse::<i32>().expect("Failed to parse int"))
                .collect()
        })
        .collect();

    println!("{}", part1(&rows));
    println!("{}", part2(&rows));
}

fn part1(rows: &Vec<Vec<i32>>) -> i32 {
    rows.iter().map(|row| get_safety(row)).sum::<i32>()
}

fn part2(rows: &Vec<Vec<i32>>) -> i32 {
    rows.iter()
        .map(|row| {
            get_level_variations(row)
                .iter()
                .map(|variation| get_safety(variation))
                .sum::<i32>()
        })
        .map(|sum| if sum > 0 { 1 } else { 0 })
        .sum::<i32>()
}

fn get_safety(levels: &Vec<i32>) -> i32 {
    let is_ascending = levels[0] < levels[1];
    for i in 1..levels.len() {
        if (levels[i] - levels[i - 1]).abs() > 3
            || (is_ascending && levels[i] < levels[i - 1])
            || (!is_ascending && levels[i] > levels[i - 1])
            || levels[i] == levels[i - 1]
        {
            return 0;
        }
    }
    return 1;
}

fn get_level_variations(levels: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    for i in 0..levels.len() {
        let mut temp = levels.clone();
        temp.remove(i);
        result.push(temp);
    }

    result
}
