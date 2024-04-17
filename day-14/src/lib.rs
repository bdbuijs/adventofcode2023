use std::collections::HashMap;

use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, mut platform) = parse_input(input).unwrap();
    assert!(input.is_empty());
    tilt_north(&mut platform);
    platform
        .iter()
        .rev()
        .enumerate()
        .map(|(row_i, row)| {
            row.iter()
                .filter(|space| space == &&Space::RoundedRock)
                .count()
                * (row_i + 1)
        })
        .sum::<usize>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, mut platform) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let target = 1_000_000_000;
    let mut map = HashMap::new();
    let mut loop_length = 0;
    let mut loops_ran = 0;
    for i in 1_usize..target {
        if let Some(&step) = map.get(&platform) {
            loop_length = i - step;
            loops_ran = i;
            break;
        }
        map.insert(platform.clone(), i);
        cycle(&mut platform);
    }
    let remaining = target - loops_ran;
    let cycles_left = remaining % loop_length;
    for _ in 0..(cycles_left + 1) {
        cycle(&mut platform);
    }
    platform
        .iter()
        .rev()
        .enumerate()
        .map(|(row_i, row)| {
            row.iter()
                .filter(|space| space == &&Space::RoundedRock)
                .count()
                * (row_i + 1)
        })
        .sum::<usize>()
        .to_string()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Space {
    RoundedRock,    // O
    CubeShapedRock, // #
    Empty,          // .
}

type Field = Vec<Vec<Space>>;

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            'O' => Self::RoundedRock,
            '#' => Self::CubeShapedRock,
            '.' => Self::Empty,
            c => panic!("Unexpected character: {c}"),
        }
    }
}

fn cycle(platform: &mut Field) {
    tilt_north(platform);
    tilt_west(platform);
    tilt_south(platform);
    tilt_east(platform);
}

fn tilt_north(platform: &mut Field) {
    let height = platform.len();
    let width = platform
        .first()
        .expect("Platform must have dimensions")
        .len();
    (0..height).for_each(|y| {
        (0..width).for_each(|x| {
            if platform[y][x] == Space::RoundedRock {
                roll_north(platform, x, y);
            }
        })
    })
}

fn tilt_south(platform: &mut Field) {
    let height = platform.len();
    let width = platform
        .first()
        .expect("Platform must have dimensions")
        .len();
    (0..height).rev().for_each(|y| {
        (0..width).for_each(|x| {
            if platform[y][x] == Space::RoundedRock {
                roll_south(platform, x, y);
            }
        })
    })
}

fn tilt_east(platform: &mut Field) {
    let height = platform.len();
    let width = platform
        .first()
        .expect("Platform must have dimensions")
        .len();
    (0..width).rev().for_each(|x| {
        (0..height).for_each(|y| {
            if platform[y][x] == Space::RoundedRock {
                roll_east(platform, x, y);
            }
        })
    })
}

fn tilt_west(platform: &mut Field) {
    let height = platform.len();
    let width = platform
        .first()
        .expect("Platform must have dimensions")
        .len();
    (0..width).for_each(|x| {
        (0..height).for_each(|y| {
            if platform[y][x] == Space::RoundedRock {
                roll_west(platform, x, y);
            }
        })
    })
}

fn roll_north(platform: &mut Field, x: usize, y: usize) {
    let mut y = y;
    platform[y][x] = Space::Empty;
    while y > 0 && matches!(platform[y - 1][x], Space::Empty) {
        y -= 1
    }
    platform[y][x] = Space::RoundedRock;
}

fn roll_south(platform: &mut Field, x: usize, y: usize) {
    let mut y = y;
    let height = platform.len();
    platform[y][x] = Space::Empty;
    while y < height - 1 && matches!(platform[y + 1][x], Space::Empty) {
        y += 1
    }
    platform[y][x] = Space::RoundedRock;
}

fn roll_east(platform: &mut Field, x: usize, y: usize) {
    let mut x = x;
    let width = platform[0].len();
    platform[y][x] = Space::Empty;
    while x < width - 1 && matches!(platform[y][x + 1], Space::Empty) {
        x += 1
    }
    platform[y][x] = Space::RoundedRock;
}

fn roll_west(platform: &mut Field, x: usize, y: usize) {
    let mut x = x;
    platform[y][x] = Space::Empty;
    while x > 0 && matches!(platform[y][x - 1], Space::Empty) {
        x -= 1
    }
    platform[y][x] = Space::RoundedRock;
}

type Line<'a> = Vec<Space>;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = many1(parse_space)(input)?;
    Ok((input, line))
}

fn parse_space(input: &str) -> IResult<&str, Space> {
    let (input, space) = one_of("O#.")(input)?;
    Ok((input, space.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let result = process_part1(input);
        assert_eq!(result, "136");
    }

    #[test]
    fn test_tilt() {
        let mut before_tilt = parse_input(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        )
        .unwrap()
        .1;
        tilt_north(&mut before_tilt);

        let after_tilt = parse_input(
            "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....",
        )
        .unwrap()
        .1;

        assert_eq!(before_tilt, after_tilt);
    }

    #[test]
    fn test_cycle() {
        let mut platform = parse_input(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        )
        .unwrap()
        .1;
        cycle(&mut platform);

        let after_one = parse_input(
            ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....",
        )
        .unwrap()
        .1;
        assert_eq!(platform, after_one);

        cycle(&mut platform);

        let after_two = parse_input(
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O",
        )
        .unwrap()
        .1;

        assert_eq!(platform, after_two);

        cycle(&mut platform);

        let after_three = parse_input(
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O",
        )
        .unwrap()
        .1;

        assert_eq!(platform, after_three);
    }

    #[test]
    fn part2() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let result = process_part2(input);
        assert_eq!(result, "64");
    }
}
