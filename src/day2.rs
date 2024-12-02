fn read_input() -> Vec<Vec<i32>> {
    let data = std::fs::read_to_string("./inputs/day2.txt").unwrap_or("0".to_string());
    let data = data.split("\n").collect::<Vec<&str>>();
    let mut int_lines: Vec<Vec<i32>> = vec![];
    let mut lineindex = 0;
    for line in data {
        let str_seq = line.split(" ").collect::<Vec<&str>>();
        let mut int_seq: Vec<i32> = vec![];
        for str in str_seq {
            let int = str.parse::<i32>();
            int_seq.push(int.unwrap());
        }
        if int_seq.first() > int_seq.last() {
            int_seq.reverse();
        }
        int_lines.push(int_seq);
        lineindex += 1;
    }
    for line in &mut int_lines {
        if line.last() < line.first() {
            line.reverse()
        }
    }
    int_lines
}

pub fn part1() {
    let mut data = read_input();
    let mut count = 0;
    'lines:  for list in data {
        for i in 1..list.len() {
            if ![1, 2, 3].contains(&(list[i] - list[i - 1])) {
                continue 'lines;
            }
        }
        count += 1;
    }
    println!("{}", count);
}

pub fn get_permutations(list: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut permutations: Vec<Vec<i32>> = vec![];
    for i in 0..list.len() {
        let mut list_cpy = list.clone();
        list_cpy.remove(i);
        permutations.push(list_cpy);
    }
    permutations
}
pub fn part2() {
    let mut data = read_input();
    let mut count = 0;
    'lines:  for list in data {
        let permutations = get_permutations(&list);
        'perm: for permutation in permutations {
            for i in 1..permutation.len() {
                if ![1, 2, 3].contains(&(permutation[i] - permutation[i - 1])) {
                    continue 'perm;
                }
            }
            count += 1;
            break 'perm;
        }
    }
    println!("{}", count);
}