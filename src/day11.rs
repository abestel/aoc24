use nom::{
    character::complete::{
        line_ending,
        space1,
    },
    combinator::{
        all_consuming,
        map,
        opt,
    },
    multi::separated_list1,
    sequence::terminated,
    Finish,
    IResult,
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Stone {
    number: i64,
}

fn parse(input: &str) -> IResult<&str, Vec<Stone>> {
    all_consuming(terminated(
        separated_list1(
            space1,
            map(nom::character::complete::i64, |number| Stone { number }),
        ),
        opt(line_ending),
    ))(input)
}

fn solve(
    name: &str,
    data: &str,
    iterations: usize,
) {
    let (_, init_stones) = parse(data).finish().unwrap();
    let mut stones = HashMap::new();

    for stone in init_stones.into_iter() {
        stones
            .entry(stone.number)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    for _ in 0..iterations {
        let mut new_stones = HashMap::new();
        for (number, count) in stones.into_iter() {
            if number == 0 {
                new_stones
                    .entry(1)
                    .and_modify(|new_count| *new_count += count)
                    .or_insert(count);
            } else {
                let number_str = number.to_string();

                if number_str.len() % 2 == 0 {
                    let half = number_str.len() / 2;
                    let (left, right) = number_str.split_at(half);
                    let left = left.parse::<i64>().unwrap_or_default();
                    let right = right.parse::<i64>().unwrap_or_default();

                    new_stones
                        .entry(left)
                        .and_modify(|new_count| *new_count += count)
                        .or_insert(count);
                    new_stones
                        .entry(right)
                        .and_modify(|new_count| *new_count += count)
                        .or_insert(count);
                } else {
                    new_stones
                        .entry(number * 2024)
                        .and_modify(|new_count| *new_count += count)
                        .or_insert(count);
                }
            }
        }

        stones = new_stones;
    }

    println!(
        "{}: number of stones: '{}'",
        name,
        stones.values().sum::<usize>()
    )
}

pub fn run() {
    solve("First Example", include_str!("data/day11/ex1"), 25);
    solve("First", include_str!("data/day11/input"), 25);
    solve("Second Example", include_str!("data/day11/ex1"), 75);
    solve("Second", include_str!("data/day11/input"), 75);
}
