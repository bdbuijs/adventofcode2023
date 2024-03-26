use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, u32 as nom_u32},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    let red = Color::Red(12);
    let green = Color::Green(13);
    let blue = Color::Blue(14);
    let (_, games) = parse_input(input).unwrap();
    let sum: u32 = games
        .into_iter()
        .filter_map(|(n, sets)| {
            if sets.iter().all(|set| {
                set.iter().all(|color| {
                    red.contains(color) || green.contains(color) || blue.contains(color)
                })
            }) {
                Some(n)
            } else {
                None
            }
        })
        .sum();
    sum.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, games) = parse_input(input).unwrap();

    let sum: u32 = games
        .into_iter()
        .map(|(_, sets)| {
            let red = sets
                .iter()
                .flat_map(|c| c.iter())
                .filter_map(|c| {
                    if let &Color::Red(n) = c {
                        Some(n)
                    } else {
                        None
                    }
                })
                .max()
                .unwrap();
            let green = sets
                .iter()
                .flat_map(|c| c.iter())
                .filter_map(|c| {
                    if let &Color::Green(n) = c {
                        Some(n)
                    } else {
                        None
                    }
                })
                .max()
                .unwrap();
            let blue = sets
                .iter()
                .flat_map(|c| c.iter())
                .filter_map(|c| {
                    if let &Color::Blue(n) = c {
                        Some(n)
                    } else {
                        None
                    }
                })
                .max()
                .unwrap();
            red * green * blue
        })
        .sum();
    sum.to_string()
}

type Line<'a> = (u32, Vec<Vec<Color>>);

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_game)(input)?;
    Ok((input, lines))
}

fn parse_game(input: &str) -> IResult<&str, Line> {
    let (input, game_number) = delimited(tag("Game "), nom_u32, tag(": "))(input)?;
    let (input, sets) = separated_list1(tag("; "), separated_list1(tag(", "), parse_color))(input)?;
    Ok((input, (game_number, dbg!(sets))))
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    let (input, count) = nom_u32(input)?;
    let (input, color) = alt((tag(" red"), tag(" green"), tag(" blue")))(input)?;
    let color = match color {
        " red" => Color::Red(count),
        " green" => Color::Green(count),
        " blue" => Color::Blue(count),
        _ => unreachable!(),
    };
    Ok((input, color))
}

#[derive(Debug)]
enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl Color {
    fn contains(&self, balls: &Self) -> bool {
        match (self, balls) {
            (Self::Red(s), Self::Red(b)) => s >= b,
            (Self::Green(s), Self::Green(b)) => s >= b,
            (Self::Blue(s), Self::Blue(b)) => s >= b,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = process_part1(input);
        assert_eq!(result, "8");
    }

    #[test]
    fn part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = process_part2(input);
        assert_eq!(result, "2286");
    }
}
