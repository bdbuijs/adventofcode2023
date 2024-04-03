use itertools::Itertools;

use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, space) = parse_input(input).unwrap();
    assert!(input.is_empty());
    process_space(space, 2)
}

pub fn process_part2(input: &str) -> String {
    let (input, space) = parse_input(input).unwrap();
    assert!(input.is_empty());
    process_space(space, 1_000_000)
}

fn process_space(space: Vec<Vec<Space>>, growth_factor: usize) -> String {
    let empty_rows = space
        .iter()
        .enumerate()
        .filter_map(|(i, row)| {
            if row.iter().all(|s| s.is_empty()) {
                Some(i)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let empty_columns = (0..(space[0].len()))
        .filter(|&n| space.iter().all(|row| row[n].is_empty()))
        .collect::<Vec<_>>();
    let galaxies: Vec<(usize, usize)> = space
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(
                move |(x, g)| {
                    if !g.is_empty() {
                        Some((x, y))
                    } else {
                        None
                    }
                },
            )
        })
        .map(|(x, y)| {
            let x_growth =
                (growth_factor - 1) * empty_columns.iter().take_while(|&&n| n < x).count();
            let y_growth = (growth_factor - 1) * empty_rows.iter().take_while(|&&n| n < y).count();
            (x + x_growth, y + y_growth)
        })
        .collect::<Vec<_>>();
    galaxies
        .iter()
        .combinations(2)
        .map(|c| c[0].0.abs_diff(c[1].0) + c[0].1.abs_diff(c[1].1))
        .sum::<usize>()
        .to_string()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Space {
    Galaxy,
    Empty,
}

impl Space {
    fn is_empty(&self) -> bool {
        match self {
            Space::Galaxy => false,
            Space::Empty => true,
        }
    }
}

type Line = Vec<Space>;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = many1(parse_space)(input)?;
    Ok((input, line))
}

fn parse_space(input: &str) -> IResult<&str, Space> {
    let (input, c) = one_of("#.")(input)?;
    let space = match c {
        '#' => Space::Galaxy,
        '.' => Space::Empty,
        _ => unreachable!(),
    };
    Ok((input, space))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let result = process_part1(input);
        assert_eq!(result, "374");
    }

    #[test]
    fn part2_1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let (input, space) = parse_input(input).unwrap();
        assert!(input.is_empty());
        let result = process_space(space, 10);
        assert_eq!(result, "1030");
    }

    #[test]
    fn part2_2() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let (input, space) = parse_input(input).unwrap();
        assert!(input.is_empty());
        let result = process_space(space, 100);
        assert_eq!(result, "8410");
    }
}
