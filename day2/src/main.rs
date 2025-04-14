use aoc_utils::fetch_puzzle_input;

fn main() -> anyhow::Result<()> {
    let puzzle_input = fetch_puzzle_input(2)?;

    println!("part1={}", part1(&puzzle_input));
    println!("part2={}", part2(&puzzle_input));

    Ok(())
}

fn part1(puzzle_input: &str) -> String {
    puzzle_input
        .split("\n")
        .filter(|line| {
            if line.is_empty() {
                return false;
            }
            let report: Vec<i16> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            is_all_increasing(&report) || is_all_decreasing(&report)
        })
        .count()
        .to_string()
}

fn part2(puzzle_input: &str) -> String {
    puzzle_input
        .split("\n")
        .filter(|line| {
            if line.is_empty() {
                return false;
            }
            let report: Vec<i16> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            create_reports_with_skip(&report)
                .iter()
                .any(|r| is_all_increasing(r) || is_all_decreasing(r))
        })
        .count()
        .to_string()
}

fn create_reports_with_skip(report: &[i16]) -> Vec<Vec<i16>> {
    let mut reports = vec![];
    for i in 0..report.len() {
        let mut report: Vec<i16> = report.to_vec();
        report.remove(i);
        reports.push(report);
    }
    reports
}

fn is_all_increasing(report: &[i16]) -> bool {
    is_all_inner(report, |a, b| b - a)
}

fn is_all_decreasing(report: &[i16]) -> bool {
    is_all_inner(report, |a, b| a - b)
}

fn is_all_inner(report: &[i16], diff_fn: fn(i16, i16) -> i16) -> bool {
    for i in 1..report.len() {
        let diff = diff_fn(report[i - 1], report[i]);
        if !(1..=3).contains(&diff) {
            return false;
        }
    }
    true
}
