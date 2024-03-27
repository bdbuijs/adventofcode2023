use itertools::Itertools;
use nom::{
    character::complete::{alphanumeric1, newline, space1, u64 as nom_u64},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

static SCORES: [&[usize]; 7] = [
    &[1, 1, 1, 1, 1],
    &[2, 1, 1, 1],
    &[2, 2, 1],
    &[3, 1, 1],
    &[3, 2],
    &[4, 1],
    &[5],
];

static CARDS_WITHOUT: &str = "23456789TJQKA";
static CARDS_WITH: &str = "J23456789TQKA";

fn process_hands(hands: Vec<Hand>) -> String {
    let mut hands = hands;
    hands.iter_mut().for_each(|h| h.score());
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid as usize)
        .sum::<usize>()
        .to_string()
}

pub fn process_part1(input: &str) -> String {
    let (_, hands) = parse_input(input).unwrap();
    process_hands(hands)
}

pub fn process_part2(input: &str) -> String {
    let (_, mut hands) = parse_input(input).unwrap();
    hands.iter_mut().for_each(|h| h.jokers = true);
    process_hands(hands)
}

#[derive(Debug, Eq, Ord)]
struct Hand<'a> {
    cards: &'a str,
    hand: Option<usize>, // 0: High Card, 1: Pair, 2: Two Pair, 3: 3-of-a-kind, 4: Full House, 5: 4-of-a-kind, 6: 5-of-a-kind
    bid: u64,
    jokers: bool,
}

impl<'a> Hand<'a> {
    fn score(&mut self) {
        if self.hand.is_none() {
            let mut score = self
                .cards
                .chars()
                .sorted_unstable()
                .dedup_with_count()
                .sorted_unstable_by_key(|c| c.0)
                .rev()
                .collect::<Vec<_>>();
            if self.jokers {
                let mut joker_count = 0;
                score.retain(|(s, c)| match c {
                    'J' => {
                        joker_count += s;
                        false
                    }
                    _ => true,
                });
                if let Some((s, _)) = score.first_mut() {
                    *s += joker_count;
                } else {
                    score.push((5, 'J')); // must be this case if the hand is empty!
                }
            }
            let score = score.into_iter().map(|(n, _)| n).collect::<Vec<_>>();
            self.hand = SCORES.iter().position(|&s| s == score.as_slice());
        }
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.hand == other.hand
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.eq(other) {
            return Some(std::cmp::Ordering::Equal);
        }
        let cards = if self.jokers {
            CARDS_WITH
        } else {
            CARDS_WITHOUT
        };
        match self.hand.cmp(&other.hand) {
            std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
            std::cmp::Ordering::Equal => {
                for (s, o) in self.cards.chars().zip(other.cards.chars()) {
                    match cards
                        .chars()
                        .position(|c| c == s)
                        .unwrap()
                        .cmp(&cards.chars().position(|c| c == o).unwrap())
                    {
                        std::cmp::Ordering::Less => return Some(std::cmp::Ordering::Less),
                        std::cmp::Ordering::Equal => {}
                        std::cmp::Ordering::Greater => return Some(std::cmp::Ordering::Greater),
                    };
                }
                None
            }
            std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Hand>> {
    let (input, lines) = separated_list1(newline, parse_hand)(input)?;
    Ok((input, lines))
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, (cards, _, bid)) = tuple((alphanumeric1, space1, nom_u64))(input)?;
    let hand = Hand {
        cards,
        hand: None,
        bid,
        jokers: false,
    };
    Ok((input, hand))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = process_part1(input);
        assert_eq!(result, "6440");
    }

    #[test]
    fn part2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = process_part2(input);
        assert_eq!(result, "5905");
    }
}
