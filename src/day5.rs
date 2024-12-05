use std::fs;
use itertools::Itertools;

fn read_input() -> (Vec<(i32, i32)>, Vec<Vec<i32>>){
    let data = fs::read_to_string("inputs/day5.txt").unwrap();
    let data = data.split("\n\n").collect::<Vec<&str>>();
    let rules = data[0].split("\n").map(|s| s.split("|").map(|s| s.parse::<i32>().unwrap_or(i32::MAX)).collect_tuple().unwrap_or_else(|| {(0,0)} )).collect::<Vec<(i32, i32)>>();
    let updates = data[1].split("\n").map(|s| s.split(",").map(|s| s.parse::<i32>().unwrap_or(i32::MAX)).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
    (rules, updates)
}

pub fn part1() {
    let (rules, updates) = read_input();
    let mut sum = 0;

    'updates: for update in updates {
        for rule in rules.iter() {
            if !update.contains(&rule.0) {
                continue;
            }
            let first_index = update.iter().position(|&r| r == rule.0).unwrap();
            let second_index = update.iter().position(|&r| r == rule.1).unwrap_or(usize::MAX);
            if first_index > second_index {
                continue 'updates;
            }
        }
        sum += update[(update.len()-1)/2];
    }
    println!("Day 5 Part 1: {}", sum);
}