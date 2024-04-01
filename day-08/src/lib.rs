use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::{
        complete::{alpha1, char as nomchar, newline},
        streaming::alphanumeric1,
    },
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
    IResult,
};
use num::Integer;

pub fn process_part1(input: &str) -> String {
    let (_, (instructions, nodes)) = parse_input(input).unwrap();
    let mut current = "AAA";
    let mut steps = 0;
    for step in instructions.chars().cycle() {
        let node = nodes.get(current).expect("Map must contain node!");
        current = match step {
            'L' => node.left,
            'R' => node.right,
            _ => unreachable!("Should only have L and R in instructions!"),
        };
        steps += 1;
        if current == "ZZZ" {
            return steps.to_string();
        }
    }
    unreachable!("Must find ZZZ somehow!");
}

pub fn process_part2(input: &str) -> String {
    let (_, (instructions, nodes)) = parse_input(input).unwrap();
    let current = nodes
        .iter()
        .filter_map(|(&name, _)| {
            if name.ends_with('A') {
                Some(name)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let mut cycles = Vec::new();
    current.into_iter().for_each(|n| {
        let mut start = 0;
        let mut name = n;
        for (count, step) in instructions.chars().cycle().enumerate() {
            let node = nodes.get(name).expect("Map must contain node!");
            name = match step {
                'L' => node.left,
                'R' => node.right,
                _ => unreachable!("Should only have L and R in instructions!"),
            };
            if name.ends_with('Z') {
                if start == 0 {
                    start = count;
                } else {
                    cycles.push(count - start);
                    break;
                }
            }
        }
    });
    cycles
        .into_iter()
        .fold(1_usize, |m, e| e.lcm(&m))
        .to_string()
}

struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

fn parse_input(input: &str) -> IResult<&str, (&str, HashMap<&str, Node>)> {
    let (input, instructions) = terminated(alpha1, tag("\n\n"))(input)?;
    let (input, nodes) = separated_list1(newline, parse_line)(input)?;
    let nodes = nodes.into_iter().collect::<HashMap<&str, Node>>();
    Ok((input, (instructions, nodes)))
}

fn parse_line(input: &str) -> IResult<&str, (&str, Node)> {
    let (input, name) = terminated(alphanumeric1, tag(" = "))(input)?;
    let (input, (left, _, right)) = delimited(
        nomchar('('),
        tuple((alphanumeric1, tag(", "), alphanumeric1)),
        nomchar(')'),
    )(input)?;
    Ok((input, (name, Node { left, right })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let result = process_part1(input);
        assert_eq!(result, "2");
    }

    #[test]
    fn part1_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let result = process_part1(input);
        assert_eq!(result, "6");
    }

    #[test]
    fn part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let result = process_part2(input);
        assert_eq!(result, "6");
    }
}
