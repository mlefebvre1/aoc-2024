use aoc_utils::fetch_puzzle_input;
use regex::Regex;

fn main() -> anyhow::Result<()> {
    let puzzle_input = fetch_puzzle_input(3)?;
    println!("part1={}", part1(&puzzle_input)?);
    println!("part2={}", part2(&puzzle_input)?);

    Ok(())
}

fn part1(puzzle_input: &str) -> anyhow::Result<String> {
    let re = Regex::new(r"mul\(\d+,\d+\)")?;
    Ok(re
        .find_iter(puzzle_input)
        .map(|m| str_to_mul(m.as_str()))
        .sum::<u64>()
        .to_string())
}

fn part2(puzzle_input: &str) -> anyhow::Result<String> {
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)")?;
    let mut values = vec![];
    let mut ignore = false;
    for m in re.find_iter(puzzle_input).map(|s| s.as_str()) {
        match m {
            "don't()" => ignore = true,
            "do()" => ignore = false,
            s => {
                if !ignore {
                    values.push(str_to_mul(s));
                }
            }
        }
    }
    Ok(values.into_iter().sum::<u64>().to_string())
}

fn str_to_mul(s: &str) -> u64 {
    let s = s.replace("mul(", "").replace(")", "");
    let mut ss = s.split(",");
    let a = ss.next().unwrap().parse::<u64>().unwrap();
    let b: u64 = ss.next().unwrap().parse().unwrap();
    a * b
}
