use nom::{
    branch::alt,
    character::complete::{
        line_ending,
        none_of,
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
use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
enum Value {
    Void,
    Antenna(char),
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Value>>> {
    all_consuming(many1(terminated(
        many1(alt((
            value(Value::Void, nom::character::complete::char('.')),
            map(none_of("\n\r"), Value::Antenna),
        ))),
        opt(line_ending),
    )))(input)
}

fn expand(
    row_index: i32,
    col_index: i32,
    row_translation: i32,
    col_translation: i32,
    num_rows: i32,
    num_cols: i32,
    max_steps: Option<usize>,
) -> Vec<(i32, i32)> {
    let mut coords = Vec::new();

    let mut r = row_index;
    let mut c = col_index;

    let mut steps = 0;

    while r >= 0 && r < num_rows && c >= 0 && c < num_cols {
        if let Some(max_steps) = max_steps {
            if steps > max_steps {
                break;
            }
        }

        coords.push((r, c));
        r += row_translation;
        c += col_translation;
        steps += 1;
    }

    coords
}

fn print(
    values: &[Vec<Value>],
    antinodes: &HashSet<(i32, i32)>,
) {
    for (row_index, row) in values.iter().enumerate() {
        for (col_index, value) in row.iter().enumerate() {
            if antinodes.contains(&(row_index as i32, col_index as i32)) {
                print!("#");
                continue;
            }

            match value {
                Value::Void => print!("."),
                Value::Antenna(antenna) => print!("{}", antenna),
            }
        }
        println!();
    }
}

fn solve(
    name: &str,
    data: &str,
    max_steps: Option<usize>,
    antinode_can_be_antenna: bool,
) {
    let (_, values) = parse(data).finish().unwrap();
    let num_rows = values.len() as i32;
    let num_cols = values[0].len() as i32;

    let antennas = values
        .iter()
        .enumerate()
        .flat_map(|(row_idx, cols)| {
            cols.iter().enumerate().filter_map(move |(col_idx, value)| {
                match value {
                    Value::Antenna(antenna) => Some((row_idx, col_idx, *antenna)),
                    _ => None,
                }
            })
        })
        .collect::<Vec<_>>();

    let mut coords = HashSet::new();

    for (i1, (r1, c1, f1)) in antennas.iter().enumerate() {
        for (i2, (r2, c2, f2)) in antennas.iter().enumerate() {
            let r1 = *r1 as i32;
            let c1 = *c1 as i32;
            let r2 = *r2 as i32;
            let c2 = *c2 as i32;

            if i1 == i2 || f1 != f2 {
                continue;
            }

            let r_diff = r1 - r2;
            let c_diff = c1 - c2;

            for (r, c) in expand(r1, c1, r_diff, c_diff, num_rows, num_cols, max_steps).iter() {
                if !antinode_can_be_antenna && *r == r1 && *c == c1 {
                    continue;
                }

                if !antinode_can_be_antenna && *r == r2 && *c == c2 {
                    continue;
                }

                coords.insert((*r, *c));
            }
        }
    }

    print(&values, &coords);

    println!("{}: Antinodes '{:?}'", name, coords.len());
}

pub fn run() {
    solve(
        "First Example",
        include_str!("data/day8/ex1"),
        Some(1),
        false,
    );
    solve("First", include_str!("data/day8/input"), Some(1), false);
    solve("Second Example", include_str!("data/day8/ex1"), None, true);
    solve("Second", include_str!("data/day8/input"), None, true);
}
