use std::fs;

fn read_input() -> (Vec<i32>, Vec<i32>)
{
    let content = fs::read_to_string("inputs/day1.txt");
    let content = content.unwrap_or("0".to_string());
    let content = content.split('\n').collect::<Vec<_>>();
    let list1 = content.iter().map(|s| s.split("   ").collect::<Vec<&str>>()[0]).collect::<Vec<&str>>().iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let list2 = content.iter().map(|s| s.split("   ").collect::<Vec<&str>>()[1]).collect::<Vec<&str>>().iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    (list1, list2)
}

pub fn part1() {
    let (mut list1, mut list2) = read_input();
    list1.sort();
    list2.sort();
    let mut sum = 0;
    for i in 0..list1.len() {
        sum += (list1[i] - list2[i]).abs();
    }
    println!("{}", sum)
}

pub fn part2() {
    let (list1,list2) = read_input();
    let overlap = list1.iter().filter(|e| list2.contains(*e)).collect::<Vec<_>>();
    for e in &overlap {
        println!("{}", e);
    }
    let mut sum = 0;
    for elem in overlap {
        let val: i32 = list2.iter().filter(|e| *e == elem).collect::<Vec<_>>().len() as i32;
        println!("{}", val);
        sum += val * elem;
    }

    println!("{}", sum);
}