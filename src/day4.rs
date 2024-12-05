use std::fs;

fn read_input() -> Vec<Vec<char>>
{
    fs::read_to_string("inputs/day4_part2_test.txt").unwrap().split("\n").map(|l| l.chars().collect()).collect()
}

pub fn part1() {
    let mut sum = 0;

    let mut diag1 = [
        ['X', ' ', ' ', ' '],
        [' ', 'M', ' ', ' '],
        [' ', ' ', 'A', ' '],
        [' ', ' ', ' ', 'S'],
    ];

    let mut diag2 = [
        ['S', ' ', ' ', ' '],
        [' ', 'A', ' ', ' '],
        [' ', ' ', 'M', ' '],
        [' ', ' ', ' ', 'X'],
    ];
    let diag3 = diag1.clone();
    let diag4 = diag2.clone();
    diag1.reverse();
    diag2.reverse();

    let diag_patterns = [diag1, diag2, diag3, diag4];
    let hor_pattern = ['X', 'M', 'A', 'S'];
    let mut hor_rev = hor_pattern.clone();
    hor_rev.reverse();
    let hor_patterns = [hor_pattern, hor_rev];

    let data = read_input();

    for x in 0..data.len()-3 {
        for y in 0..data[x].len()-3 {
            'pattern: for pattern in &diag_patterns {
                for dx in 0..4 {
                    for dy in 0..4 {
                        if pattern[dx][dy] != ' ' && pattern[dx][dy] != data[x+dx][y+dy] {
                            continue 'pattern;
                        }
                    }
                }
                sum += 1;
            }
        }
    }

    for x in 0..data.len() {
        for y in 0..data[x].len()-3 {
            'pattern: for pattern in &hor_patterns {
                for i in 0..4 {
                    if pattern[i] != data[x][y+i] {
                        continue 'pattern;
                    }
                }
                sum += 1;
            }
        }
    }

    for x in 0..data.len()-3 {
        for y in 0..data[x].len() {
            'pattern: for pattern in &hor_patterns {
                for i in 0..4 {
                    if pattern[i] != data[x+i][y] {
                        continue 'pattern;
                    }
                }
                sum += 1;
            }
        }
    }

    println!("sum: {}", sum);
}

pub fn part2() {
    let mut sum = 0;

    let mut pattern1 = [
        ['M', ' ', 'S'],
        [' ', 'A', ' '],
        ['M', ' ', 'S']
    ];

    let mut pattern2 = [
        ['S', ' ', 'S'],
        [' ', 'A', ' '],
        ['M', ' ', 'M']
    ];

    let mut pattern3 = pattern1.clone();
    let pattern4 = pattern2.clone();
    for mut i in 0..pattern1.len() {
        let a = pattern1[i];
    }
    for a in pattern3 {
        println!("{:?}", a);
    }
    for a in pattern1 {
        println!("{:?}", a);
    }
    pattern2.reverse();

    let diag_patterns = [pattern1, pattern2, pattern3, pattern4];
    let data = read_input();

    for x in 0..data.len()-2 {
        for y in 0..data[x].len()-2 {
            let mut p_index = 0;
            'pattern: for pattern in &diag_patterns {
                p_index += 1;
                for dx in 0..3 {
                    for dy in 0..3 {
                        if pattern[dx][dy] != ' ' && pattern[dx][dy] != data[x+dx][y+dy] {
                            continue 'pattern;
                        }
                    }
                }
                println!("x: {}, y: {}, index: {}", x, y, p_index);
                sum += 1;
            }
        }
    }

    println!("sum: {}", sum);
}