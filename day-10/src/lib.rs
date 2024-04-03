use std::collections::VecDeque;

use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, field) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let looop = find_loop(&field);
    (looop.len() / 2).to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, mut field) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let looop = find_loop(&field);
    let (x, y) = looop[0];
    let max_x = field[0].len() - 1;
    let max_y = field.len() - 1;
    let north = if y > 0 {
        if let Some(p) = &field[y - 1][x] {
            match p {
                Pipe::NorthSouth | Pipe::SouthWest | Pipe::SouthEast => true,
                _ => {
                    false // not connected to Start
                }
            }
        } else {
            false
        }
    } else {
        false
    };
    let west = if x > 0 {
        if let Some(p) = &field[y][x - 1] {
            match p {
                Pipe::EastWest | Pipe::NorthEast | Pipe::SouthEast => true,
                _ => {
                    false // not connected to Start}
                }
            }
        } else {
            false
        }
    } else {
        false
    };
    let east = if x < max_x {
        if let Some(p) = &field[y][x + 1] {
            match p {
                Pipe::EastWest | Pipe::NorthWest | Pipe::SouthWest => true,
                _ => {
                    false // not connected to Start}
                }
            }
        } else {
            false
        }
    } else {
        false
    };
    let south = if y < max_y {
        if let Some(p) = &field[y + 1][x] {
            match p {
                Pipe::NorthSouth | Pipe::NorthWest | Pipe::NorthEast => true,
                _ => {
                    false // not connected to Start}
                }
            }
        } else {
            false
        }
    } else {
        false
    };
    let start_pipe = match (north, south, east, west) {
        (true, true, false, false) => Pipe::NorthSouth,
        (true, false, true, false) => Pipe::NorthEast,
        (true, false, false, true) => Pipe::NorthWest,
        (false, true, true, false) => Pipe::SouthEast,
        (false, true, false, true) => Pipe::SouthWest,
        (false, false, true, true) => Pipe::EastWest,
        _ => unreachable!("Valid start should be connected to exactly 2 other pipes"),
    };
    let _ = field
        .get_mut(y)
        .expect("Must be start")
        .get_mut(x)
        .expect("Must be start")
        .insert(start_pipe);

    field.iter_mut().enumerate().for_each(|(y, row)| {
        row.iter_mut().enumerate().for_each(|(x, e)| {
            if !looop.contains(&(x, y)) {
                e.take();
            }
        })
    });
    let insides = field
        .into_iter()
        .map(|row| {
            let (_, insides) = row.into_iter().fold(((false, false), 0), |acc, e| {
                let ((crossed_north, crossed_south), insides) = acc;
                if let Some(pipe) = e {
                    let (n, s) = evaluate(&pipe);
                    ((crossed_north ^ n, crossed_south ^ s), insides)
                } else if crossed_north && crossed_south {
                    // we've crossed an 'odd' number of loop lines and are thus inside the loop
                    ((crossed_north, crossed_south), insides + 1)
                } else {
                    ((crossed_north, crossed_south), insides)
                }
            });
            insides
        })
        .sum::<i32>();

    insides.to_string()
}

