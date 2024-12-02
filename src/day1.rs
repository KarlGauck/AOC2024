use std::fs;

pub fn read_input(){
    let content = fs::read_to_string("inputs/day1.txt");
    let content = content.unwrap_or("0".to_string());
    let content = content.split('\n').collect::<Vec<_>>();
    let mut list1 = content.iter().map(|s| s.split("   ").collect::<Vec<&str>>()[0]).collect::<Vec<&str>>().iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut list2 = content.iter().map(|s| s.split("   ").collect::<Vec<&str>>()[1]).collect::<Vec<&str>>().iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    list1.sort();
    list2.sort();
    let mut sum = 0;
    for i in 0..list1.len() {
        sum += (list1[i] - list2[i]).abs();
    }
    println!("{}", sum)
}