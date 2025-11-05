use anyhow::anyhow;
use aoc_utils::fetch_puzzle_input;

fn main() -> anyhow::Result<()> {
    let puzzle_input = fetch_puzzle_input(7)?;
    println!("part1={}", part1(&puzzle_input)?);
    println!("part2={}", part2(&puzzle_input)?);
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<String> {
    let equations: Vec<_> = parse(input).collect();
    let ans: usize = equations
        .iter()
        .filter_map(|eq| eq.eval(2).ok()?.then_some(eq.lhs))
        .sum();
    Ok(ans.to_string())
}

fn part2(input: &str) -> anyhow::Result<String> {
    let equations: Vec<_> = parse(input).collect();
    let ans: usize = equations
        .iter()
        .filter_map(|eq| eq.eval(3).ok()?.then_some(eq.lhs))
        .sum();
    Ok(ans.to_string())
}

#[derive(Debug)]
struct Equation {
    lhs: usize,
    rhs: Vec<usize>,
}
impl Equation {
    fn eval(&self, nb_ops: usize) -> anyhow::Result<bool> {
        let nb_digits = self.rhs.len() - 1;
        let nb_combs = nb_ops.pow(nb_digits as u32);

        /*
        create a combinations of operations. below is an example with 3 ops and 2 digits
            where + -> 0    * -> 1      || -> 2

        i   comb[0] comb[1]
        0     0       0
        1     0       1
        2     0       2
        3     1       0
        4     1       1
        5     1       2
        6     2       0
        7     2       1
        8     2       2
        */
        for i in 0..nb_combs {
            let comb = make_comb(i, nb_digits, nb_ops)?;
            let rhs = self.eval_comb(&comb);
            if self.lhs == rhs {
                return Ok(true);
            }
        }
        Ok(false)
    }
    fn eval_comb(&self, comb: &[Operation]) -> usize {
        let mut n0 = self.rhs[0];
        for (i, &op) in comb.iter().enumerate() {
            let n1 = self.rhs[i + 1];

            n0 = match op {
                Operation::Add => n0 + n1,
                Operation::Multiply => n0 * n1,
                Operation::Combine => concat(n0, n1),
            };
        }

        n0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operation {
    Add,
    Multiply,
    Combine,
}

impl TryFrom<usize> for Operation {
    type Error = anyhow::Error;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Operation::Add),
            1 => Ok(Operation::Multiply),
            2 => Ok(Operation::Combine),
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

fn make_comb(mut n: usize, nb_digits: usize, nb_ops: usize) -> anyhow::Result<Vec<Operation>> {
    let mut out = vec![Operation::Add; nb_digits];
    let mut i = 0;
    loop {
        out[(nb_digits - 1) - i] = (n % nb_ops).try_into()?;
        n /= nb_ops;
        if n == 0 {
            return Ok(out);
        }
        i += 1;
    }
}

fn concat(x: usize, y: usize) -> usize {
    x * 10_usize.pow(y.ilog10() + 1) + y
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_bin() {
        assert_eq!(
            make_comb(6, 3, 2).unwrap(),
            vec![Operation::Multiply, Operation::Multiply, Operation::Add]
        );
        assert_eq!(
            make_comb(15, 4, 2).unwrap(),
            vec![
                Operation::Multiply,
                Operation::Multiply,
                Operation::Multiply,
                Operation::Multiply
            ]
        );
        assert_eq!(
            make_comb(7, 2, 3).unwrap(),
            vec![Operation::Combine, Operation::Multiply]
        )
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat(10, 10), 1010);
        assert_eq!(concat(1, 7), 17);
        assert_eq!(concat(111, 111), 111111);
        assert_eq!(concat(1, 783), 1783);
        assert_eq!(concat(1278, 5), 12785);
        assert_eq!(concat(582457, 319), 582457319);
    }
}
