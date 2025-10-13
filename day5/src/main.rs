use std::{num::ParseIntError, result::Result, str::FromStr};

use anyhow::anyhow;
use aoc_utils::fetch_puzzle_input;

fn main() -> anyhow::Result<()> {
    let puzzle_input = fetch_puzzle_input(5)?;
    let start = std::time::Instant::now();
    println!("part1={}", part1(&puzzle_input)?);
    println!("part2={}", part2(&puzzle_input)?);
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<String> {
    let (orders, updates) = parse(input)?;
    let ans: usize = updates
        .iter()
        .filter(|update| update.is_ordered(&orders))
        .map(|update| update.mid_page_number() as usize)
        .sum();
    Ok(ans.to_string())
}

fn part2(input: &str) -> anyhow::Result<String> {
    let (orders, updates) = parse(input)?;
    let ans: usize = updates
        .iter()
        .filter(|update| !update.is_ordered(&orders))
        .map(|update| update.fix_ordering(&orders).mid_page_number() as usize)
        .sum();
    Ok(ans.to_string())
}

fn parse(input: &str) -> anyhow::Result<(Vec<Order>, Vec<Update>)> {
    let mut lines = input.split('\n');
    let lines_ref = lines.by_ref();

    let orders: anyhow::Result<Vec<Order>> = lines_ref
        .take_while(|line| !line.is_empty())
        .map(Order::from_str)
        .collect();

    let updates: anyhow::Result<Vec<Update>> = lines_ref
        .take_while(|line| !line.is_empty())
        .map(Update::from_str)
        .collect();

    Ok((orders?, updates?))
}

#[derive(Debug)]
struct Order {
    left: u8,
    right: u8,
}
impl Order {
    fn left_contains(&self, n: u8) -> bool {
        self.left == n
    }
}
impl FromStr for Order {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ss = s.split('|');

        let left = ss
            .next()
            .map(|n| n.parse::<u8>())
            .ok_or(anyhow!("Failed to get Order left number"))?;

        let right = ss
            .next()
            .map(|n| n.parse::<u8>())
            .ok_or(anyhow!("Failed to get Order right number"))?;

        Ok(Order {
            left: left?,
            right: right?,
        })
    }
}

#[derive(Debug, Clone)]
struct Update(Vec<u8>);
impl Update {
    fn is_ordered(&self, orders: &[Order]) -> bool {
        for i in 1..self.0.len() {
            let num = self.0[i];
            // find all order that contains the number to the left
            let lefts = orders
                .iter()
                .filter(|&order| order.left_contains(num))
                .collect::<Vec<_>>();

            // Check that each numbers to the left of that number is in correct order:
            // -> That they don't appear in any ordering to the right of the current number
            for j in 0..i {
                let left_num = self.0[j];
                if lefts.iter().any(|left| left.right == left_num) {
                    return false;
                }
            }
        }
        true
    }

    fn fix_ordering(&self, orders: &[Order]) -> Update {
        let mut update_out = self.clone();
        for i in 1..update_out.0.len() {
            let num = update_out.0[i];
            // find all order that contains the number to the left
            let lefts = orders
                .iter()
                .filter(|&order| order.left_contains(num))
                .collect::<Vec<_>>();

            // Check that each numbers to the left of that number is in correct order:
            // -> That they don't appear in any ordering to the right of the current number
            for j in 0..i {
                let left_num = update_out.0[j];
                if lefts.iter().any(|left| left.right == left_num) {
                    update_out.0.swap(i, j);
                }
            }
        }
        update_out
    }

    fn mid_page_number(&self) -> u8 {
        let i = self.0.len().div_ceil(2) - 1;
        self.0[i]
    }
}
impl FromStr for Update {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pages: Result<Vec<u8>, ParseIntError> = s.split(',').map(|page| page.parse()).collect();
        Ok(Update(pages?))
    }
}
