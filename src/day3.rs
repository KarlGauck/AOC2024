use std::fs;
use regex::Regex;

fn read_input() -> String{
    fs::read_to_string("inputs/day3.txt").unwrap()
}

/*
pub fn part1() {
    let string = read_input();

    // index ranges, describing where the inputs of mul(a,b) start
    let mut num1_first = 0;
    let mut num1_second = 0;
    let mut num2_first = 0;
    let mut num2_second = 0;

    // describes where we are in the string "mul(a,b)" while reading in the next pattern
    let mut pattern_state = 0;
    let mut sum = 0;

    for char_index in 0..string.len() {
        let c = string.chars().collect::<Vec<char>>()[char_index];
        pattern_state = match pattern_state {
            0 | 8 => if c == 'm' {1} else {0}
            1 => if c == 'u' {pattern_state + 1} else { if c == 'm' {1} else {0} }
            2 => if c == 'l' {pattern_state + 1} else { if c == 'm' {1} else {0} }
            3 => if c == '(' {pattern_state + 1} else { if c == 'm' {pattern_state + 1} else {0} }
            4 => {
                let mut res = if c == 'm' {1} else {0};
                if ('0'..'9').contains(&c) {
                    num1_first = char_index;
                    res = pattern_state + 1
                }
                res
            }
            5 => {
                let mut res = if c == 'm' {1} else {0};
                if c == ',' {
                    num1_second = char_index;
                    res = pattern_state + 1;
                }
                else if ('0'..'9').contains(&c) {
                    res = pattern_state
                }
                res
            }
            6 => {
                let mut res = if c == 'm' {1} else {0};
                if ('0'..'9').contains(&c) {
                    num2_first = char_index;
                    res = pattern_state + 1
                }
                res
            }
            7 => {
                let mut res = if c == 'm' {1} else {0};
                if c == ')' {
                    num2_second = char_index;
                    res = pattern_state + 1;
                }
                else if ('0'..'9').contains(&c) {
                    res = pattern_state
                }
                res
            }
            _default => 0
        };

        if pattern_state == 8 {
            println!("{}", string[num1_first-4..num2_second+1].to_string());
            let int1 = string[num1_first..num1_second].parse::<i32>().unwrap();
            let int2 = string[num2_first..num2_second].parse::<i32>().unwrap();
            sum += int1 * int2;
        }
    }

    println!("{}", sum);
}
*/

pub fn part1()
{
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let mut sum = 0;
    for cap in re.captures_iter(&read_input()) {
        let cap = cap.get(0);
        match cap {
            Some(a) => {
                let str = a.as_str().replace("mul(", "").replace(")", "");
                let str = str.split(",").collect::<Vec<&str>>();
                sum += str[0].parse::<i32>().unwrap() * str[1].parse::<i32>().unwrap();
            }
            None => {}
        }
    }
    println!("Sum: {}", sum);
}