use std::{default, fmt::Display, str::FromStr};

use anyhow::anyhow;
use aoc_utils::fetch_puzzle_input;

fn main() -> anyhow::Result<()> {
    let puzzle_input = fetch_puzzle_input(6)?;
    //     let puzzle_input = "....#.....
    // .........#
    // ..........
    // ..#.......
    // .......#..
    // ..........
    // .#..^.....
    // ........#.
    // #.........
    // ......#..."
    //         .to_string();
    println!("part1={}", part1(&puzzle_input)?);
    println!("part2={}", part2(&puzzle_input)?);
    Ok(())
}
fn part1(input: &str) -> anyhow::Result<String> {
    let mut guard = Guard::new(Map::from_str(input)?)?;
    guard.run();
    println!("{}", guard.map);
    // run simulation

    let ans: usize = guard.map.count_visited();
    Ok(ans.to_string())
}

fn part2(input: &str) -> anyhow::Result<String> {
    let ans: usize = 0;
    Ok(ans.to_string())
}

#[derive(Debug)]
struct Map(Vec<Vec<MapObject>>);
impl Map {
    fn guard_loc(&self) -> Option<(usize, usize)> {
        for (i, row) in self.0.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if let MapObject::Guard = col {
                    return Some((i, j));
                }
            }
        }
        None
    }
    fn is_obstructed(&self, loc: (usize, usize)) -> bool {
        matches!(self.0[loc.0][loc.1], MapObject::Obstruction)
    }

    fn set_object(&mut self, loc: (usize, usize), obj: MapObject) {
        self.0[loc.0][loc.1] = obj;
    }

    fn shape(&self) -> (usize, usize) {
        let row = self.0.len();
        let col = self.0.first().unwrap().iter().count();
        (row, col)
    }
    fn count_visited(&self) -> usize {
        self.0
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|col| matches!(col, MapObject::GuardVisited))
                    .count()
            })
            .sum()
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .split('\n')
            .map(|line| {
                line.chars()
                    .map(MapObject::try_from)
                    .collect::<anyhow::Result<Vec<_>>>()
            })
            .collect::<anyhow::Result<Vec<_>>>();
        Ok(Map(map?))
    }
}
impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for row in self.0.iter() {
            for col in row {
                out.push_str(&col.to_string());
            }
            out.push('\n');
        }
        write!(f, "{out}")
    }
}

#[derive(Debug)]
enum MapObject {
    Obstruction,
    Nothing,
    Guard,
    GuardVisited,
}

impl TryFrom<char> for MapObject {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(MapObject::Nothing),
            '#' => Ok(MapObject::Obstruction),
            '^' => Ok(MapObject::Guard),
            'X' => Ok(MapObject::GuardVisited),
            _ => Err(anyhow::anyhow!("Unrecognized map object {value}")),
        }
    }
}

impl Display for MapObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapObject::Nothing => write!(f, "."),
            MapObject::Obstruction => write!(f, "#"),
            MapObject::Guard => write!(f, "^"),
            MapObject::GuardVisited => write!(f, "X"),
        }
    }
}

struct Guard {
    dir: Direction,
    loc: (usize, usize),
    map: Map,
}
impl Guard {
    pub fn new(map: Map) -> anyhow::Result<Guard> {
        let guard_loc = map
            .guard_loc()
            .ok_or(anyhow::anyhow!("No guard present on the map!"))?;
        Ok(Guard {
            dir: Direction::default(),
            loc: guard_loc,
            map,
        })
    }
    pub fn run(&mut self) {
        // self.mv() returns true when exiting the map
        while !self.mv() {}
    }

    fn mv(&mut self) -> bool {
        self.map.set_object(self.loc, MapObject::GuardVisited);
        if let Some(next_loc) = self.next_loc() {
            if self.map.is_obstructed(next_loc) {
                self.dir = self.next_dir();
                return self.mv(); // mv in the new direction
            // set object
            } else {
                self.loc = next_loc;
                self.map.set_object(self.loc, MapObject::Guard);
            }

            false
        } else {
            // we exit the map, we are done
            true
        }
    }
    fn next_loc(&self) -> Option<(usize, usize)> {
        let (row, col) = self.loc;
        let (next_row, next_col) = match self.dir {
            Direction::Up => (row as isize - 1, col as isize),
            Direction::Down => (row as isize + 1, col as isize),
            Direction::Left => (row as isize, col as isize - 1),
            Direction::Right => (row as isize, col as isize + 1),
        };

        let (max_row, max_col) = self.map.shape();
        if (next_row < 0)
            || (next_row >= max_row as isize)
            || (next_col < 0)
            || (next_col >= max_col as isize)
        {
            None
        } else {
            Some((next_row as usize, next_col as usize))
        }
    }
    fn next_dir(&self) -> Direction {
        match self.dir {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Up
    }
}
