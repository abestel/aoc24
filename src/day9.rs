#[derive(Clone, Debug)]
struct File {
    id: i32,
    size: i32,
}

#[derive(Clone, Debug)]
struct Empty {
    size: i32,
}

#[derive(Clone, Debug)]
enum Block {
    File(File),
    Empty(Empty),
}

#[derive(Clone, Debug)]
enum Bit {
    File(i32),
    Empty,
}

fn parse(input: &str) -> Vec<Block> {
    let mut blocks = Vec::new();

    let mut file_id = 0;
    for (idx, c) in input.chars().enumerate() {
        let size = c.to_digit(10).unwrap() as i32;
        if idx % 2 == 0 {
            blocks.push(Block::File(File { id: file_id, size }));
            file_id += 1;
        } else {
            blocks.push(Block::Empty(Empty { size }));
        }
    }

    blocks
}

fn first(
    name: &str,
    data: &str,
) {
    let blocks = parse(data);

    let total_size = blocks
        .iter()
        .map(|block| {
            match block {
                Block::File(File { size, .. }) => *size as usize,
                Block::Empty(Empty { size }) => *size as usize,
            }
        })
        .sum::<usize>();

    let mut disk = Vec::with_capacity(total_size);
    for block in blocks {
        match block {
            Block::File(File { size, id }) => {
                for _ in 0..size {
                    disk.push(Bit::File(id));
                }
            }
            Block::Empty(Empty { size }) => {
                for _ in 0..size {
                    disk.push(Bit::Empty);
                }
            }
        }
    }

    let mut left_index = 0;
    let mut right_index = disk.len() - 1;

    while left_index < right_index {
        let left = &disk[left_index];

        match left {
            Bit::File(_) => {
                left_index += 1;
                continue;
            }
            Bit::Empty => {}
        }

        let right = &disk[right_index];
        let right_file_id = match right {
            Bit::Empty => {
                right_index -= 1;
                continue;
            }
            Bit::File(file_id) => *file_id,
        };

        disk[left_index] = Bit::File(right_file_id);
        disk[right_index] = Bit::Empty;
    }

    let result = disk
        .iter()
        .enumerate()
        .map(|(idx, value)| {
            (idx as i64)
                * match value {
                    Bit::File(file_id) => *file_id as i64,
                    Bit::Empty => 0,
                }
        })
        .sum::<i64>();

    println!("{}: {}", name, result);
}

fn second(
    name: &str,
    data: &str,
) {
    let mut blocks = parse(data);

    let mut block_idx = blocks.len() - 1;
    while block_idx > 0 {
        let block = blocks[block_idx].clone();

        match block {
            Block::File(File { size, id }) => {
                // Find the first block that has enough empty space
                let empty_idx = blocks.iter().enumerate().find_map(|(idx, block)| {
                    if idx > block_idx {
                        None
                    } else {
                        match block {
                            Block::Empty(empty) => {
                                if empty.size >= size {
                                    Some((idx, empty.clone()))
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        }
                    }
                });

                if let Some((idx, empty)) = empty_idx {
                    blocks[block_idx] = Block::Empty(Empty { size });
                    if empty.size == size {
                        blocks[idx] = Block::File(File { size, id });
                        block_idx -= 1;
                    } else {
                        blocks[idx] = Block::File(File { size, id });
                        blocks.insert(
                            idx + 1,
                            Block::Empty(Empty {
                                size: empty.size - size,
                            }),
                        );
                    }
                } else {
                    block_idx -= 1;
                }
            }
            Block::Empty(_) => {
                block_idx -= 1;
                continue;
            }
        }
    }

    let result = blocks
        .iter()
        .flat_map(|block| {
            match block {
                Block::File(File { id, size }) => {
                    vec![*id; *size as usize]
                }
                Block::Empty(Empty { size }) => {
                    vec![0; *size as usize]
                }
            }
        })
        .enumerate()
        .map(|(idx, value)| (idx as i64) * (value as i64))
        .sum::<i64>();

    println!("{}: {}", name, result);
}

pub fn run() {
    first("First Example", include_str!("data/day9/ex1"));
    first("First", include_str!("data/day9/input"));
    second("Second Example", include_str!("data/day9/ex1"));
    second("Second", include_str!("data/day9/input"));
}
