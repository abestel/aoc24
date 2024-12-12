use std::collections::HashSet;

const TRANSLATIONS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

struct Puzzle {
    gardens: Vec<Vec<char>>,
    num_rows: i32,
    num_cols: i32,
}

impl Puzzle {
    fn new(gardens: Vec<Vec<char>>) -> Self {
        let num_rows = gardens.len() as i32;
        let num_cols = gardens[0].len() as i32;

        Self {
            gardens,
            num_rows,
            num_cols,
        }
    }

    fn find_zones(&self) -> Vec<Zone> {
        let mut all_visited = HashSet::new();
        let mut zones = Vec::new();

        for (row_idx, row) in self.gardens.iter().enumerate() {
            for (col_idx, garden) in row.iter().enumerate() {
                if all_visited.contains(&(row_idx as i32, col_idx as i32)) {
                    continue;
                }

                // Start a new zone
                let mut visited = HashSet::new();
                let mut next = HashSet::new();
                next.insert((row_idx as i32, col_idx as i32));

                loop {
                    if next.is_empty() {
                        break;
                    }

                    let mut new_next = HashSet::new();
                    for (row_idx, col_idx) in &next {
                        new_next.extend(TRANSLATIONS.iter().filter_map(
                            |(row_delta, col_delta)| {
                                let row_idx = row_idx + row_delta;
                                let col_idx = col_idx + col_delta;

                                if row_idx < 0 || row_idx >= self.num_rows {
                                    return None;
                                }
                                if col_idx < 0 || col_idx >= self.num_cols {
                                    return None;
                                }
                                if visited.contains(&(row_idx, col_idx)) {
                                    return None;
                                }
                                if all_visited.contains(&(row_idx, col_idx)) {
                                    return None;
                                }

                                let value = self.gardens[row_idx as usize][col_idx as usize];
                                if value != *garden {
                                    return None;
                                }

                                Some((row_idx, col_idx))
                            },
                        ));
                    }

                    visited.extend(next);

                    next = new_next;
                }

                zones.push(Zone {
                    coords: visited.clone(),
                    garden: *garden,
                });
                all_visited.extend(visited);
            }
        }

        zones
    }
}

fn parse(data: &str) -> Puzzle {
    Puzzle::new(data.lines().map(|line| line.chars().collect()).collect())
}

#[derive(Debug, Clone)]
struct Zone {
    garden: char,
    coords: HashSet<(i32, i32)>,
}

impl Zone {
    fn area(&self) -> usize {
        self.coords.len()
    }

    fn perimeter_contributors(&self) -> Vec<(i32, i32)> {
        let mut perimeter_contributors = Vec::new();

        for (row_idx, col_idx) in &self.coords {
            for (row_delta, col_delta) in &TRANSLATIONS {
                let row_idx = row_idx + row_delta;
                let col_idx = col_idx + col_delta;

                if !self.coords.contains(&(row_idx, col_idx)) {
                    perimeter_contributors.push((row_idx, col_idx));
                }
            }
        }

        perimeter_contributors
    }

    fn perimeter(&self) -> usize {
        self.perimeter_contributors().len()
    }

    fn sides(&self) -> usize {
        let perimeter = self.perimeter_contributors();

        let mut corners = Vec::new();
        for (r, c) in self.coords.iter().copied() {
            // Top left corner
            if (perimeter.contains(&(r - 1, c)) && perimeter.contains(&(r, c - 1)))
                || (perimeter.contains(&(r - 1, c - 1))
                    && self.coords.contains(&(r - 1, c))
                    && self.coords.contains(&(r, c - 1)))
            {
                corners.push((r - 1, c - 1));
            }

            // Top right corner
            if (perimeter.contains(&(r - 1, c)) && perimeter.contains(&(r, c + 1)))
                || (perimeter.contains(&(r - 1, c + 1))
                    && self.coords.contains(&(r - 1, c))
                    && self.coords.contains(&(r, c + 1)))
            {
                corners.push((r - 1, c + 1));
            }

            // Bottom left corner
            if (perimeter.contains(&(r + 1, c)) && perimeter.contains(&(r, c - 1)))
                || (perimeter.contains(&(r + 1, c - 1))
                    && self.coords.contains(&(r + 1, c))
                    && self.coords.contains(&(r, c - 1)))
            {
                corners.push((r + 1, c - 1));
            }

            // Bottom right corner
            if (perimeter.contains(&(r + 1, c)) && perimeter.contains(&(r, c + 1)))
                || (perimeter.contains(&(r + 1, c + 1))
                    && self.coords.contains(&(r + 1, c))
                    && self.coords.contains(&(r, c + 1)))
            {
                corners.push((r + 1, c + 1));
            }
        }

        corners.len()
    }
}

fn first(
    name: &str,
    data: &str,
) {
    let data = parse(data);
    let zones = data.find_zones();

    let score = zones
        .iter()
        .map(|zone| zone.area() * zone.perimeter())
        .sum::<usize>();
    println!("{}: {:#?}", name, score);
}

fn second(
    name: &str,
    data: &str,
) {
    let data = parse(data);
    let zones = data.find_zones();

    let score = zones
        .iter()
        .map(|zone| zone.area() * zone.sides())
        .sum::<usize>();
    println!("{}: {:#?}", name, score);
}

pub fn run() {
    first("First example", include_str!("data/day12/ex1"));
    first("First", include_str!("data/day12/input"));
    second("Second example", include_str!("data/day12/ex1")); // 1206
    second("Second example 2", include_str!("data/day12/ex2")); // 368
    second("Second", include_str!("data/day12/input"));
}
