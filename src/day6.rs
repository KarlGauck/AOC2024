use std::fs;
use itertools::Itertools;
use crate::vector;

fn read_input() -> Vec<Vec<char>>{
    let data = fs::read_to_string("inputs/day6.txt").unwrap()
        .split("\n")
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    data
}

pub enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl Dir {
    pub fn char(&self) -> char {
        match &self {
            Dir::UP => '^',
            Dir::DOWN => 'v',
            Dir::LEFT => '>',
            Dir::RIGHT => '<'
        }
    }

    pub fn rotate(&self) -> Dir {
        match &self {
            Dir::UP => Dir::RIGHT,
            Dir::RIGHT => Dir::DOWN,
            Dir::DOWN => Dir::LEFT,
            Dir::LEFT => Dir::UP
        }
    }

    pub fn step(&self) -> (i32, i32) {
        match &self {
            Dir::UP => (0, -1),
            Dir::RIGHT => (1, 0),
            Dir::DOWN => (0, 1),
            Dir::LEFT => (-1, 0)
        }
    }

    pub fn values() -> [Dir; 4] {
        [Dir::UP, Dir::RIGHT, Dir::DOWN, Dir::LEFT]
    }

    pub fn char_list() -> [char; 4] {
        [Dir::UP.char(), Dir::RIGHT.char(), Dir::DOWN.char(), Dir::LEFT.char()]
    }

}

pub trait DirConvertible {
    fn to_dir(&self) -> Option<Dir>;
}

impl DirConvertible for char {
    fn to_dir(&self) -> Option<Dir> {
        match &self {
            '^' => Some(Dir::UP),
            'v' => Some(Dir::DOWN),
            '<' => Some(Dir::LEFT),
            '>' => Some(Dir::RIGHT),
            _ => {None}
        }
    }
}

pub fn part1() {
    let data = read_input();
    let vertical_index = data.iter()
        .position(|s| s.iter().any(|c| Dir::char_list().contains(c)))
        .unwrap() as i32;
    let horizontal_index = data[vertical_index as usize].iter()
        .position(|c| Dir::char_list().contains(c))
        .unwrap() as i32;

    let mut visited = vec![vec![false; data[0].len()]; data.len()];

    let mut position = (horizontal_index, vertical_index);
    let mut direction = data[vertical_index as usize][horizontal_index as usize].to_dir().unwrap();

    loop {
        let (x, y) = &position;
        visited[*x as usize][*y as usize] = true;

        let new_pos = (x + direction.step().0, y + direction.step().1);
        let (new_x, new_y) = &new_pos;

        if !(0..(data.len() as i32)).contains(new_y) || !(0..(data[0].len() as i32)).contains(new_x) {
            break;
        }
        let c = data[*new_y as usize][*new_x as usize];
        println!("char: {}", c);
        if data[*new_y as usize][*new_x as usize] == '#' {
            direction = direction.rotate()
        } else {
            position = new_pos;
        }
    }

    let sum = visited.iter().map(
            |l| l.iter().map(|b| if *b {1} else {0})
                .collect::<Vec<i32>>()
        )
        .collect::<Vec<Vec<i32>>>()
        .iter().flatten()
        .sum::<i32>();
    println!("{}", sum);
}