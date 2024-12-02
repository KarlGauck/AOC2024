mod day1;
mod day2;

fn main() {
    println!("{}", "61".parse::<i32>().unwrap());
    day2::part1();
    day2::part2();
    let permutations = day2::get_permutations(&vec![0, 1, 2, 3, 4]);
    for permutation in permutations {
        for i in permutation {
            print!("{}", i);
            print!(" ");
        }
        println!();
    }
}
