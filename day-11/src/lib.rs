use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char as nomchar,
    character::complete::{alpha1, newline, one_of, space0, space1},
    combinator::recognize,
    multi::{many1, separated_list1},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    "".to_string()
}

pub fn process_part2(input: &str) -> String {
    "".to_string()
}

type Line<'a> = Vec<&'a str>;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = separated_list1(space1, alpha1)(input)?;
    Ok((input, line))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "";
        let result = process_part1(input);
        assert_eq!(result, "");
    }

    #[test]
    fn part2() {
        let input = "";
        let result = process_part2(input);
        assert_eq!(result, "");
    }
}
