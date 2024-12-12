use itertools::Itertools;
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Topology {
    nodes: Vec<Vec<i32>>,
    num_rows: i32,
    num_cols: i32,
}

const TRANSLATIONS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

impl Topology {
    fn new(nodes: Vec<Vec<i32>>) -> Self {
        let num_rows = nodes.len() as i32;
        let num_cols = nodes[0].len() as i32;

        Topology {
            nodes,
            num_rows,
            num_cols,
        }
    }

    fn trailheads(&self) -> Vec<(i32, i32)> {
        let mut trailheads = Vec::new();

        for (row_idx, row) in self.nodes.iter().enumerate() {
            for (col_idx, height) in row.iter().enumerate() {
                if *height == 0 {
                    trailheads.push((row_idx as i32, col_idx as i32));
                }
            }
        }

        trailheads
    }

    fn neighbours(
        &self,
        row_idx: i32,
        col_idx: i32,
    ) -> Vec<(i32, i32, i32)> {
        TRANSLATIONS
            .iter()
            .filter_map(|(row_delta, col_delta)| {
                let row_idx = row_idx + row_delta;
                let col_idx = col_idx + col_delta;

                if row_idx < 0 || row_idx >= self.num_rows {
                    return None;
                }
                if col_idx < 0 || col_idx >= self.num_cols {
                    return None;
                }

                Some((
                    row_idx,
                    col_idx,
                    self.nodes[row_idx as usize][col_idx as usize],
                ))
            })
            .collect()
    }

    fn find_paths(&self) -> Vec<Vec<(i32, i32, i32)>> {
        let trailheads = self.trailheads();

        let mut paths = Vec::new();
        trailheads
            .into_iter()
            .for_each(|(row_idx, col_idx)| paths.push(vec![(row_idx, col_idx, 0)]));

        loop {
            if paths
                .iter()
                .all(|path| path.last().is_some_and(|(_, _, height)| *height == 9))
            {
                break;
            }

            let mut new_paths = Vec::new();
            for path in paths {
                if let Some((row_idx, col_idx, height)) = path.last() {
                    if *height == 9 {
                        new_paths.push(path);
                        continue;
                    }

                    for neighbour in self.neighbours(*row_idx, *col_idx).into_iter().filter(
                        |(row_idx, col_idx, neighbour_height)| {
                            !path.contains(&(*row_idx, *col_idx, *neighbour_height))
                                && *neighbour_height == height + 1
                        },
                    ) {
                        let (row_idx, col_idx, height) = neighbour;
                        let mut new_path = path.clone();
                        new_path.push((row_idx, col_idx, height));
                        new_paths.push(new_path);
                    }
                }
            }

            paths = new_paths;
        }

        paths
    }
}

fn parse(data: &str) -> Topology {
    Topology::new(
        data.lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect()
            })
            .collect(),
    )
}

fn first(
    name: &str,
    data: &str,
) {
    let topology = parse(data);
    let paths = topology.find_paths();

    let score = paths
        .iter()
        .into_group_map_by(|path| path.first().unwrap())
        .values()
        .map(|v| {
            v.iter()
                .filter_map(|path| path.last())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum::<usize>();

    println!("{}: Score is '{:?}'", name, score);
}

fn second(
    name: &str,
    data: &str,
) {
    let topology = parse(data);
    let paths = topology.find_paths();
    let score = paths.len();

    println!("{}: Score is '{:?}'", name, score);
}

pub fn run() {
    first("First example", include_str!("data/day10/ex1"));
    first("First", include_str!("data/day10/input"));
    second("Second example", include_str!("data/day10/ex1"));
    second("Second", include_str!("data/day10/input"));
}
