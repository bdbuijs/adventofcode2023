use nom::{
    bytes::complete::tag,
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, patterns) = parse_input(input).unwrap();
    assert!(input.is_empty());
    patterns
        .into_iter()
        .map(|p| p.score_part_1())
        .sum::<usize>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, patterns) = parse_input(input).unwrap();
    assert!(input.is_empty());
    patterns
        .into_iter()
        .map(|p| p.score_part_2())
        .sum::<usize>()
        .to_string()
}

#[derive(Debug, PartialEq, Eq)]
enum Location {
    Ash,
    Rock,
}

impl From<char> for Location {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ash,
            '#' => Self::Rock,
            c => panic!("Invalid character: {}", c),
        }
    }
}

#[derive(Debug)]
struct Pattern {
    field: Vec<Vec<Location>>,
}

impl Pattern {
    fn score_part_1(&self) -> usize {
        let mut total = 0;
        // check top/bottom reflection (horizontal mirror)
        if let Err(n) = self
            .field
            .windows(2)
            .enumerate()
            .try_fold(0, |_, (i, win)| {
                if win[0] == win[1] {
                    // start of reflection, maybe? Check the rest
                    if self
                        .field
                        .iter()
                        .skip(i + 1)
                        .zip(self.field.iter().rev().skip(self.field.len() - i - 1))
                        .all(|(l, r)| l == r)
                    {
                        Err(i + 1)
                    } else {
                        Ok(0)
                    }
                } else {
                    Ok(0)
                }
            })
        {
            total += n * 100;
        }
        // check left/right reflection (vertical mirror)
        else if let Err(n) = (0..(self.width() - 1)).try_fold(0, |_, col| {
            if self.field.iter().all(|row| row[col] == row[col + 1]) {
                // two columns match, does the rest?
                if (0..col)
                    .rev()
                    .zip((col + 2)..self.width())
                    .all(|(l, r)| self.field.iter().all(|row| row[l] == row[r]))
                {
                    Err(col + 1)
                } else {
                    Ok(0)
                }
            } else {
                Ok(0)
            }
        }) {
            total += n;
        }

        total
    }

    fn score_part_2(&self) -> usize {
        let mut total = 0;
        // check top/bottom reflection (horizontal mirror)
        if let Err(n) = (0..(self.height() - 1)).try_fold(0_usize, |_, row| {
            if self.match_rows(row, row + 1) {
                // we've matched without a smudge, see if the rest matches with one smudge
                if (0..row)
                    .rev()
                    .zip((row + 2)..self.height())
                    .map(|(first, second)| self.match_rows_with_smudge(first, second))
                    .sum::<usize>()
                    == 1
                {
                    Err(row + 1)
                } else {
                    Ok(0)
                }
            } else if self.match_rows_with_smudge(row, row + 1) == 1 {
                // we've matched with a smudge, see if the rest matches without a smudge
                if (0..row)
                    .rev()
                    .zip((row + 2)..self.height())
                    .all(|(first, second)| self.match_rows(first, second))
                {
                    Err(row + 1)
                } else {
                    Ok(0)
                }
            } else {
                Ok(0)
            }
        }) {
            total += n * 100;
        }
        // check left/right reflection (vertical mirror)
        else if let Err(n) = (0..(self.width() - 1)).try_fold(0, |_, col| {
            if self.match_columns(col, col + 1) {
                // we've matched without a smudge, see if the rest matches with one smudge
                if (0..col)
                    .rev()
                    .zip((col + 2)..self.width())
                    .map(|(first, second)| self.match_columns_with_smudge(first, second))
                    .sum::<usize>()
                    == 1
                {
                    Err(col + 1)
                } else {
                    Ok(0)
                }
            } else if self.match_columns_with_smudge(col, col + 1) == 1 {
                // we've matched with a smudge, see if the rest matches without a smudge
                if (0..col)
                    .rev()
                    .zip((col + 2)..self.width())
                    .all(|(first, second)| self.match_columns(first, second))
                {
                    Err(col + 1)
                } else {
                    Ok(0)
                }
            } else {
                Ok(0)
            }
        }) {
            total += n;
        }

        total
    }

    fn height(&self) -> usize {
        self.field.len()
    }

    fn width(&self) -> usize {
        self.field
            .first()
            .expect("Field must have dimensions!")
            .len()
    }

    fn match_rows(&self, first: usize, second: usize) -> bool {
        assert!(
            first < self.field.len() && second < self.field.len(),
            "Rows to check out of bounds, the len was {} and the rows were {first} and {second}.",
            self.field.len()
        );
        self.field[first] == self.field[second]
    }

    fn match_rows_with_smudge(&self, first: usize, second: usize) -> usize {
        assert!(
            first < self.field.len() && second < self.field.len(),
            "Rows to check out of bounds, the len was {} and the rows were {first} and {second}.",
            self.field.len()
        );
        let mismatches = self.field[first]
            .iter()
            .zip(self.field[second].iter())
            .fold(0_usize, |acc, (l, r)| if l == r { acc } else { acc + 1 });
        mismatches
    }

    fn match_columns(&self, first: usize, second: usize) -> bool {
        assert!(
            first < self.field[0].len() && second < self.field[0].len(),
            "Columns to check out of bounds, the len was {} and the rows were {first} and {second}.",
            self.field.len()
        );
        self.field.iter().all(|row| row[first] == row[second])
    }

    fn match_columns_with_smudge(&self, first: usize, second: usize) -> usize {
        assert!(
            first < self.field[0].len() && second < self.field[0].len(),
            "Columns to check out of bounds, the len was {} and the rows were {first} and {second}.",
            self.field.len()
        );
        self.field
            .iter()
            .map(|row| if row[first] == row[second] { 0 } else { 1 })
            .sum::<usize>()
    }
}

impl From<Vec<Vec<Location>>> for Pattern {
    fn from(value: Vec<Vec<Location>>) -> Self {
        Self { field: value }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Pattern>> {
    let (input, patterns) = separated_list1(tag("\n\n"), parse_pattern)(input)?;
    Ok((input, patterns))
}

fn parse_pattern(input: &str) -> IResult<&str, Pattern> {
    let (input, pattern) = separated_list1(newline, many1(parse_loc))(input)?;
    Ok((input, pattern.into()))
}

fn parse_loc(input: &str) -> IResult<&str, Location> {
    let (input, c) = one_of(".#")(input)?;
    Ok((input, c.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let result = process_part1(input);
        assert_eq!(result, "405");
    }

    #[test]
    fn part2() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let result = process_part2(input);
        assert_eq!(result, "400");
    }
}
