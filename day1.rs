use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn read_file(path: &str) -> String {
    let mut f = File::open(path).expect("File not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("Something went wrong reading the file");

    return contents;
}

fn parse_int(input: &str) -> i32 {
    return input.parse::<i32>().unwrap();
}

fn count_frequency(adjustments: Vec<&str>) -> i32 {
    return adjustments
        .into_iter()
        .map(|a| parse_int(a))
        .sum::<i32>();
}

fn part_1(input: &String) -> i32 {
    return count_frequency(input.lines().collect());
}

fn part_2(input: &String) -> i32 {
    let mut seen: HashSet<i32> = HashSet::new();
    let mut step_generator = input.lines().collect::<Vec<&str>>().into_iter().cycle();
    let mut frequency = 0;

    while seen.insert(frequency) {
        let adjustment = step_generator.next().unwrap();
        frequency = frequency + parse_int(adjustment);
    }

    frequency
}

fn main() {
    let contents = read_file("./inputs/day1_input.txt");

    let answer_1 = part_1(&contents);
    let answer_2 = part_2(&contents);

    println!("Frequency: {}", answer_1);
    println!("First duplicate frequency: {}", answer_2);
}
