use anyhow::anyhow;
use aoc_utils::fetch_puzzle_input;

fn main() -> anyhow::Result<()> {
    let puzzle_input = fetch_puzzle_input(7)?;
    //     let puzzle_input = r#"190: 10 19
    // 3267: 81 40 27
    // 83: 17 5
    // 156: 15 6
    // 7290: 6 8 6 15
    // 161011: 16 10 13
    // 192: 17 8 14
    // 21037: 9 7 18 13
    // 292: 11 6 16 20
    // "#;
    println!("part1={}", part1(&puzzle_input)?);
    println!("part2={}", part2(&puzzle_input)?);
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<String> {
    let equations: Vec<_> = parse(input).collect();
    let ans: usize = equations
        .iter()
        .filter_map(|eq| eq.eval().then_some(eq.lhs))
        .sum();
    Ok(ans.to_string())
}

fn part2(input: &str) -> anyhow::Result<String> {
    let _ = parse(input);
    let ans = 0;
    Ok(ans.to_string())
}

#[derive(Debug)]
struct Equation {
    lhs: usize,
    rhs: Vec<usize>,
}
impl Equation {
    fn eval(&self) -> bool {
        let nb_digits = self.rhs.len() - 1;
        let nb_tests = 2_i32.pow(nb_digits as u32) as usize;

        for test in 0..nb_tests {
            let ops = usize_to_ops(test, nb_digits);
            let rhs = self.eval_comb(&ops);
            if self.lhs == rhs {
                return true;
            }
        }
        false
    }
    fn eval_comb(&self, ops: &[Operation]) -> usize {
        let mut n0 = self.rhs[0];
        for (i, &op) in ops.iter().enumerate() {
            let n1 = self.rhs[i + 1];

            n0 = match op {
                Operation::Add => n0 + n1,
                Operation::Multiply => n0 * n1,
            };
        }

        n0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operation {
    Add,
    Multiply,
}
impl TryFrom<usize> for Operation {
    type Error = anyhow::Error;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Operation::Add),
            1 => Ok(Operation::Multiply),
            _ => Err(anyhow!("Invalid operation")),
        }
    }
}

fn parse(input: &str) -> impl Iterator<Item = Equation> {
    input.split('\n').filter_map(|line| {
        if line.is_empty() {
            None
        } else {
            let mut line_split = line.split(':');
            let lhs = line_split.next().unwrap().trim().parse().unwrap();
            let rhs: Vec<usize> = line_split
                .next()
                .unwrap()
                .split_whitespace()
                .filter_map(|n| {
                    if !n.is_empty() {
                        Some(n.parse().unwrap())
                    } else {
                        None
                    }
                })
                .collect();

            Some(Equation { lhs, rhs })
        }
    })
}

fn usize_to_ops(mut n: usize, nb_digits: usize) -> Vec<Operation> {
    let mut out = vec![Operation::Add; nb_digits];
    let mut i = 0;
    loop {
        out[(nb_digits - 1) - i] = (n % 2).try_into().unwrap(); // can't be anything else than 0 or 1 so unwrap is fine
        n /= 2;
        if n == 0 {
            return out;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_bin() {
        assert_eq!(
            usize_to_ops(6, 3),
            vec![Operation::Multiply, Operation::Multiply, Operation::Add]
        );
        assert_eq!(
            usize_to_ops(15, 4),
            vec![
                Operation::Multiply,
                Operation::Multiply,
                Operation::Multiply,
                Operation::Multiply
            ]
        );
    }
}
