use std::cmp::PartialEq;
use std::fs;

fn read_input() -> Vec<(i64, Vec<i64>)>
{
    let data = fs::read_to_string("inputs/day7.txt").unwrap();
    let data = data.split("\n")
        .map(|s| s.split(": ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let mut final_data: Vec<(i64, Vec<i64>)> = vec![];
    for date in &data {
        let mut tuple: (i64, Vec<i64>) = (0, vec![]);
        tuple.0 = date[0].parse::<i64>().unwrap();
        tuple.1 = date[1].split(" ").map(|s| s.parse::<i64>().unwrap()).collect();
        final_data.push(tuple);
    }
    final_data
}

pub fn binary_add(num: Vec<bool>) -> Vec<bool> {
    let mut result = vec![];
    let mut flipped = false;
    for dig in num {
        result.push(if flipped {dig} else {!dig});
        if !dig {
            flipped = true;
        }
    }
    result
}

pub fn part1() {
    let data = read_input();

    let mut sum: i64 = 0;

    for (key, mut values) in data {
        let num_ops = values.len() - 1;
        let mut ops = vec![false; num_ops];
        let first = values[0];
        values.remove(0);

        loop {
            let mut acc = first;
            for i in 0..num_ops {
                acc = if ops[i] { acc + values[i] } else { acc * values[i] };
            }
            if acc == key {
                sum += key;
                break;
            }
            if !ops.contains(&false) {
                break;
            }
            ops = binary_add(ops);
        }
    }
    println!("day 7.1 {}", sum);
}

#[derive(PartialEq, Eq, Clone)]
enum Troolian {
    TRUE = 0,
    FALSE = 1,
    TRALSE = 2
}

pub fn teneary_add(num: Vec<Troolian>) -> Vec<Troolian> {
    let mut result = vec![];
    let mut incremented = false;
    for dig in num {
        result.push(if incremented { dig.clone() } else {
            match &dig {
                Troolian::TRUE => Troolian::FALSE,
                Troolian::FALSE => Troolian::TRALSE,
                Troolian::TRALSE => Troolian::TRUE,
            }
        });
        if dig != Troolian::TRALSE {
            incremented = true;
        }
    }
    result
}

pub fn part2() {
    let data = read_input();

    let mut sum: i64 = 0;

    for (key, mut values) in data {
        let num_ops = values.len() - 1;
        let mut ops = vec![Troolian::TRUE; num_ops];
        let first = values[0];
        values.remove(0);

        loop {
            let mut acc = first;
            for i in 0..num_ops {
                acc = match ops[i] {
                    Troolian::TRUE => acc + values[i],
                    Troolian::FALSE => acc * values[i],
                    Troolian::TRALSE => {
                        let mut str = acc.to_string();
                        str.push_str(values[i].to_string().as_str());
                        str.parse::<i64>().unwrap()
                    },
                }
            }
            if acc == key {
                sum += key;
                break;
            }
            if !ops.contains(&Troolian::TRUE) && !ops.contains(&Troolian::FALSE) {
                break;
            }
            ops = teneary_add(ops);
        }
    }
    println!("day 7.1 {}", sum);
}