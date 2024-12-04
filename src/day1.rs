use nom::{
    self,
    character::complete::{
        line_ending,
        space1,
    },
    combinator::{
        all_consuming,
        map,
        opt,
    },
    multi::many1,
    sequence::{
        separated_pair,
        terminated,
    },
    Finish,
};
use std::collections::HashMap;

fn parse(input: &str) -> nom::IResult<&str, (Vec<i32>, Vec<i32>)> {
    all_consuming(map(
        many1(terminated(
            separated_pair(
                nom::character::complete::i32,
                space1,
                nom::character::complete::i32,
            ),
            opt(line_ending),
        )),
        |vec| vec.into_iter().unzip(),
    ))(input)
}

fn first(
    name: &str,
    data: &str,
) {
    let (_, (mut left, mut right)) = parse(data).finish().unwrap();

    left.sort();
    right.sort();

    let sum: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    println!("{}: Total distance between lists is '{}'", name, sum);
}

fn occurrences(numbers: &[i32]) -> HashMap<i32, i32> {
    let mut occurrences = HashMap::new();

    for number in numbers {
        *occurrences.entry(*number).or_insert(0) += 1;
    }

    occurrences
}

fn second(
    name: &str,
    data: &str,
) {
    let (_, (left, right)) = parse(data).finish().unwrap();

    let left_occurrences = occurrences(&left);
    let right_occurrences = occurrences(&right);

    let sum: i32 = left_occurrences
        .iter()
        .map(|(number, count)| {
            let right_count = right_occurrences.get(number).unwrap_or(&0);
            number * count * right_count
        })
        .sum();

    println!("{}: Similarity between lists is '{}'", name, sum);
}

pub fn run() {
    first("First example", include_str!("data/day1/ex1"));
    first("First", include_str!("data/day1/input"));
    second("Second example", include_str!("data/day1/ex1"));
    second("Second", include_str!("data/day1/input"));
}
