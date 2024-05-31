use std::str::FromStr;

use nom::{
    bytes::complete::take_while_m_n,
    character::complete::{char as nomchar, digit1, newline, one_of, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, trenches) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let lagoon = calculate_area(trenches);

    lagoon.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, mut trenches) = parse_input(input).unwrap();
    assert!(input.is_empty());
    trenches.iter_mut().for_each(|t| *t = t.correct());
    let lagoon = calculate_area(trenches);

    lagoon.to_string()
}

fn calculate_area(trenches: Vec<Trench>) -> u64 {
    let (mut x, mut y) = (0_i64, 0_i64);
    let mut points = vec![(x, y)];
    let mut perimeter = 0;
    for trench in trenches {
        match trench.direction {
            Direction::Up => {
                y -= trench.length as i64;
            }
            Direction::Down => {
                y += trench.length as i64;
            }
            Direction::Left => {
                x -= trench.length as i64;
            }
            Direction::Right => {
                x += trench.length as i64;
            }
        }
        points.push((x, y));
        perimeter += trench.length;
    }
    assert_eq!((x, y), (0, 0));
    points
        .iter()
        .zip(points.iter().skip(1))
        .map(|(&(x1, y1), &(x2, y2))| (y1 + y2) * (x1 - x2))
        .sum::<i64>()
        .unsigned_abs()
        / 2
        + (perimeter / 2 + 1) as u64
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            c => panic!("Invalid direction: {}", c),
        }
    }
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            3 => Self::Up,
            1 => Self::Down,
            2 => Self::Left,
            0 => Self::Right,
            b => panic!("Invalid byte value: {}", b),
        }
    }
}

#[derive(Debug)]
struct Trench<'a> {
    direction: Direction,
    length: usize,
    hex: &'a str,
}

impl<'a> Trench<'a> {
    fn new(direction: Direction, length: usize, hex: &'a str) -> Self {
        Self {
            direction,
            length,
            hex,
        }
    }

    fn correct(&self) -> Self {
        self.hex.into()
    }
}

impl<'a> From<&str> for Trench<'a> {
    fn from(value: &str) -> Self {
        assert_eq!(
            value.len(),
            6,
            "Invalid length of hex string ({}). Input was: {}",
            value.len(),
            value
        );
        let length = u32::from_str_radix(&value[..5], 16)
            .unwrap_or_else(|_| panic!("Invalid hex for length: {}", &value[..5]))
            as usize;
        let direction = u8::from_str_radix(&value[5..6], 16)
            .unwrap_or_else(|_| panic!("Invalid hex byte for g: {}", &value[5..6]))
            .into();
        let hex = "";
        Self {
            direction,
            length,
            hex,
        }
    }
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Trench>> {
    let (input, lines) = separated_list1(newline, parse_trench)(input)?;
    Ok((input, lines))
}

fn parse_trench(input: &str) -> IResult<&str, Trench> {
    let (input, direction) = terminated(parse_direction, space1)(input)?;
    let (input, length) = terminated(parse_usize, space1)(input)?;
    let (input, rgb) = nom::sequence::delimited(
        nomchar('('),
        preceded(nomchar('#'), take_while_m_n(6, 6, is_hex_digit)),
        nomchar(')'),
    )(input)?;
    let trench = Trench::new(direction, length, rgb.into());
    Ok((input, trench))
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (input, c) = one_of("UDLR")(input)?;
    Ok((input, c.into()))
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, FromStr::from_str)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        let result = process_part1(input);
        assert_eq!(result, "62");
    }

    #[test]
    fn part2() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        let result = process_part2(input);
        assert_eq!(result, "952408144115");
    }
}
