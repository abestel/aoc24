use itertools::Itertools;
use nom::{
    character::complete::{
        line_ending,
        space1,
    },
    combinator::{
        all_consuming,
        opt,
    },
    multi::{
        many1,
        separated_list1,
    },
    sequence::terminated,
    Finish,
    IResult,
};

fn parse(data: &str) -> IResult<&str, Vec<Vec<i32>>> {
    all_consuming(many1(terminated(
        separated_list1(space1, nom::character::complete::i32),
        opt(line_ending),
    )))(data)
}

fn is_safe(level: &[i32]) -> bool {
    let mut is_increasing = false;
    for (idx, (x, y)) in level.iter().tuple_windows().enumerate() {
        let diff = y - x;
        if !(1..=3).contains(&diff.abs()) {
            return false;
        }

        if idx == 0 {
            is_increasing = diff > 0;
        } else {
            let is_increasing_new = diff > 0;
            if is_increasing != is_increasing_new {
                return false;
            }
        }
    }

    true
}

fn first(
    name: &str,
    data: &str,
) {
    let (_, data) = parse(data).finish().unwrap();
    let safe = data.iter().filter(|level| is_safe(level)).count();
    println!("{}: Safe levels: '{:?}'", name, safe);
}

fn second(
    name: &str,
    data: &str,
) {
    let (_, data) = parse(data).finish().unwrap();

    let mut safe_levels = 0;
    for level in data.iter() {
        if is_safe(level) {
            safe_levels += 1;
        } else {
            // Retry removing one element every time
            for (idx, _) in level.iter().enumerate() {
                let mut new_level = level.to_vec();
                new_level.remove(idx);
                if is_safe(&new_level) {
                    safe_levels += 1;
                    break;
                }
            }
        }
    }

    println!("{}: Safe levels: '{:?}'", name, safe_levels);
}

pub fn run() {
    first("First example", include_str!("data/day2/ex1"));
    first("First", include_str!("data/day2/input"));
    second("Second example", include_str!("data/day2/ex1"));
    second("Second", include_str!("data/day2/input"));
}
