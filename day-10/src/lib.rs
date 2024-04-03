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
    // let max_x = field[0].len() - 1;
    // let max_y = field.len() - 1;

    let north = matches!(
        y.checked_sub(1)
            .and_then(|y| field.get(y).and_then(|row| row.get(x))),
        Some(Some((_, true, _, _)))
    );
    let south = matches!(
        field.get(y + 1).and_then(|row| row.get(x)),
        Some(Some((true, _, _, _)))
    );

    let east = matches!(
        field.get(y).and_then(|row| row.get(x + 1)),
        Some(Some((_, _, _, true)))
    );
    let west = matches!(
        x.checked_sub(1)
            .and_then(|x| field.get(y).and_then(|row| row.get(x))),
        Some(Some((_, _, true, _)))
    );

    let start_pipe = (north, south, east, west);
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
                if let Some((n, s, _, _)) = e {
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

fn find_loop(field: &[Vec<Option<Pipe>>]) -> Vec<(usize, usize)> {
    let mut queue = VecDeque::new();
    let mut looop = Vec::new();
    let start = field
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .position(|e| e == &Some((true, true, true, true)))
                .map(|x| (x, y))
        })
        .expect("There's always a start!");

    looop.push(start);
    queue.push_back((start, 0_usize, Direction::Start));

    while let Some(((x, y), step, direction)) = queue.pop_front() {
        let (coords, dir) = match direction {
            Direction::Start => {
                if let Some(Some((_, true, _, _))) = y
                    .checked_sub(1)
                    .and_then(|y| field.get(y).and_then(|row| row.get(x)))
                {
                    ((x, y - 1), Direction::North)
                } else if let Some(Some((_, _, true, _))) = x
                    .checked_sub(1)
                    .and_then(|x| field.get(y).and_then(|row| row.get(x)))
                {
                    ((x - 1, y), Direction::West)
                } else if let Some(Some((_, _, _, true))) =
                    field.get(y).and_then(|row| row.get(x + 1))
                {
                    ((x + 1, y), Direction::East)
                } else if let Some(Some((true, _, _, _))) =
                    field.get(y + 1).and_then(|row| row.get(x))
                {
                    ((x, y + 1), Direction::South)
                } else {
                    unreachable!("Start must go somewhere")
                }
            }
            Direction::North => {
                if let Some(pipe) = field[y][x] {
                    match pipe {
                        (true, true, false, false) => ((x, y - 1), Direction::North),
                        (false, true, true, false) => ((x + 1, y), Direction::East),
                        (false, true, false, true) => ((x - 1, y), Direction::West),
                        (true, true, true, true) => {
                            break;
                        }
                        _ => unreachable!("Shouldn't be a dead end {:?}", pipe),
                    }
                } else {
                    unreachable!("Shouldn't get off track")
                }
            }
            Direction::South => {
                if let Some(pipe) = field[y][x] {
                    match pipe {
                        (true, true, false, false) => ((x, y + 1), Direction::South),
                        (true, false, true, false) => ((x + 1, y), Direction::East),
                        (true, false, false, true) => ((x - 1, y), Direction::West),
                        (true, true, true, true) => {
                            break;
                        }
                        _ => unreachable!("Shouldn't be a dead end {:?}", pipe),
                    }
                } else {
                    unreachable!("Shouldn't get off track")
                }
            }
            Direction::East => {
                if let Some(pipe) = field[y][x] {
                    match pipe {
                        (true, false, false, true) => ((x, y - 1), Direction::North),
                        (false, true, false, true) => ((x, y + 1), Direction::South),
                        (false, false, true, true) => ((x + 1, y), Direction::East),
                        (true, true, true, true) => {
                            break;
                        }
                        _ => unreachable!("Shouldn't be a dead end {:?}", pipe),
                    }
                } else {
                    unreachable!("Shouldn't get off track")
                }
            }
            Direction::West => {
                if let Some(pipe) = field[y][x] {
                    match pipe {
                        (true, false, true, false) => ((x, y - 1), Direction::North),
                        (false, true, true, false) => ((x, y + 1), Direction::South),
                        (false, false, true, true) => ((x - 1, y), Direction::West),
                        (true, true, true, true) => {
                            break;
                        }
                        _ => unreachable!("Shouldn't be a dead end {:?}", pipe),
                    }
                } else {
                    unreachable!("Shouldn't get off track")
                }
            }
        };
        looop.push(coords);
        queue.push_back((coords, step + 1, dir));
    }

    looop.pop(); // remove extraneous start
    looop
}

type Pipe = (bool, bool, bool, bool); // (north, south, east, west)

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Start,
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
        'S' => Some((true, true, true, true)),
        '|' => Some((true, true, false, false)),
        '-' => Some((false, false, true, true)),
        'L' => Some((true, false, true, false)),
        'J' => Some((true, false, false, true)),
        '7' => Some((false, true, false, true)),
        'F' => Some((false, true, true, false)),
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
