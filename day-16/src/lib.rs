use std::collections::HashSet;

use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, mut contraption) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let mut beams = Beams::default();
    beams.bounce(&mut contraption);
    contraption.count_energised().to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, mut contraption) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let beamses: Vec<Beams> = (0..contraption.width())
        .flat_map(|x| {
            [
                Beams::new(Beam::new(Direction::South, x, 0)),
                Beams::new(Beam::new(Direction::North, x, contraption.height() - 1)),
            ]
            .into_iter()
        })
        .chain((0..contraption.height()).flat_map(|y| {
            [
                Beams::new(Beam::new(Direction::East, 0, y)),
                Beams::new(Beam::new(Direction::West, contraption.width() - 1, y)),
            ]
            .into_iter()
        }))
        .collect();
    beamses
        .into_iter()
        .map(|mut beams| {
            contraption.reset();
            beams.bounce(&mut contraption);
            contraption.count_energised()
        })
        .max()
        .expect("Should be a biggest one!")
        .to_string()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    PositiveMirror,
    NegativeMirror,
    HorizontalSplitter,
    VerticalSplitter,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '/' => Self::PositiveMirror,
            '\\' => Self::NegativeMirror,
            '-' => Self::HorizontalSplitter,
            '|' => Self::VerticalSplitter,
            c => panic!("Unexpected character: {c}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Beam {
    direction: Direction,
    x: usize,
    y: usize,
}

impl Beam {
    fn new(direction: Direction, x: usize, y: usize) -> Self {
        Self { direction, x, y }
    }

    fn north(self) -> Self {
        Self {
            direction: Direction::North,
            x: self.x,
            y: self.y - 1,
        }
    }

    fn south(self) -> Self {
        Self {
            direction: Direction::South,
            x: self.x,
            y: self.y + 1,
        }
    }

    fn east(self) -> Self {
        Self {
            direction: Direction::East,
            x: self.x + 1,
            y: self.y,
        }
    }

    fn west(self) -> Self {
        Self {
            direction: Direction::West,
            x: self.x - 1,
            y: self.y,
        }
    }

    fn step(self, contraption: &mut Contraption) -> [Option<Beam>; 2] {
        let (x, y) = (self.x, self.y);
        let direction = self.direction;
        contraption.energised[y][x] = true;
        let tile = contraption.grid[y][x];
        match tile {
            Tile::PositiveMirror => match direction {
                Direction::North if x < contraption.width() - 1 => [Some(self.east()), None],
                Direction::South if x > 0 => [Some(self.west()), None],
                Direction::East if y > 0 => [Some(self.north()), None],
                Direction::West if y < contraption.height() - 1 => [Some(self.south()), None],
                _ => [None, None],
            },
            Tile::NegativeMirror => match direction {
                Direction::North if x > 0 => [Some(self.west()), None],
                Direction::South if x < contraption.width() - 1 => [Some(self.east()), None],
                Direction::East if y < contraption.height() - 1 => [Some(self.south()), None],
                Direction::West if y > 0 => [Some(self.north()), None],
                _ => [None, None],
            },
            Tile::HorizontalSplitter
                if matches!(direction, Direction::North | Direction::South) =>
            {
                [
                    Some(Self {
                        direction: Direction::East,
                        x,
                        y,
                    }),
                    Some(Self {
                        direction: Direction::West,
                        x,
                        y,
                    }),
                ]
            }
            Tile::VerticalSplitter if matches!(direction, Direction::East | Direction::West) => [
                Some(Self {
                    direction: Direction::North,
                    x,
                    y,
                }),
                Some(Self {
                    direction: Direction::South,
                    x,
                    y,
                }),
            ],
            // all other cases: treat as empty
            _ => match direction {
                Direction::North if y > 0 => [Some(self.north()), None],
                Direction::South if y < contraption.height() - 1 => [Some(self.south()), None],
                Direction::East if x < contraption.width() - 1 => [Some(self.east()), None],
                Direction::West if x > 0 => [Some(self.west()), None],
                _ => [None, None],
            },
        }
    }
}

impl Default for Beam {
    fn default() -> Self {
        Self {
            direction: Direction::East,
            x: 0,
            y: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct Beams {
    beams: Vec<Beam>,
    cache: HashSet<Beam>,
}

impl Beams {
    fn new(start_beam: Beam) -> Self {
        Self {
            beams: vec![start_beam],
            cache: HashSet::new(),
        }
    }

    fn bounce(&mut self, contraption: &mut Contraption) {
        while !self.beams.is_empty() {
            let mut beams = self.beams.clone();
            beams = beams
                .into_iter()
                .flat_map(|beam| {
                    beam.step(contraption)
                        .into_iter()
                        .flatten()
                        .filter(|b| !self.cache.contains(b))
                })
                .collect();
            self.update_cache(&beams);
            self.beams = beams;
        }
    }

    fn update_cache(&mut self, beams: &[Beam]) {
        beams.iter().for_each(|b| {
            self.cache.insert(*b);
        })
    }
}

#[derive(Debug)]
struct Contraption {
    grid: Vec<Vec<Tile>>,
    energised: Vec<Vec<bool>>,
}

impl Default for Beams {
    fn default() -> Self {
        Self {
            beams: vec![Beam::default()],
            cache: Default::default(),
        }
    }
}

impl Contraption {
    fn count_energised(&self) -> usize {
        self.energised
            .iter()
            .map(|row| row.iter().filter(|&&t| t).count())
            .sum()
    }

    fn reset(&mut self) {
        self.energised.iter_mut().for_each(|row| row.fill(false))
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }
}

impl From<Vec<Vec<Tile>>> for Contraption {
    fn from(value: Vec<Vec<Tile>>) -> Self {
        let energised = vec![vec![false; value[0].len()]; value.len()];
        Self {
            grid: value,
            energised,
        }
    }
}

type Line<'a> = Vec<Tile>;

fn parse_input(input: &str) -> IResult<&str, Contraption> {
    let (input, contraption) = separated_list1(newline, parse_line)(input)?;
    Ok((input, contraption.into()))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = many1(parse_tile)(input)?;
    Ok((input, line))
}

fn parse_tile(input: &str) -> IResult<&str, Tile> {
    let (input, tile) = one_of(r"./\-|")(input)?;
    Ok((input, tile.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        let result = process_part1(input);
        assert_eq!(result, "46");
    }

    #[test]
    fn part2() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        let result = process_part2(input);
        assert_eq!(result, "51");
    }
}
