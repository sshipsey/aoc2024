use utils::input::read_lines;

fn main() {
    let rows = read_lines("day04/input.txt").expect("Failed to parse input");
    let matrix = rows
        .into_iter()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    println!("{}", part1(&matrix));
    println!("{}", part2(&matrix));
}

fn part1(matrix: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            let count = vec![
                get_left(&matrix, x, y),
                get_up(&matrix, x, y),
                get_right(&matrix, x, y),
                get_down(&matrix, x, y),
                get_up_left(&matrix, x, y),
                get_up_right(&matrix, x, y),
                get_right_down(&matrix, x, y),
                get_left_down(&matrix, x, y),
            ]
            .iter()
            .map(|is_xmas| if *is_xmas { 1 } else { 0 })
            .sum::<i32>();
            sum += count;
        }
    }
    sum
}

fn part2(matrix: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    sum
}

fn get_left(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x < 3 {
        return false;
    }
    check_xmas(vec![
        matrix[y][x],
        matrix[y][x - 1],
        matrix[y][x - 2],
        matrix[y][x - 3],
    ])
}

fn get_up(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if y < 3 {
        return false;
    }
    check_xmas(vec![
        matrix[y][x],
        matrix[y - 1][x],
        matrix[y - 2][x],
        matrix[y - 3][x],
    ])
}

fn get_right(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x > matrix[0].len() - 4 {
        return false;
    }
    check_xmas(vec![
        matrix[y][x],
        matrix[y][x + 1],
        matrix[y][x + 2],
        matrix[y][x + 3],
    ])
}

fn get_down(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if y > matrix.len() - 4 {
        return false;
    }
    check_xmas(vec![
        matrix[y][x],
        matrix[y + 1][x],
        matrix[y + 2][x],
        matrix[y + 3][x],
    ])
}

fn get_up_left(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x < 3 || y < 3 {
        return false;
    }
    check_xmas(vec![
        matrix[y][x],
        matrix[y - 1][x - 1],
        matrix[y - 2][x - 2],
        matrix[y - 3][x - 3],
    ])
}

fn get_up_right(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x > matrix[0].len() - 4 || y < 3 {
        return false;
    }
    check_xmas(vec![
        matrix[y][x],
        matrix[y - 1][x + 1],
        matrix[y - 2][x + 2],
        matrix[y - 3][x + 3],
    ])
}

fn get_right_down(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x > matrix[0].len() - 4 || y > matrix.len() - 4 {
        return false;
    }
    check_xmas(vec![
        matrix[y][x],
        matrix[y + 1][x + 1],
        matrix[y + 2][x + 2],
        matrix[y + 3][x + 3],
    ])
}

fn get_left_down(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x < 3 || y > matrix.len() - 4 {
        return false;
    }
    check_xmas(vec![
        matrix[y][x],
        matrix[y + 1][x - 1],
        matrix[y + 2][x - 2],
        matrix[y + 3][x - 3],
    ])
}

fn check_xmas(chars: Vec<char>) -> bool {
    chars.into_iter().collect::<String>() == "XMAS"
}