fn find_loop(field: &Vec<Vec<Option<Pipe>>>) -> Vec<(usize, usize)> {
    let max_x = field[0].len() - 1;
    let max_y = field.len() - 1;
    let mut queue = VecDeque::new();
    let mut looop = Vec::new();
    let start = field
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .position(|e| e == &Some(Pipe::Start))
                .map(|x| (x, y))
        })
        .expect("There's always a start!");

    looop.push(start);
    queue.push_back((start, 0_usize, Direction::North));

    while let Some(((x, y), step, direction)) = queue.pop_front() {
        if let Some(pipe) = &field[y][x] {
            match pipe {
                Pipe::Start => {
                    if step == 0 {
                        // we're starting!
                        // look north
                        if y > 0 {
                            if let Some(p) = &field[y - 1][x] {
                                match p {
                                    Pipe::NorthSouth | Pipe::SouthWest | Pipe::SouthEast => {
                                        let coords = (x, y - 1);
                                        looop.push(coords);
                                        queue.push_back((coords, 1, Direction::North));
                                        continue;
                                    }
                                    _ => { // not connected to Start}
                                    }
                                }
                            }
                        }
                        // look west
                        if x > 0 {
                            if let Some(p) = &field[y][x - 1] {
                                match p {
                                    Pipe::EastWest | Pipe::NorthEast | Pipe::SouthEast => {
                                        let coords = (x - 1, y);
                                        looop.push(coords);
                                        queue.push_back((coords, 1, Direction::West));
                                        continue;
                                    }
                                    _ => { // not connected to Start}
                                    }
                                }
                            }
                        }
                        // look east
                        if x < max_x {
                            if let Some(p) = &field[y][x + 1] {
                                match p {
                                    Pipe::EastWest | Pipe::NorthWest | Pipe::SouthWest => {
                                        let coords = (x + 1, y);
                                        looop.push(coords);
                                        queue.push_back((coords, 1, Direction::East));
                                        continue;
                                    }
                                    _ => { // not connected to Start}
                                    }
                                }
                            }
                        }
                        // look south
                        if y < max_y {
                            if let Some(p) = &field[y + 1][x] {
                                match p {
                                    Pipe::NorthSouth | Pipe::NorthWest | Pipe::NorthEast => {
                                        let coords = (x, y + 1);
                                        looop.push(coords);
                                        queue.push_back((coords, 1, Direction::South));
                                        continue;
                                    }
                                    _ => { // not connected to Start}
                                    }
                                }
                            }
                        }
                    } else {
                        // we're at the end!
                        break;
                    }
                }
                Pipe::NorthSouth => {
                    if y > 0 && direction == Direction::North {
                        let coords = (x, y - 1);
                        looop.push(coords);
                        queue.push_back((coords, step + 1, Direction::North));
                    }
                    if y < max_y && direction == Direction::South {
                        let coords = (x, y + 1);
                        looop.push(coords);
                        queue.push_back((coords, step + 1, Direction::South));
                    }
                }
                Pipe::EastWest => {
                    if x > 0 && direction == Direction::West {
                        let coords = (x - 1, y);
                        looop.push(coords);
                        queue.push_back((coords, step + 1, Direction::West));
                    }
                    if x < max_x && direction == Direction::East {
                        let coords = (x + 1, y);
                        looop.push(coords);
                        queue.push_back((coords, step + 1, Direction::East));
                    }
                }
                Pipe::NorthEast => {
                    if y > 0 && direction == Direction::West {
                        let coords = (x, y - 1);
                        looop.push(coords);
                        queue.push_back((coords, step + 1, Direction::North));
                    }
                    if x < max_x && direction == Direction::South {
                        let coords = (x + 1, y);
                        looop.push(coords);
                        queue.push_back((coords, step + 1, Direction::East));
                    }
                }
                Pipe::NorthWest => {
                    if y > 0 && direction == Direction::East {
                        let coords = (x, y - 1);
                        looop.push(coords);
                        queue.push_back((coords, step + 1, Direction::North));
                    }
                    if x > 0 && direction == Direction::South {
                        let coords = (x - 1, y);
                        looop.push(coords);
                        queue.push_back((coords, step + 1, Direction::West));
                    }
                }
                Pipe::SouthWest => {
                    if y < max_y && direction == Direction::East {
                        let coords = (x, y + 1);
                        looop.push(coords);
                        queue.push_back((coords, step + 1, Direction::South));
                    }
                    if x > 0 && direction == Direction::North {
                        let coords = (x - 1, y);
                        looop.push(coords);
                        queue.push_back((coords, step + 1, Direction::West));
                    }
                }
                Pipe::SouthEast => {
                    if y < max_y && direction == Direction::West {
                        let coords = (x, y + 1);
                        looop.push(coords);
                        queue.push_back((coords, step + 1, Direction::South));
                    }
                    if x < max_x && direction == Direction::North {
                        let coords = (x + 1, y);
                        looop.push(coords);
                        queue.push_back((coords, step + 1, Direction::East));
                    }
                }
            }
        }
    }

    looop.pop(); // remove extraneous start
    looop
}

fn evaluate(pipe: &Pipe) -> (bool, bool) {
    // (north, south)
    match pipe {
        Pipe::Start => unreachable!(), // Start is really NorthSouth in the input, kinda cheating, could've figured this out with code, but who cares?
        Pipe::NorthSouth => (true, true),
        Pipe::EastWest => (false, false),
        Pipe::NorthEast => (true, false),
        Pipe::NorthWest => (true, false),
        Pipe::SouthWest => (false, true),
        Pipe::SouthEast => (false, true),
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Pipe {
    Start,
    NorthSouth, // |
    EastWest,   // -
    NorthEast,  // L
    NorthWest,  // J
    SouthWest,  // 7
    SouthEast,  // F
}
#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

type Line<'a> = Vec<Option<Pipe>>;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = many1(parse_pipe)(input)?;
    Ok((input, line))
}

fn parse_pipe(input: &str) -> IResult<&str, Option<Pipe>> {
    let (input, c) = one_of("S|-LJ7F.")(input)?;
    let pipe = match c {
        'S' => Some(Pipe::Start),
        '|' => Some(Pipe::NorthSouth),
        '-' => Some(Pipe::EastWest),
        'L' => Some(Pipe::NorthEast),
        'J' => Some(Pipe::NorthWest),
        '7' => Some(Pipe::SouthWest),
        'F' => Some(Pipe::SouthEast),
        '.' => None,
        _ => unreachable!(),
    };
    Ok((input, pipe))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        let result = process_part1(input);
        assert_eq!(result, "4");
    }

    #[test]
    fn part1_2() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        let result = process_part1(input);
        assert_eq!(result, "8");
    }

    #[test]
    fn part2_1() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let result = process_part2(input);
        assert_eq!(result, "4");
    }

    #[test]
    fn part2_2() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let result = process_part2(input);
        assert_eq!(result, "8");
    }

    #[test]
    fn part2_3() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let result = process_part2(input);
        assert_eq!(result, "10");
    }
}
