use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn read_file(path: &str) -> String {
    let mut f = File::open(path).expect("File not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("Something went wrong reading the file");

    return contents;
}

fn part_1(input: &String) -> i32 {

    let box_ids = input.lines().collect::<Vec<&str>>();
    let mut twos = 0;
    let mut threes = 0;

    for box_id in box_ids.into_iter() {
        let mut counts = HashMap::new();

        for letter in box_id.chars() {
            *counts.entry(letter).or_insert(0) += 1;
        }

        let mut values = counts.values();
        let mut two_add = 0;
        let mut three_add = 0;

        for &val in values {
            if two_add != 0 && three_add != 0 {
                break;
            }

            if val == 2 {
                two_add = 1;
                continue;
            }

            if val == 3 {
                three_add = 1;
            }
        }

        twos += two_add;
        threes += three_add;
    }

    return twos * threes;
}

fn remove_diff(s1: &str, s2: &str) -> String {
    let mut commons = String::from("");
    let s1_parts: Vec<char> = s1.chars().collect();
    let s2_parts: Vec<char> = s2.chars().collect();

    for i in 0..s1.len() {
        if s1_parts[i] == s2_parts[i] {
            commons.push(s1_parts[i]);
        }
    }

    return commons;
}

fn part_2(input: &String) -> String {
    let box_ids = input.lines().collect::<Vec<&str>>();
    let dupe = box_ids.clone();

    for box1 in &box_ids {
        for dup in &dupe {
            if box1 != dup && hamming(&box1, &dup) == 1 {
                return remove_diff(&box1, &dup);
            }
        }
    }

    return String::from("Test");
}

fn hamming(s1: &str, s2: &str) -> i32 {
    if s1.len() != s2.len() {
        return 0;
    }

    let mut diff = 0;
    let s1_parts: Vec<char> = s1.chars().collect();
    let s2_parts: Vec<char> = s2.chars().collect();

    for (a,b) in s1_parts.iter().zip(s2_parts.iter()) {
        if a != b {
            diff += 1;
        }
    }

    return diff;
}

fn main() {
    let contents = read_file("./inputs/day2_input.txt");

    let answer_1 = part_1(&contents);
    let answer_2 = part_2(&contents);
    let ham_test = hamming("abcde", "axcye");

    println!("Answer #1: {}", answer_1);
    println!("Hamming test: {}", ham_test);
    println!("Answer #2: {}", answer_2);
}
