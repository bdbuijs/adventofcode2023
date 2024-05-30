use std::collections::{BinaryHeap, HashMap};

use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, city) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let (target_x, target_y) = (city[0].len() - 1, city.len() - 1);
    let target = Coord::new(target_x, target_y);
    let shortest = shortest_path(target, &city, neighbours::<1, 3>);
    shortest.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, city) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let (target_x, target_y) = (city[0].len() - 1, city.len() - 1);
    let target = Coord::new(target_x, target_y);
    let shortest = shortest_path(target, &city, neighbours::<4, 10>);
    shortest.to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn start() -> Self {
        Self { x: 0, y: 0 }
    }

    #[inline(always)]
    fn tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Node {
    coord: Coord,
    direction: Direction,
    steps: usize,
}

impl Node {
    fn new(coord: Coord, direction: Direction, steps: usize) -> Self {
        Self {
            coord,
            direction,
            steps,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Step {
    value: usize,
    node: Node,
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .value
            .cmp(&self.value)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Returns Vec of all nodes that are valid neighbours of the given node in the given city.
/// MIN_STEPS and MAX_STEPS are the constraints given by the problem
fn neighbours<const MIN_STEPS: usize, const MAX_STEPS: usize>(
    node: Node,
    height: usize,
    width: usize,
) -> Vec<Node> {
    let (x, y) = node.coord.tuple();
    let directions_and_coords = [
        (Direction::West, x > 0, Coord::new(x - 1, y)),
        (Direction::North, y > 0, Coord::new(x, y - 1)),
        (Direction::East, x < width, Coord::new(x + 1, y)),
        (Direction::South, y < height, Coord::new(x, y + 1)),
    ];
    directions_and_coords
        .into_iter()
        .filter_map(|(direction, valid, new_coord)| {
            if !valid || direction == node.direction.opposite() {
                // we cannot go off-grid or in the opposite direction
                None
            } else if direction != node.direction && node.steps >= MIN_STEPS {
                // we cannot make a turn before taking MIN_STEPS
                Some(Node::new(new_coord, direction, 1))
            } else if direction == node.direction && node.steps < MAX_STEPS {
                // we cannot go straight after taking MAX_STEPS
                Some(Node::new(new_coord, direction, node.steps + 1))
            } else {
                None
            }
        })
        .collect()
}

fn shortest_path<F>(target: Coord, city: &[Vec<u8>], neighbour_func: F) -> usize
where
    F: Fn(Node, usize, usize) -> Vec<Node>,
{
    let height = city.len() - 1;
    let width = city[0].len() - 1;

    let start_east = Node::new(Coord::start(), Direction::East, 0);
    let start_south = Node::new(Coord::start(), Direction::South, 0);

    let mut distances = HashMap::new();
    distances.insert(start_east.clone(), 0_usize);
    distances.insert(start_south.clone(), 0);

    let mut queue = BinaryHeap::new();
    queue.push(Step {
        value: 0,
        node: start_east,
    });
    queue.push(Step {
        value: 0,
        node: start_south,
    });

    while let Some(Step { value, node }) = queue.pop() {
        if node.coord == target {
            return value;
        }

        for neighbour in neighbour_func(node, height, width).into_iter() {
            let (x, y) = neighbour.coord.tuple();
            let new_value = value + city[y][x] as usize;
            if let Some(&shortest_so_far) = distances.get(&neighbour) {
                if new_value >= shortest_so_far {
                    continue;
                }
            }
            distances.insert(neighbour.clone(), new_value);
            queue.push(Step {
                value: new_value,
                node: neighbour,
            });
        }
    }

    unreachable!("Cannot find a path to the target!");
}

// fn shortest_path(target: (usize, usize), city: &City) -> usize {
//     let height = city.len() - 1;
//     let width = city[0].len() - 1;
//     let mut to_visit = BTreeSet::new();
//     to_visit.insert(Node::start());
//     let mut visited = vec![vec![false; width + 1]; height + 1];
//     // let mut id = 1_usize;
//     while let Some(Node {
//         value,
//         // id: _,
//         location: (x, y),
//         path_to,
//     }) = to_visit.pop_first()
//     {
//         if x + y > 3 {
//             break;
//         }
//         dbg!(x + y);
//         // have we made it yet?
//         if (x, y) == target {
//             return value;
//         }
//         // id += 1;
//         // look North
//         if y > 0
//             && !visited[y - 1][x]
//             && !matches!(path_to, | Direction::South(steps) | Direction::North(steps) if steps > 2)
//         {
//             let new_path = match path_to {
//                 Direction::North(steps) => Direction::North(steps + 1),
//                 _ => Direction::North(1),
//             };
//             to_visit.insert(Node {
//                 value: value + city[y - 1][x] as usize,
//                 // id,
//                 location: (x, y - 1),
//                 path_to: new_path,
//             });
//             // id += 1;
//         }
//         // look South
//         if y < height
//             && !visited[y + 1][x]
//             && !matches!(path_to, | Direction::North(steps) | Direction::South(steps) if steps > 2)
//         {
//             let new_path = match path_to {
//                 Direction::South(steps) => Direction::South(steps + 1),
//                 _ => Direction::South(1),
//             };
//             to_visit.insert(Node {
//                 value: value + city[y + 1][x] as usize,
//                 // id,
//                 location: (x, y + 1),
//                 path_to: new_path,
//             });
//             // id += 1;
//         }
//         // look East
//         if x < width
//             && !visited[y][x + 1]
//             && !matches!(path_to, | Direction::West(steps) | Direction::East(steps) if steps > 2)
//         {
//             let new_path = match path_to {
//                 Direction::East(steps) => Direction::East(steps + 1),
//                 _ => Direction::East(1),
//             };
//             to_visit.insert(Node {
//                 value: value + city[y][x + 1] as usize,
//                 // id,
//                 location: (x + 1, y),
//                 path_to: new_path,
//             });
//             // id += 1;
//         }
//         // look West
//         if x > 0
//             && !visited[y][x - 1]
//             && !matches!(path_to, | Direction::East(steps) | Direction::West(steps) if steps > 2)
//         {
//             let new_path = match path_to {
//                 Direction::West(steps) => Direction::West(steps + 1),
//                 _ => Direction::West(1),
//             };
//             to_visit.insert(Node {
//                 value: value + city[y][x - 1] as usize,
//                 // id,
//                 location: (x - 1, y),
//                 path_to: new_path,
//             });
//             // id += 1;
//         }
//         visited[y][x] = true;
//         dbg!(&to_visit);
//     }

//     unreachable!("Cannot find a path to the target!");
// }

// #[derive(Debug, PartialEq, Eq)]
// enum Direction {
//     North(u8),
//     South(u8),
//     East(u8),
//     West(u8),
// }

// impl Direction {
//     fn len(&self) -> u8 {
//         match self {
//             Direction::North(steps) => *steps,
//             Direction::South(steps) => *steps,
//             Direction::East(steps) => *steps,
//             Direction::West(steps) => *steps,
//         }
//     }
// }

// impl PartialOrd for Direction {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         other.len().partial_cmp(&self.len())
//     }
// }

// impl Ord for Direction {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.partial_cmp(other).unwrap()
//     }
// }

// #[derive(PartialEq, Eq)]
// struct Node {
//     value: usize,
//     // id: usize,
//     location: (usize, usize),
//     path_to: Direction,
// }

// impl Node {
//     fn start() -> Self {
//         Self {
//             value: 0,
//             location: (0, 0),
//             path_to: Direction::East(0),
//             // id: 0,
//         }
//     }
// }

// impl std::fmt::Debug for Node {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str(format!("{} - {:?}", self.value, self.location).as_str())
//     }
// }

// impl PartialOrd for Node {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         match self.value.partial_cmp(&other.value) {
//             Some(core::cmp::Ordering::Equal) => {}
//             ord => return ord,
//         }
//         let manhattan = self.location.0 + self.location.1;
//         let other_manhattan = other.location.0 + other.location.1;
//         match manhattan.partial_cmp(&other_manhattan) {
//             Some(core::cmp::Ordering::Equal) => {}
//             ord => return ord,
//         }
//         match self.path_to.partial_cmp(&other.path_to) {
//             Some(core::cmp::Ordering::Equal) => {}
//             ord => return ord,
//         }
//         self.location.partial_cmp(&other.location)
//     }
// }

// impl Ord for Node {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.partial_cmp(other)
//             .expect("Ordering should already be complete")
//     }
// }

type Line<'a> = Vec<u8>;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = many1(parse_digit)(input)?;
    Ok((input, line))
}

fn parse_digit(input: &str) -> IResult<&str, u8> {
    let (input, digit) = one_of("123456789")(input)?;
    let digit = digit.to_digit(10).expect("Definitely a digit") as u8;
    Ok((input, digit))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let result = process_part1(input);
        assert_eq!(result, "102");
    }

    #[test]
    fn part2() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let result = process_part2(input);
        assert_eq!(result, "94");
    }
}
