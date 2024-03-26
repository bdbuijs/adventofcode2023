use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    character::complete::satisfy,
    combinator::map_res,
    multi::{many1, separated_list1},
    AsChar, IResult,
};

pub fn process_part1(input: &str) -> String {
    let answer: u32 = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| {
                    if c.is_ascii_digit() {
                        c.to_digit(10)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .map(|digits| {
            digits.first().expect("should be two digits in input") * 10
                + digits.last().expect("should be 2 digits in input")
        })
        .sum();
    answer.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, numbers) = parse_input(input).unwrap();
    let sum: u32 = numbers.into_iter().sum();
    sum.to_string()
}

fn parse_input(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, u32> {
    let (input, digits) = many1(parse_digit)(input)?;
    let digits: Vec<u32> = digits.into_iter().flatten().collect();
    let number = digits.first().expect("should be two digits in input") * 10
        + digits.last().expect("should be 2 digits in input");
    Ok((input, number))
}

fn parse_digit(input: &str) -> IResult<&str, Option<u32>> {
    let (input, digit) = alt((
        parse_single_digit,
        parse_written_out_digit,
        parse_character_to_none,
    ))(input)?;
    Ok((input, digit))
}

fn parse_single_digit(input: &str) -> IResult<&str, Option<u32>> {
    let (input, c) = satisfy(|c| c.is_dec_digit())(input)?;
    let digit = c.to_digit(10);
    assert!(digit.is_some());
    Ok((input, digit))
}

fn parse_written_out_digit(input: &str) -> IResult<&str, Option<u32>> {
    let (_, digit) = alt((
        map_res(tag("one"), |_| Ok::<std::option::Option<u32>, ()>(Some(1))),
        map_res(tag("two"), |_| Ok::<std::option::Option<u32>, ()>(Some(2))),
        map_res(tag("three"), |_| {
            Ok::<std::option::Option<u32>, ()>(Some(3))
        }),
        map_res(tag("four"), |_| Ok::<std::option::Option<u32>, ()>(Some(4))),
        map_res(tag("five"), |_| Ok::<std::option::Option<u32>, ()>(Some(5))),
        map_res(tag("six"), |_| Ok::<std::option::Option<u32>, ()>(Some(6))),
        map_res(tag("seven"), |_| {
            Ok::<std::option::Option<u32>, ()>(Some(7))
        }),
        map_res(tag("eight"), |_| {
            Ok::<std::option::Option<u32>, ()>(Some(8))
        }),
        map_res(tag("nine"), |_| Ok::<std::option::Option<u32>, ()>(Some(9))),
    ))(input)?;
    Ok((&input[1..], digit)) // the fact that this was necessary because 'twone' and the like are possible was a pain!
}

fn parse_character_to_none(input: &str) -> IResult<&str, Option<u32>> {
    let (input, _c) = satisfy(|c| c.is_alphanumeric())(input)?;
    Ok((input, None))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = process_part1(input);
        assert_eq!(result, "142");
    }

    #[test]
    fn part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = process_part2(input);
        assert_eq!(result, "281");
    }
}
