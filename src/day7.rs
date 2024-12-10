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
    fn find_combinations(
        &self,
        operators: Box<dyn Fn(&Operator, i64) -> Vec<Operator>>,
    ) -> Vec<Operator> {
        let mut combinations = Vec::new();

        for (idx, number) in self.numbers.iter().enumerate() {
            if idx == 0 {
                combinations.push(Operator::Identity(*number));
            } else {
                combinations = combinations
                    .iter()
                    .flat_map(|op| operators(op, *number))
                    .collect();
            }
        }

        combinations
    }
}

#[derive(Clone, Debug)]
enum Operator {
    Identity(i64),
    Add(Box<Operator>, Box<Operator>),
    Multiply(Box<Operator>, Box<Operator>),
    Concat(Box<Operator>, Box<Operator>),
}

impl Operator {
    fn result(&self) -> i64 {
        match self {
            Operator::Identity(num) => *num,
            Operator::Add(left, right) => left.result() + right.result(),
            Operator::Multiply(left, right) => left.result() * right.result(),
            Operator::Concat(left, right) => {
                format!("{}{}", left.result(), right.result())
                    .parse()
                    .unwrap()
            }
        }
    }
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
            let combinations = calibration.find_combinations(Box::new(|op, number| {
                vec![
                    Operator::Add(Box::new(op.clone()), Box::new(Operator::Identity(number))),
                    Operator::Multiply(Box::new(op.clone()), Box::new(Operator::Identity(number))),
                ]
            }));

            let has_result = combinations
                .iter()
                .any(|op| op.result() == calibration.result);

            if has_result {
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
            let combinations = calibration.find_combinations(Box::new(|op, number| {
                vec![
                    Operator::Add(Box::new(op.clone()), Box::new(Operator::Identity(number))),
                    Operator::Multiply(Box::new(op.clone()), Box::new(Operator::Identity(number))),
                    Operator::Concat(Box::new(op.clone()), Box::new(Operator::Identity(number))),
                ]
            }));

            let has_result = combinations
                .iter()
                .any(|op| op.result() == calibration.result);

            if has_result {
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
