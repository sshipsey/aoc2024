use utils::input;
use utils::iterable::count;
use utils::matrix::transpose;

fn main() {
    let input_file = "day01/input.txt";

    let lines = input::read_lines(input_file).expect("Failed to read input file");
    let pairs: Vec<Vec<&str>> = lines
        .iter()
        .map(|line| line.split("   ").collect())
        .collect();

    let number_pairs_transposed: Vec<Vec<i32>> = transpose(pairs)
        .into_iter()
        .map(|pair| {
            pair.into_iter()
                .map(|value| value.parse::<i32>().expect("failed to parse value"))
                .collect()
        })
        .map(|mut row: Vec<i32>| {
            row.sort();
            row
        })
        .collect();

    println!("{}", part1(number_pairs_transposed.clone()));
    println!("{}", part2(number_pairs_transposed.clone()));
}

fn part1(number_pairs_transposed: Vec<Vec<i32>>) -> i32 {
    let number_pairs_zipped: Vec<_> = number_pairs_transposed[0]
        .iter()
        .zip(number_pairs_transposed[1].iter())
        .collect();

    number_pairs_zipped
        .iter()
        .map(|nums| (nums.0 - nums.1).abs())
        .sum::<i32>()
}

fn part2(pairs: Vec<Vec<i32>>) -> i32 {
    let counts_map = count(pairs[1].iter());
    pairs[0].iter().fold(0, |acc, val| {
        acc + (val * *counts_map.get(val).unwrap_or(&0) as i32)
    })
}
