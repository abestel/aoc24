use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::{
        all_consuming,
        opt,
        value,
    },
    multi::many1,
    sequence::terminated,
    Finish,
    IResult,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Value {
    X,
    M,
    A,
    S,
    Whatever,
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Value>>> {
    all_consuming(many1(terminated(
        many1(alt((
            value(Value::X, tag("X")),
            value(Value::M, tag("M")),
            value(Value::A, tag("A")),
            value(Value::S, tag("S")),
        ))),
        opt(line_ending),
    )))(input)
}

struct Puzzle {
    data: Vec<Vec<Value>>,
    num_rows: usize,
    num_cols: usize,
}

impl Puzzle {
    fn new(data: Vec<Vec<Value>>) -> Self {
        let num_rows = data.len();
        let num_cols = data[0].len();
        Self {
            data,
            num_rows,
            num_cols,
        }
    }

    fn get_opt(
        &self,
        row_idx: usize,
        col_idx: usize,
        translation: (i32, i32),
    ) -> Value {
        let new_row = row_idx as i32 + translation.0;
        let new_col = col_idx as i32 + translation.1;

        if new_row < 0 || new_row >= self.num_rows as i32 {
            return Value::Whatever;
        }

        if new_col < 0 || new_col >= self.num_cols as i32 {
            return Value::Whatever;
        }

        self.data[new_row as usize][new_col as usize]
    }

    fn iterate(
        &self,
        i: usize,
        j: usize,
        translations: &[Vec<(i32, i32)>],
    ) -> Vec<Vec<Value>> {
        let mut result = Vec::new();
        for translation in translations.iter() {
            let mut word = Vec::new();
            for t in translation {
                let value = self.get_opt(i, j, *t);
                word.push(value);
            }
            result.push(word);
        }
        result
    }
}

fn first(
    name: &str,
    data: &str,
) {
    let (_, data) = parse(data).finish().unwrap();
    let puzzle = Puzzle::new(data);

    let xmas = vec![Value::X, Value::M, Value::A, Value::S];
    let translations: Vec<Vec<(i32, i32)>> = vec![
        // Top
        vec![(0, 0), (-1, 0), (-2, 0), (-3, 0)],
        // Bottom
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        // Right
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        // Left
        vec![(0, 0), (0, -1), (0, -2), (0, -3)],
        // Top Right
        vec![(0, 0), (-1, 1), (-2, 2), (-3, 3)],
        // Top Left
        vec![(0, 0), (-1, -1), (-2, -2), (-3, -3)],
        // Bottom Right
        vec![(0, 0), (1, 1), (2, 2), (3, 3)],
        // Bottom Left
        vec![(0, 0), (1, -1), (2, -2), (3, -3)],
    ];

    let mut xmas_count = 0;
    for (row_idx, row) in puzzle.data.iter().enumerate() {
        for (col_idx, value) in row.iter().enumerate() {
            if value != &Value::X {
                continue;
            }

            let results = puzzle.iterate(row_idx, col_idx, &translations);
            xmas_count += results.iter().filter(|word| **word == xmas).count();
        }
    }

    println!("{}: XMAS: '{:?}'", name, xmas_count);
}

fn second(
    name: &str,
    data: &str,
) {
    let (_, data) = parse(data).finish().unwrap();
    let puzzle = Puzzle::new(data);

    let mas = vec![Value::M, Value::A, Value::S];
    let sam = vec![Value::S, Value::A, Value::M];

    let translations: Vec<Vec<(i32, i32)>> = vec![
        // Top Left -> Bottom Right
        vec![(-1, -1), (0, 0), (1, 1)],
        // Top Right -> Bottom Left
        vec![(-1, 1), (0, 0), (1, -1)],
    ];

    let mut xmas_count = 0;
    for (row_idx, row) in puzzle.data.iter().enumerate() {
        for (col_idx, value) in row.iter().enumerate() {
            if value != &Value::A {
                continue;
            }

            let results = puzzle.iterate(row_idx, col_idx, &translations);
            if results.iter().all(|word| *word == mas || *word == sam) {
                xmas_count += 1;
            }
        }
    }

    println!("{}: X-MAS: '{:?}'", name, xmas_count);
}

pub fn run() {
    first("First Example", include_str!("data/day4/ex1"));
    first("First", include_str!("data/day4/input"));
    second("Second Example", include_str!("data/day4/ex1"));
    second("Second", include_str!("data/day4/input"));
}
