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
use std::collections::{
    HashMap,
    HashSet,
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct OrderingRule {
    left: i32,
    right: i32,
}

#[derive(Clone, Debug)]
struct Puzzle {
    left_to_right: HashMap<i32, HashSet<i32>>,
    updates: Vec<Vec<i32>>,
}

#[derive(Clone, Debug)]
struct Updates {
    safe: Vec<Vec<i32>>,
    not_safe: Vec<Vec<i32>>,
}

impl Puzzle {
    fn new(
        rules: Vec<OrderingRule>,
        updates: Vec<Vec<i32>>,
    ) -> Self {
        let left_to_right: HashMap<i32, HashSet<i32>> =
            rules.iter().fold(HashMap::new(), |mut acc, rule| {
                acc.entry(rule.left).or_default().insert(rule.right);
                acc
            });

        Self {
            left_to_right,
            updates,
        }
    }

    fn is_safe_update(
        &self,
        update: &[i32],
    ) -> bool {
        for (idx, value) in update.iter().enumerate() {
            let pages_after = self
                .left_to_right
                .get(value)
                .unwrap_or(&HashSet::new())
                .clone();
            if !update[0..idx].iter().all(|b| !pages_after.contains(b)) {
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
        |(rules, updates)| Puzzle::new(rules, updates),
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
                if puzzle
                    .left_to_right
                    .get(b)
                    .is_some_and(|set| set.contains(a))
                {
                    // Check if there is a rule stating that b is before a
                    std::cmp::Ordering::Greater
                } else if puzzle
                    .left_to_right
                    .get(a)
                    .is_some_and(|set| set.contains(b))
                {
                    // Check is there is a rule stating that a is before b
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
