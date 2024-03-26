use std::collections::HashMap;

use nom::{
    character::complete::newline,
    character::complete::none_of,
    multi::{many1, separated_list1},
    AsChar, IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, schematic) = parse_input(input).unwrap();
    let ymax = schematic.len() - 1;
    let xmax = schematic.first().unwrap().len() - 1;
    let mut total = 0;
    for y in 0..=ymax {
        let mut x = 0;
        while x <= xmax {
            if let Pos::Num(_) = schematic[y][x] {
                let xstart = x.saturating_sub(1);
                let ystart = y.saturating_sub(1);
                let mut num = 0;
                while let Pos::Num(n) = schematic[y][x] {
                    num *= 10;
                    num += n;
                    x += 1;
                    if x > xmax {
                        break;
                    }
                }
                let xend = xmax.min(x);
                let yend = ymax.min(y + 1);
                let counts = (ystart..=yend).any(|check_y| {
                    (xstart..=xend).any(|check_x| schematic[check_y][check_x].is_sym())
                });
                if counts {
                    total += num;
                }
            }
            x += 1;
        }
    }
    total.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, mut schematic) = parse_input(input).unwrap();
    schematic.iter_mut().enumerate().for_each(|(y, row)| {
        row.iter_mut().enumerate().for_each(|(x, p)| {
            if let Pos::Gear(0, 0) = p {
                *p = Pos::Gear(x, y)
            }
        })
    });
    let mut gears = HashMap::new();
    let ymax = schematic.len() - 1;
    let xmax = schematic.first().unwrap().len() - 1;
    for y in 0..=ymax {
        let mut x = 0;
        while x <= xmax {
            if let Pos::Num(_) = schematic[y][x] {
                let xstart = x.saturating_sub(1);
                let ystart = y.saturating_sub(1);
                let mut num = 0;
                while let Pos::Num(n) = schematic[y][x] {
                    num *= 10;
                    num += n;
                    x += 1;
                    if x > xmax {
                        break;
                    }
                }
                let xend = xmax.min(x);
                let yend = ymax.min(y + 1);
                (ystart..=yend).for_each(|check_y| {
                    (xstart..=xend).for_each(|check_x| {
                        let gear = &schematic[check_y][check_x];
                        if gear.is_gear() {
                            gears.entry(gear).or_insert_with(Vec::new).push(num);
                        }
                    })
                });
            }
            x += 1;
        }
    }
    gears
        .values()
        .filter_map(|v| {
            if v.len() == 2 {
                Some(v.iter().product::<u32>())
            } else {
                None
            }
        })
        .sum::<u32>()
        .to_string()
}

type Line<'a> = Vec<Pos>;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = many1(parse_pos)(input)?;
    Ok((input, line))
}

fn parse_pos(input: &str) -> IResult<&str, Pos> {
    let (input, c) = none_of("\n")(input)?;
    let p = match c {
        '.' => Pos::Dot,
        '*' => Pos::Gear(0, 0),
        x => {
            if x.is_dec_digit() {
                Pos::Num(x.to_digit(10).unwrap())
            } else {
                Pos::Sym
            }
        }
    };
    Ok((input, p))
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Pos {
    Dot,
    Num(u32),
    Sym,
    Gear(usize, usize), // x, y
}

impl Pos {
    fn is_sym(&self) -> bool {
        matches!(self, Self::Gear(_, _) | Self::Sym)
    }

    fn is_gear(&self) -> bool {
        matches!(self, Self::Gear(_, _))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = process_part1(input);
        assert_eq!(result, "4361");
    }

    #[test]
    fn part2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = process_part2(input);
        assert_eq!(result, "467835");
    }
}
