use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::{Add, Sub},
};

#[derive(Debug)]
struct Pipe {
    type_: char,
    position: Point,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i64,
    y: i64,
}
impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}
impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[allow(dead_code)]
fn part_1(input: &str) -> i64 {
    let pipes_map = HashMap::from([
        ('|', [Point::new(0, -1), Point::new(0, 1)].to_vec()),
        ('-', [Point::new(-1, 0), Point::new(1, 0)].to_vec()),
        ('L', [Point::new(0, -1), Point::new(1, 0)].to_vec()),
        ('J', [Point::new(0, -1), Point::new(-1, 0)].to_vec()),
        ('7', [Point::new(0, 1), Point::new(-1, 0)].to_vec()),
        ('F', [Point::new(0, 1), Point::new(1, 0)].to_vec()),
        ('.', [].to_vec()),
        (
            'S',
            [
                Point::new(0, 1),
                Point::new(1, 0),
                Point::new(0, -1),
                Point::new(-1, 0),
            ]
            .to_vec(),
        ),
    ]);

    let mut starting_position = Point::new(0, 0);

    let _ = input.lines().enumerate().find(|(index, line)| {
        if line.contains('S') {
            let y_position = index.to_owned() as i64;
            let x_position = line.chars().position(|char| char == 'S').unwrap() as i64;

            starting_position = Point::new(x_position, y_position);
            true
        } else {
            false
        }
    });

    let input_matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    // Create the first pipe
    let head = Box::new(Pipe {
        type_: 'S',
        position: starting_position,
    });

    let mut seen_pipes: HashSet<Point> = HashSet::new();
    let mut distances: HashMap<char, i64> = HashMap::new();

    let mut bfs_queue = VecDeque::new();

    bfs_queue.push_back((head, 0));

    while !bfs_queue.is_empty() {
        if let Some((pipe, dist)) = bfs_queue.pop_front() {
            if seen_pipes.contains(&pipe.position) {
                continue;
            }

            seen_pipes.insert(pipe.position);
            distances.insert(pipe.type_, dist);

            for position in pipes_map.get(&pipe.type_).unwrap().iter() {
                let new_position = pipe.position + *position;
                if seen_pipes.contains(&new_position) {
                    continue;
                }

                if let Some(input_matrix_row) = input_matrix.get(new_position.y as usize) {
                    if let Some(input_matrix_char) = input_matrix_row.get(new_position.x as usize) {
                        bfs_queue.push_back((
                            Box::new(Pipe {
                                type_: *input_matrix_char,
                                position: new_position,
                            }),
                            dist + 1,
                        ));
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            }
        }
    }

    println!("seen_pipes: {:?}", seen_pipes);
    println!("distances: {:?}", distances);

    distances.values().max().unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::print_green;
    use std::path::Path;

    #[test]
    fn test_part_1() {
        let input = r#"
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"#
        .trim_start_matches('\n');

        let result = part_1(input);
        assert_eq!(result, 8);
    }

    #[test]
    fn run_part_1() {
        let input = include_str!("./input.txt");

        let result = part_1(input);

        let file_name = file!();
        let file_path = Path::new(file_name);
        let file_name = file_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split('.')
            .next()
            .unwrap();

        let mut dir_name: &str = "";

        if let Some(dir) = file_path.parent() {
            dir_name = dir.to_str().unwrap();
            dir_name = dir_name.split('/').last().unwrap();
        }

        print_green(&format!("{dir_name}, {file_name} result: {}", result));
    }
}
