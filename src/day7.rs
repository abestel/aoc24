use nom::{
    character::complete::{
        char,
        line_ending,
        space1,
    },
    combinator::{
        all_consuming,
        map,
    },
    multi::separated_list1,
    sequence::{
        separated_pair,
        tuple,
    },
    Finish,
    IResult,
};

#[derive(Clone, Debug)]
struct Calibration {
    result: i64,
    numbers: Vec<i64>,
}

impl Calibration {
    fn has_matching_combination(
        &self,
        operators: Vec<Operator>,
    ) -> bool {
        let mut combinations = Vec::new();

        for (idx, number) in self.numbers.iter().enumerate() {
            if idx == 0 {
                combinations.push(*number);
            } else {
                combinations = combinations
                    .iter()
                    .flat_map(|prev| {
                        operators.iter().map(move |op| {
                            match op {
                                Operator::Add => prev + number,
                                Operator::Multiply => prev * number,
                                Operator::Concat => format!("{}{}", prev, number).parse().unwrap(),
                            }
                        })
                    })
                    .filter(|&num| num <= self.result)
                    .collect();
            }
        }

        combinations.iter().any(|num| *num == self.result)
    }
}

#[derive(Clone, Debug)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

fn parse(input: &str) -> IResult<&str, Vec<Calibration>> {
    all_consuming(separated_list1(
        line_ending,
        map(
            separated_pair(
                nom::character::complete::i64,
                tuple((char(':'), space1)),
                separated_list1(space1, nom::character::complete::i64),
            ),
            |(result, numbers)| Calibration { result, numbers },
        ),
    ))(input)
}

fn first(
    name: &str,
    data: &str,
) {
    let (_, calibrations) = parse(data).finish().unwrap();

    let res: i64 = calibrations
        .iter()
        .filter_map(|calibration| {
            if calibration.has_matching_combination(vec![Operator::Add, Operator::Multiply]) {
                Some(calibration.result)
            } else {
                None
            }
        })
        .sum();

    println!("{}: {}", name, res);
}

fn second(
    name: &str,
    data: &str,
) {
    let (_, calibrations) = parse(data).finish().unwrap();

    let res: i64 = calibrations
        .iter()
        .filter_map(|calibration| {
            if calibration.has_matching_combination(vec![
                Operator::Add,
                Operator::Multiply,
                Operator::Concat,
            ]) {
                Some(calibration.result)
            } else {
                None
            }
        })
        .sum();

    println!("{}: {}", name, res);
}

pub fn run() {
    first("First Example", include_str!("data/day7/ex1"));
    first("First", include_str!("data/day7/input"));
    second("Second Example", include_str!("data/day7/ex1"));
    second("Second", include_str!("data/day7/input"));
}
