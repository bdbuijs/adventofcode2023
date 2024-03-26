use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{char as nomchar, u32 as nom_u32},
    character::complete::{newline, space1},
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, cards) = parse_input(input).unwrap();
    let points: u32 = cards
        .iter()
        .map(|card| {
            let mut win = HashSet::new();
            card.win.iter().for_each(|&n| {
                win.insert(n);
            });
            let mut have = HashSet::new();
            card.have.iter().for_each(|&n| {
                have.insert(n);
            });
            let winning = win.intersection(&have).count() as u32;
            if let Some(w) = winning.checked_sub(1) {
                2_u32.pow(w)
            } else {
                0
            }
        })
        .sum();
    points.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, cards) = parse_input(input).unwrap();
    let top_card = cards.last().unwrap().n;
    let cards = cards
        .into_iter()
        .map(|c| (c.n, c))
        .collect::<HashMap<_, _>>();
    let mut counts = vec![1_u32; top_card as usize + 1];
    counts[0] = 0;
    (1..=top_card).for_each(|card_no| {
        let card = cards.get(&card_no).unwrap();
        let current_card_count = counts[card_no as usize];
        let start = card_no as usize + 1;
        let mut win = HashSet::new();
        card.win.iter().for_each(|&n| {
            win.insert(n);
        });
        let mut have = HashSet::new();
        card.have.iter().for_each(|&n| {
            have.insert(n);
        });
        let winning = win.intersection(&have).count();
        (start..(start + winning)).for_each(|i| {
            counts[i] += current_card_count;
        })
    });
    let total_cards = counts.iter().sum::<u32>();
    total_cards.to_string()
}

type Line<'a> = Card;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, n) = delimited(
        tuple((tag("Card"), space1)),
        nom_u32,
        tuple((nomchar(':'), space1)),
    )(input)?;
    let (input, win) = terminated(
        separated_list1(space1, nom_u32),
        tuple((space1, nomchar('|'), space1)),
    )(input)?;
    let (input, have) = separated_list1(space1, nom_u32)(input)?;
    let line = Card { n, win, have };
    Ok((input, line))
}

#[derive(Debug)]
struct Card {
    n: u32,
    win: Vec<u32>,
    have: Vec<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = process_part1(input);
        assert_eq!(result, "13");
    }

    #[test]
    fn part2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = process_part2(input);
        assert_eq!(result, "30");
    }
}
