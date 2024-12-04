use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{
        all_consuming,
        map,
        value,
    },
    multi::many1,
    sequence::tuple,
    Finish,
    IResult,
};

#[derive(Clone, Debug)]
struct Mul {
    left: i32,
    right: i32,
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Do,
    DoNot,
}

#[derive(Clone, Debug)]
enum Value {
    Garbage,
    Mul(Mul),
    Instruction(Instruction),
}

fn parse(input: &str) -> IResult<&str, Vec<Value>> {
    all_consuming(many1(alt((
        map(
            tuple((
                tag("mul"),
                tag("("),
                nom::character::complete::i32,
                tag(","),
                nom::character::complete::i32,
                tag(")"),
            )),
            |(_, _, left, _, right, _)| Value::Mul(Mul { left, right }),
        ),
        value(Value::Instruction(Instruction::Do), tag("do()")),
        value(Value::Instruction(Instruction::DoNot), tag("don't()")),
        value(Value::Garbage, anychar),
    ))))(input)
}

fn first(
    name: &str,
    data: &str,
) {
    let (_, data) = parse(data).finish().unwrap();
    let result: i32 = data
        .iter()
        .map(|value| {
            match value {
                Value::Garbage => 0,
                Value::Mul(mul) => mul.left * mul.right,
                Value::Instruction(_) => 0,
            }
        })
        .sum();
    println!("{}: Result is '{:#?}'", name, result);
}

fn second(
    name: &str,
    data: &str,
) {
    let (_, data) = parse(data).finish().unwrap();
    let mut result = 0;
    let mut instruction = Instruction::Do;

    for value in data.iter() {
        match value {
            Value::Garbage => (),
            Value::Mul(mul) => {
                match instruction {
                    Instruction::Do => result += mul.left * mul.right,
                    Instruction::DoNot => (),
                }
            }
            Value::Instruction(new_instruction) => instruction = *new_instruction,
        }
    }

    println!("{}: Result is '{:#?}'", name, result);
}

pub fn run() {
    first("First Example", include_str!("data/day3/ex1"));
    first("First", include_str!("data/day3/input"));
    second("Second Example", include_str!("data/day3/ex2"));
    second("Second", include_str!("data/day3/input"));
}
