use nom::{
    character::complete::i64 as nom_i64,
    character::complete::{newline, space1},
    multi::separated_list1,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, readings) = parse_input(input).unwrap();
    readings
        .into_iter()
        .map(process_history)
        .sum::<i64>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, readings) = parse_input(input).unwrap();
    readings
        .into_iter()
        .map(process_history_backwards)
        .sum::<i64>()
        .to_string()
}

fn process_history(history: Vec<i64>) -> i64 {
    let mut history = history;
    let mut ends = vec![*history.last().expect("Must be at least one reading!")];
    loop {
        let mut zeroes = true;
        history = history
            .windows(2)
            .map(|x| {
                let r = x[1] - x[0];
                zeroes &= r == 0;
                r
            })
            .collect::<Vec<i64>>();
        ends.push(
            *history
                .last()
                .expect("Must be last element if still looping!"),
        );
        if zeroes {
            break;
        }
    }
    ends.into_iter().sum()
}

fn process_history_backwards(history: Vec<i64>) -> i64 {
    let mut history = history;
    let mut ends = vec![*history.first().expect("Must be at least one reading!")];
    loop {
        let mut zeroes = true;
        history = history
            .windows(2)
            .map(|x| {
                let r = x[1] - x[0];
                zeroes &= r == 0;
                r
            })
            .collect::<Vec<i64>>();
        ends.push(
            *history
                .first()
                .expect("Must be last element if still looping!"),
        );
        if zeroes {
            break;
        }
    }
    ends.into_iter().rev().fold(0, |acc, e| e - acc)
}

type Line<'a> = Vec<i64>;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = separated_list1(space1, nom_i64)(input)?;
    Ok((input, line))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result = process_part1(input);
        assert_eq!(result, "114");
    }

    #[test]
    fn part2() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result = process_part2(input);
        assert_eq!(result, "2");
    }
}
