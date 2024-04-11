use nom::{
    character::complete::{char as nomchar, newline, one_of, u8 as nom_u8},
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, rows) = parse_input(input).unwrap();
    assert!(input.is_empty());
    rows.into_iter()
        .map(|(mut springs, groups)| {
            let placeable = groups.iter().sum::<u8>()
                - springs.iter().filter(|s| s == &&Spring::Damaged).count() as u8;
            solve(&mut springs, placeable, 0, &groups)
        })
        .sum::<usize>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, rows) = parse_input(input).unwrap();
    assert!(input.is_empty());
    "".to_string()
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Spring {
    Operational, // '.'
    Damaged,     // '#'
    Unknown,     // '?'
}

fn solve(springs: &mut Vec<Spring>, placeable: u8, solutions: usize, groups: &[u8]) -> usize {
    if placeable == 0 {
        let solved = is_solved(springs, groups);
        if solved {
            return solutions + 1;
        } else {
            return solutions;
        }
    }
    let mut solutions = solutions;
    for i in 0..springs.len() {
        if springs[i] == Spring::Unknown {
            springs[i] = Spring::Damaged;
            let new_solutions = solve(springs, placeable - 1, solutions, groups);
            if new_solutions > solutions {
                solutions = new_solutions;
            }
            springs[i] = Spring::Operational;
            let new_solutions = solve(springs, placeable, solutions, groups);
            if new_solutions > solutions {
                solutions = new_solutions;
            }
            springs[i] = Spring::Unknown;
            return solutions;
        }
    }
    solutions
}

fn is_solved(springs: &[Spring], groups: &[u8]) -> bool {
    let (mut found, count) = springs
        .iter()
        .fold((vec![], 0_u8), |(mut acc, mut count), el| {
            match el {
                Spring::Damaged => count += 1,
                _ => {
                    if count > 0 {
                        acc.push(count);
                        count = 0;
                    }
                }
            }
            (acc, count)
        });
    if count > 0 {
        found.push(count);
    }

    found == groups
}

fn unfold(springs: Vec<Spring>, groups: Vec<u8>) -> (Vec<Spring>, Vec<u8>) {
    let springs = springs
        .iter()
        .cloned()
        .chain(std::iter::once(Spring::Unknown))
        .cycle()
        .take(5 * springs.len() + 4)
        .collect();
    let groups = groups
        .iter()
        .cloned()
        .cycle()
        .take(5 * groups.len())
        .collect();
    (springs, groups)
}

type Line<'a> = (Vec<Spring>, Vec<u8>);

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    // let (input, line) = terminated(many1(parse_span), nomchar(' '))(input)?;
    let (input, springs) = terminated(many1(parse_spring), nomchar(' '))(input)?;
    let (input, groups) = parse_groups(input)?;
    Ok((input, (springs, groups)))
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
    fn test_is_solved() {
        let springs = vec![
            Spring::Operational,
            Spring::Damaged,
            Spring::Damaged,
            Spring::Damaged,
            Spring::Operational,
            Spring::Damaged,
            Spring::Damaged,
            Spring::Operational,
            Spring::Damaged,
            Spring::Operational,
            Spring::Operational,
            Spring::Operational,
        ];
        assert!(is_solved(springs.as_slice(), vec![3, 2, 1].as_slice()));
    }

    #[test]
    fn temp() {
        let input = "?###???????? 3,2,1";
        // let input = ".###.##.? 3,2,1";
        let (input, (mut springs, groups)) = parse_line(input).expect("Parsing error");
        assert!(input.is_empty());
        let result = solve(&mut springs, 3, 0, &groups);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_unfold() {
        let input = "???.### 1,1,3";
        let (_, folded) = parse_line(input).unwrap();
        let expected_input =
            "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3";
        let (_, expected) = parse_line(expected_input).unwrap();
        let output = unfold(folded.0, folded.1);
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
