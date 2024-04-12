use std::collections::HashMap;

use nom::{
    character::complete::{char as nomchar, newline, one_of, u8 as nom_u8},
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, rows) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let mut cache = HashMap::new();
    rows.into_iter()
        .map(|record| count_solutions(&mut cache, &record))
        .sum::<usize>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, rows) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let mut cache = HashMap::new();
    rows.into_iter()
        .map(|record| count_solutions(&mut cache, &record.unfold()))
        .sum::<usize>()
        .to_string()
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Spring {
    Operational, // '.'
    Damaged,     // '#'
    Unknown,     // '?'
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            c => panic!("Unexpected character: {c}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Record {
    springs: Vec<Spring>,
    groups: Vec<u8>,
}

impl Record {
    fn new(springs: Vec<Spring>, groups: Vec<u8>) -> Self {
        Self { springs, groups }
    }

    fn next_spring(&self) -> Self {
        Self {
            springs: self.springs[1..].to_vec(),
            groups: self.groups.clone(),
        }
    }

    fn next_group(&self) -> Self {
        let end = (self.groups[0] as usize + 1).min(self.springs.len());
        Self {
            springs: self.springs[end..].to_vec(),
            groups: self.groups[1..].to_vec(),
        }
    }

    fn unfold(self) -> Self {
        let springs = self
            .springs
            .iter()
            .cloned()
            .chain(std::iter::once(Spring::Unknown))
            .cycle()
            .take(5 * self.springs.len() + 4)
            .collect();
        let groups = self
            .groups
            .iter()
            .cloned()
            .cycle()
            .take(5 * self.groups.len())
            .collect();
        Self { springs, groups }
    }
}

fn count_solutions(cache: &mut HashMap<Record, usize>, record: &Record) -> usize {
    if let Some(&solutions) = cache.get(record) {
        return solutions;
    }

    // no groups left + no damaged springs left = solved!
    if record.groups.is_empty() {
        let solutions = if record.springs.iter().all(|s| s != &Spring::Damaged) {
            1
        } else {
            0
        };
        cache.insert(record.clone(), solutions);
        return solutions;
    }

    // enough space left?
    if record.springs.len() < record.groups.iter().sum::<u8>() as usize + record.groups.len() - 1 {
        cache.insert(record.clone(), 0);
        return 0;
    }

    // skip operational spring
    if record.springs[0] == Spring::Operational {
        let solutions = count_solutions(cache, &record.next_spring());
        cache.insert(record.clone(), solutions);
        return solutions;
    }

    let mut solutions = 0;
    // if next group fits, recurse
    let group = record.groups[0] as usize;
    let fits = record.springs[0..group]
        .iter()
        .all(|s| s != &Spring::Operational)
        && ((record.springs.len() > group && record.springs[group] != Spring::Damaged)
            || record.springs.len() == group);
    // does this properly account for the case where the group exactly fits?
    if fits {
        solutions += count_solutions(cache, &record.next_group());
    }

    // also try skipping unknown
    if record.springs[0] == Spring::Unknown {
        solutions += count_solutions(cache, &record.next_spring());
    }

    cache.insert(record.clone(), solutions);
    solutions
}

fn parse_input(input: &str) -> IResult<&str, Vec<Record>> {
    let (input, lines) = separated_list1(newline, parse_record)(input)?;
    Ok((input, lines))
}

fn parse_record(input: &str) -> IResult<&str, Record> {
    // let (input, line) = terminated(many1(parse_span), nomchar(' '))(input)?;
    let (input, springs) = terminated(many1(parse_spring), nomchar(' '))(input)?;
    let (input, groups) = parse_groups(input)?;
    Ok((input, Record::new(springs, groups)))
}

fn parse_spring(input: &str) -> IResult<&str, Spring> {
    let (input, c) = one_of(".#?")(input)?;
    let spring = match c {
        '.' => Spring::Operational,
        '#' => Spring::Damaged,
        '?' => Spring::Unknown,
        _ => unreachable!("Unexpected character"),
    };
    Ok((input, spring))
}

fn parse_groups(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, groups) = separated_list1(nomchar(','), nom_u8)(input)?;
    Ok((input, groups))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unfold() {
        let input = "???.### 1,1,3";
        let (_, folded) = parse_record(input).unwrap();
        let expected_input =
            "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3";
        let (_, expected) = parse_record(expected_input).unwrap();
        let output = folded.unfold();
        assert_eq!(output, expected);
    }

    #[test]
    fn part1() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let result = process_part1(input);
        assert_eq!(result, "21");
    }

    #[test]
    fn part2() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let result = process_part2(input);
        assert_eq!(result, "525152");
    }
}
