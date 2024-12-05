use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::{
        all_consuming,
        map,
        opt,
    },
    multi::{
        many1,
        separated_list1,
    },
    sequence::{
        separated_pair,
        terminated,
    },
    Finish,
    IResult,
};
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq)]
struct OrderingRule {
    left: i32,
    right: i32,
}

#[derive(Clone, Debug)]
struct Puzzle {
    rules: Vec<OrderingRule>,
    updates: Vec<Vec<i32>>,
}

#[derive(Clone, Debug)]
struct Updates {
    safe: Vec<Vec<i32>>,
    not_safe: Vec<Vec<i32>>,
}

impl Puzzle {
    fn is_safe_update(
        &self,
        update: &[i32],
    ) -> bool {
        for (idx, value) in update.iter().enumerate() {
            let mut pages_before = HashSet::new();
            let mut pages_after = HashSet::new();

            for rule in self.rules.iter() {
                if rule.left == *value {
                    pages_after.insert(rule.right);
                }

                if rule.right == *value {
                    pages_before.insert(rule.left);
                }
            }

            let before_safe = update[0..idx].iter().all(|b| !pages_after.contains(b));
            let after_safe = update[idx..].iter().all(|a| !pages_before.contains(a));

            if !before_safe || !after_safe {
                return false;
            }
        }

        true
    }

    fn partition_updates(&self) -> Updates {
        let mut safe = Vec::new();
        let mut not_safe = Vec::new();

        for update in self.updates.iter() {
            if self.is_safe_update(update) {
                safe.push(update.clone());
            } else {
                not_safe.push(update.clone());
            }
        }

        Updates { safe, not_safe }
    }
}

fn parse(input: &str) -> IResult<&str, Puzzle> {
    let rules = many1(terminated(
        map(
            separated_pair(
                nom::character::complete::i32,
                tag("|"),
                nom::character::complete::i32,
            ),
            |(left, right)| OrderingRule { left, right },
        ),
        opt(line_ending),
    ));

    let updates = many1(terminated(
        separated_list1(tag(","), nom::character::complete::i32),
        opt(line_ending),
    ));

    all_consuming(map(
        separated_pair(rules, line_ending, updates),
        |(rules, updates)| Puzzle { rules, updates },
    ))(input)
}

fn sum_middle_value(updates: &[Vec<i32>]) -> i32 {
    updates.iter().map(|update| update[update.len() / 2]).sum()
}

fn first(
    name: &str,
    data: &str,
) {
    let (_, puzzle) = parse(data).finish().unwrap();

    let Updates { safe, .. } = puzzle.partition_updates();
    let sum_middle = sum_middle_value(&safe);

    println!("{}: Sum of middle pages is '{:#?}'", name, sum_middle);
}

fn second(
    name: &str,
    data: &str,
) {
    let (_, puzzle) = parse(data).finish().unwrap();

    let Updates { not_safe, .. } = puzzle.partition_updates();

    let mut fixed = Vec::new();
    for update in not_safe.iter() {
        let sorted = update
            .iter()
            .copied()
            .sorted_by(|a, b| {
                if puzzle.rules.contains(&OrderingRule {
                    left: *b,
                    right: *a,
                }) {
                    std::cmp::Ordering::Greater
                } else if puzzle.rules.contains(&OrderingRule {
                    left: *a,
                    right: *b,
                }) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            })
            .collect::<Vec<_>>();
        fixed.push(sorted);
    }

    let sum_middle = sum_middle_value(&fixed);

    println!("{}: Sum of middle pages is '{:#?}'", name, sum_middle);
}

pub fn run() {
    first("First Example", include_str!("data/day5/ex1"));
    first("First", include_str!("data/day5/input"));
    second("Second Example", include_str!("data/day5/ex1"));
    second("Second", include_str!("data/day5/input"));
}
