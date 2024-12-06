use nom::{
    branch::alt,
    character::complete::{
        char,
        line_ending,
    },
    combinator::{
        all_consuming,
        map,
        opt,
        value,
    },
    multi::many1,
    sequence::terminated,
    Finish,
    IResult,
};
use rayon::prelude::*;
use std::{
    collections::{
        HashMap,
        HashSet,
    },
    fmt::Display,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Value {
    Obstruction,
    Guard(Direction),
    Empty,
}

#[derive(Clone, Debug)]
struct Puzzle {
    values: Vec<Vec<Value>>,
    visited: HashMap<(usize, usize), HashSet<Direction>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Outcome {
    Loop,
    OutOfBounds,
    Continue,
}

impl Puzzle {
    fn new(values: Vec<Vec<Value>>) -> Self {
        Self {
            values,
            visited: HashMap::new(),
        }
    }

    // Returns true if there is at least one guard in the puzzle.
    fn move_guard(&mut self) -> Outcome {
        let mut outcome = Outcome::Continue;
        let guard = self.guard();

        if let Some(((row_idx, col_idx), direction)) = guard {
            if self
                .visited
                .entry((row_idx, col_idx))
                .or_default()
                .contains(&direction)
            {
                outcome = Outcome::Loop;
            } else {
                self.visited
                    .entry((row_idx, col_idx))
                    .or_default()
                    .insert(direction);
                if let Some((new_row_idx, new_col_idx)) = self.next(row_idx, col_idx, direction) {
                    let next_value = self.values[new_row_idx][new_col_idx];

                    if next_value == Value::Obstruction {
                        // Turn right
                        let new_direction = match direction {
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Up,
                            Direction::Right => Direction::Down,
                        };

                        self.values[row_idx][col_idx] = Value::Guard(new_direction);
                    } else {
                        self.values[new_row_idx][new_col_idx] = Value::Guard(direction);
                        self.values[row_idx][col_idx] = Value::Empty;
                    }
                } else {
                    self.values[row_idx][col_idx] = Value::Empty;
                    outcome = Outcome::OutOfBounds;
                }
            }
        }

        outcome
    }

    fn guard(&self) -> Option<((usize, usize), Direction)> {
        self.values
            .iter()
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row.iter().enumerate().filter_map(move |(col_idx, value)| {
                    match value {
                        Value::Guard(direction) => Some(((row_idx, col_idx), *direction)),
                        _ => None,
                    }
                })
            })
            .next()
    }

    fn next(
        &self,
        row_idx: usize,
        col_idx: usize,
        direction: Direction,
    ) -> Option<(usize, usize)> {
        match direction {
            Direction::Up => {
                if row_idx == 0 {
                    None
                } else {
                    Some((row_idx - 1, col_idx))
                }
            }
            Direction::Down => {
                if row_idx == self.values.len() - 1 {
                    None
                } else {
                    Some((row_idx + 1, col_idx))
                }
            }
            Direction::Left => {
                if col_idx == 0 {
                    None
                } else {
                    Some((row_idx, col_idx - 1))
                }
            }
            Direction::Right => {
                if col_idx == self.values[row_idx].len() - 1 {
                    None
                } else {
                    Some((row_idx, col_idx + 1))
                }
            }
        }
    }
}

impl Display for Puzzle {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        for (row_idx, row) in self.values.iter().enumerate() {
            for (col_idx, value) in row.iter().enumerate() {
                let c = match value {
                    Value::Obstruction => '#',
                    Value::Guard(Direction::Up) => '^',
                    Value::Guard(Direction::Left) => '<',
                    Value::Guard(Direction::Right) => '>',
                    Value::Guard(Direction::Down) => 'V',
                    Value::Empty => {
                        if self.visited.contains_key(&(row_idx, col_idx)) {
                            'X'
                        } else {
                            '.'
                        }
                    }
                };

                write!(f, "{}", c)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

fn parse(input: &str) -> IResult<&str, Puzzle> {
    all_consuming(map(
        many1(terminated(
            many1(alt((
                value(Value::Obstruction, char('#')),
                value(Value::Guard(Direction::Up), char('^')),
                value(Value::Guard(Direction::Left), char('<')),
                value(Value::Guard(Direction::Right), char('>')),
                value(Value::Guard(Direction::Down), char('V')),
                value(Value::Empty, char('.')),
            ))),
            opt(line_ending),
        )),
        Puzzle::new,
    ))(input)
}

fn run_puzzle(puzzle: &mut Puzzle) -> Outcome {
    loop {
        // println!("{}\n\n", puzzle);
        match puzzle.move_guard() {
            Outcome::Loop => {
                return Outcome::Loop;
            }
            Outcome::OutOfBounds => {
                return Outcome::OutOfBounds;
            }
            Outcome::Continue => {}
        }
    }
}

fn first(
    name: &str,
    input: &str,
) {
    let (_, mut puzzle) = parse(input).finish().unwrap();

    run_puzzle(&mut puzzle);

    println!("{}: Visited '{}'", name, puzzle.visited.keys().len());
}

fn second(
    name: &str,
    input: &str,
) {
    let (_, original) = parse(input).finish().unwrap();

    // Obstacles should be on visited spots
    let candidates = {
        let mut puzzle = original.clone();
        run_puzzle(&mut puzzle);
        puzzle
            .visited
            .keys()
            .to_owned()
            .cloned()
            .filter(|coords| original.guard().unwrap().0 != *coords)
            .collect::<Vec<_>>()
    };

    let loops = candidates
        .par_iter()
        .filter(|(row_idx, col_idx)| {
            let mut new_puzzle = original.clone();
            new_puzzle.values[*row_idx][*col_idx] = Value::Obstruction;
            run_puzzle(&mut new_puzzle) == Outcome::Loop
        })
        .count();

    println!("{}: Loops '{}'", name, loops);
}

pub fn run() {
    first("First example", include_str!("data/day6/ex1"));
    first("First", include_str!("data/day6/input"));
    second("Second example", include_str!("data/day6/ex1"));
    second("Second", include_str!("data/day6/input"));
}
