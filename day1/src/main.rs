use std::collections::HashMap;

use aoc_utils::fetch_puzzle_input;

fn main() {
    let puzzle_input = fetch_puzzle_input(1).unwrap();

    let left_list: Vec<i64> = puzzle_input
        .lines()
        .map(|line| line.split_whitespace().nth(0).unwrap().parse().unwrap())
        .collect();

    let right_list: Vec<i64> = puzzle_input
        .lines()
        .map(|line| line.split_whitespace().nth(1).unwrap().parse().unwrap())
        .collect();

    println!(
        "part1={}",
        solve_part1(left_list.clone(), right_list.clone())
    );
    println!("part2={}", solve_part2(left_list, right_list));
}

fn solve_part1(mut left_list: Vec<i64>, mut right_list: Vec<i64>) -> String {
    left_list.sort();
    right_list.sort();

    let total: i64 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();

    total.to_string()
}

fn solve_part2(left_list: Vec<i64>, right_list: Vec<i64>) -> String {
    let mut occurences = HashMap::new();
    for n in left_list.iter() {
        if !occurences.contains_key(n) {
            let nb_occurences = right_list.iter().filter(|&x| *x == *n).count();
            occurences.insert(n, nb_occurences);
        }
    }

    let similarity_score: i64 = left_list
        .iter()
        .map(|left| {
            let multiply = occurences.get(left).unwrap_or(&0);
            *left * (*multiply) as i64
        })
        .sum();
    similarity_score.to_string()
}
