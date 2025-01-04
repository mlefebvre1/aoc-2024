use aoc_utils::fetch_puzzle_input;

fn main() {
    // let puzzle_input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
    let puzzle_input = fetch_puzzle_input(2).unwrap();

    println!("part1={}", part1(&puzzle_input));
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

            let deltas = calculate_deltas(&report);
            same_signs(&deltas) && adjacent_levels(&deltas)
        })
        .count()
        .to_string()
}

fn calculate_deltas(report: &[i16]) -> Vec<i16> {
    let mut deltas = vec![0; report.len() - 1];
    for i in 0..report.len() - 1 {
        let n1 = report[i];
        let n2 = report[i + 1];
        deltas[i] = n2 - n1;
    }
    deltas
}

fn same_signs(deltas: &[i16]) -> bool {
    let is_positive = deltas[0].is_positive();
    deltas.iter().all(|n| n.is_positive() == is_positive)
}

fn adjacent_levels(deltas: &[i16]) -> bool {
    deltas.iter().all(|n| n.abs() >= 1 && n.abs() <= 3)
}
