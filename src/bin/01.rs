use std::collections::HashMap;

enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Direction {
    fn next(&self, turn: &str) -> Direction {
        let right = "R" == turn;

        match *self {
            Direction::North => {
                if right {
                    Direction::East
                } else {
                    Direction::West
                }
            }
            Direction::South => {
                if right {
                    Direction::West
                } else {
                    Direction::East
                }
            }
            Direction::East => {
                if right {
                    Direction::South
                } else {
                    Direction::North
                }
            }
            Direction::West => {
                if right {
                    Direction::North
                } else {
                    Direction::South
                }
            }
        }
    }
}

impl Coord {
    fn walk(&self, facing: &Direction, distance: i32) -> Coord {
        match *facing {
            Direction::North => Coord {
                x: self.x,
                y: self.y + distance,
            },
            Direction::South => Coord {
                x: self.x,
                y: self.y - distance,
            },
            Direction::East => Coord {
                x: self.x + distance,
                y: self.y,
            },
            Direction::West => Coord {
                x: self.x - distance,
                y: self.y,
            },
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut pos = Coord { x: 0, y: 0 };
    let steps = input.trim().split(',').map(|x| x.trim().to_string());
    let mut direction = Direction::North;

    for step in steps {
        let (dir, dist) = step.split_at(1);
        let dist: i32 = dist.parse().unwrap();

        direction = direction.next(dir);
        pos = pos.walk(&direction, dist);
    }

    Some((pos.x).abs() + (pos.y).abs())
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut pos = Coord { x: 0, y: 0 };
    let mut pos_set: HashMap<Coord, i32> = HashMap::new();
    let steps = input.trim().split(',').map(|x| x.trim().to_string());
    let mut direction = Direction::North;
    let mut result_pos = Coord { x: 0, y: 0 };

    for step in steps {
        let (dir, dist) = step.split_at(1);
        let dist: i32 = dist.parse().unwrap();

        direction = direction.next(dir);
        pos = pos.walk(&direction, dist);

        match pos_set.get(&pos) {
            Some(_) => {
                if *pos_set.get(&pos).unwrap() == 2 {
                    result_pos = pos;
                    eprintln!("pos: {} {}", result_pos.x, result_pos.y);
                    break;
                } else {
                    pos_set.insert(pos, pos_set.get(&pos).unwrap() + 1);
                }
            }
            None => {
                pos_set.insert(pos, 1);
            }
        }
    }

    for pos in pos_set.iter() {
        eprintln!("pos: {} {}, times: {}", pos.0.x, pos.0.y, pos.1);
    }

    // let result = pos_set
    //    .iter()
    //    .find_map(|(key, &val)| if val == 2 { Some(key) } else { None });

    Some((result_pos.x).abs() + (result_pos.y).abs())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(8));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(4));
    }
}
