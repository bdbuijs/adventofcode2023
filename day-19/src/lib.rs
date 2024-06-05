use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
    ops::RangeInclusive,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char as nomchar, newline, one_of, u64 as nomu64},
    multi::separated_list1,
    sequence::{delimited, terminated},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, (workflows, parts)) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let mut accepted: Vec<Part> = Vec::new();
    parts.iter().for_each(|part| {
        let mut next = workflows.get(&"in").expect("There's always an in");
        loop {
            let target = next.apply(part);
            match target {
                Target::Accepted => accepted.push(part.clone()),
                Target::Rejected => {}
                Target::Workflow(id) => {
                    next = workflows.get(id).expect("Target workflow must exist");
                    continue;
                }
            }
            break;
        }
    });

    let total = accepted.iter().map(|p| p.sum()).sum::<usize>();
    total.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, (workflows, _)) = parse_input(input).unwrap();
    assert!(input.is_empty());
    // (workflow_id, PossiblePart)
    let mut accepted = Vec::new();
    let mut queue: VecDeque<(&str, PossiblePart)> = VecDeque::new();
    queue.push_back(("in", PossiblePart::default()));
    while let Some((workflow_id, mut possible_part)) = queue.pop_front() {
        let workflow = workflows.get(workflow_id).expect("Workflow should exist");
        for rule in workflow.rules.iter() {
            let (next, more) = rule.apply_possible(possible_part);
            if let Some((target, more_possible)) = more {
                match target {
                    Target::Rejected => {} // we're done with this part
                    Target::Accepted => accepted.push(more_possible),
                    Target::Workflow(id) => queue.push_back((id, more_possible)),
                }
            }
            if let Some(next) = next {
                possible_part = next; // keep going with this part
            } else {
                break; // nowhere else to go in this workflow
            }
        }
    }
    let combinations = accepted.iter().map(|p| p.combinations()).sum::<usize>();

    combinations.to_string()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Rating {
    X,
    M,
    A,
    S,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Target<'a> {
    Accepted,
    Rejected,
    Workflow(&'a str),
}

impl<'a> Target<'a> {
    fn new(id: &'a str) -> Self {
        match id {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            s => Self::Workflow(s),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Rule<'a> {
    rating: Rating,
    compare: Ordering,
    value: usize,
    target: Target<'a>,
}

impl<'a> Rule<'a> {
    fn direct(target: Target<'a>) -> Self {
        Self {
            rating: Rating::X,
            compare: Ordering::Equal,
            value: 0,
            target,
        }
    }

    fn conditional(rating: Rating, compare: Ordering, value: usize, target: Target<'a>) -> Self {
        Self {
            rating,
            compare,
            value,
            target,
        }
    }

    fn apply(&'a self, part: &'_ Part) -> Option<Target<'a>> {
        match self.compare {
            Ordering::Equal => Some(self.target),
            Ordering::Less => {
                if part.rate(self.rating) < self.value {
                    Some(self.target)
                } else {
                    None
                }
            }

            Ordering::Greater => {
                if part.rate(self.rating) > self.value {
                    Some(self.target)
                } else {
                    None
                }
            }
        }
    }

    /// (Some(if any possible part makes it 'past' this rule), Some(if this rule sends a possible part to a target))
    fn apply_possible(
        &'a self,
        possible_part: PossiblePart,
    ) -> (Option<PossiblePart>, Option<(Target<'a>, PossiblePart)>) {
        match self.compare {
            Ordering::Equal => (None, Some((self.target, possible_part))),
            Ordering::Less => {
                let (left, right) = possible_part.split(self.rating, self.value - 1);
                (right.remaining(), Some((self.target, left)))
            }
            Ordering::Greater => {
                let (left, right) = possible_part.split(self.rating, self.value);
                (left.remaining(), Some((self.target, right)))
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Workflow<'a> {
    id: &'a str,
    rules: Vec<Rule<'a>>,
}

impl<'a> Workflow<'a> {
    fn apply(&'a self, part: &Part) -> Target<'a> {
        for rule in self.rules.iter() {
            if let Some(target) = rule.apply(part) {
                return target;
            }
        }
        unreachable!("Workflow should always have a fallback that applies!");
    }
}

#[derive(Debug, Clone)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn new(x: usize, m: usize, a: usize, s: usize) -> Self {
        Self { x, m, a, s }
    }

    fn rate(&self, rating: Rating) -> usize {
        match rating {
            Rating::X => self.x,
            Rating::M => self.m,
            Rating::A => self.a,
            Rating::S => self.s,
        }
    }

    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
struct PossiblePart {
    x: RangeInclusive<usize>,
    m: RangeInclusive<usize>,
    a: RangeInclusive<usize>,
    s: RangeInclusive<usize>,
}

impl PossiblePart {
    #[allow(clippy::reversed_empty_ranges)]
    /// Value is included in 'left' half of range
    fn split(&self, rating: Rating, value: usize) -> (PossiblePart, PossiblePart) {
        let (left_x, left_m, left_a, left_s, right_x, right_m, right_a, right_s);
        match rating {
            Rating::X => {
                left_x = (*self.x.start())..=value;
                left_m = self.m.clone();
                left_a = self.a.clone();
                left_s = self.s.clone();
                right_x = (value + 1)..=(*self.x.end());
                right_m = self.m.clone();
                right_a = self.a.clone();
                right_s = self.s.clone();
            }
            Rating::M => {
                left_x = self.x.clone();
                left_m = (*self.m.start())..=value;
                left_a = self.a.clone();
                left_s = self.s.clone();
                right_x = self.x.clone();
                right_m = (value + 1)..=(*self.m.end());
                right_a = self.a.clone();
                right_s = self.s.clone();
            }
            Rating::A => {
                left_x = self.x.clone();
                left_m = self.m.clone();
                left_a = (*self.a.start())..=value;
                left_s = self.s.clone();
                right_x = self.x.clone();
                right_m = self.m.clone();
                right_a = (value + 1)..=(*self.a.end());
                right_s = self.s.clone();
            }
            Rating::S => {
                left_x = self.x.clone();
                left_m = self.m.clone();
                left_a = self.a.clone();
                left_s = (*self.s.start())..=value;
                right_x = self.x.clone();
                right_m = self.m.clone();
                right_a = self.a.clone();
                right_s = (value + 1)..=(*self.s.end());
            }
        }
        (
            Self {
                x: left_x,
                m: left_m,
                a: left_a,
                s: left_s,
            },
            Self {
                x: right_x,
                m: right_m,
                a: right_a,
                s: right_s,
            },
        )
    }

    /// returns Some(Self) if not all ranges are empty
    fn remaining(self) -> Option<Self> {
        if !self.x.is_empty() && !self.m.is_empty() && !self.a.is_empty() && !self.s.is_empty() {
            Some(self)
        } else {
            None
        }
    }

    fn combinations(&self) -> usize {
        self.x.clone().count()
            * self.m.clone().count()
            * self.a.clone().count()
            * self.s.clone().count()
    }
}

impl Default for PossiblePart {
    fn default() -> Self {
        Self {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, (HashMap<&str, Workflow>, Vec<Part>)> {
    let (input, workflows) =
        terminated(separated_list1(newline, parse_workflow), tag("\n\n"))(input)?;
    let workflows = HashMap::from_iter(workflows.into_iter().map(|w| (w.id, w)));
    let (input, parts) = separated_list1(newline, parse_part)(input)?;
    Ok((input, (workflows, parts)))
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, id) = alpha1(input)?;
    let (input, rules) = delimited(
        nomchar('{'),
        separated_list1(nomchar(','), parse_rule),
        nomchar('}'),
    )(input)?;
    let workflow = Workflow { id, rules };
    Ok((input, workflow))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, rule) = alt((parse_conditional_rule, parse_direct_rule))(input)?;
    Ok((input, rule))
}

fn parse_conditional_rule(input: &str) -> IResult<&str, Rule> {
    let (input, rating) = one_of("xmas")(input)?;
    let (input, compare) = one_of("<>")(input)?;
    let (input, value) = terminated(nomu64, nomchar(':'))(input)?;
    let (input, target) = alpha1(input)?;
    let rating = match rating {
        'x' => Rating::X,
        'm' => Rating::M,
        'a' => Rating::A,
        's' => Rating::S,
        _ => unreachable!(),
    };
    let compare = match compare {
        '<' => Ordering::Less,
        '>' => Ordering::Greater,
        _ => unreachable!(),
    };
    let value = value as usize;
    let target = Target::new(target);

    let rule = Rule::conditional(rating, compare, value, target);
    Ok((input, rule))
}

fn parse_direct_rule(input: &str) -> IResult<&str, Rule> {
    let (input, id) = alpha1(input)?;
    let target = Target::new(id);
    let rule = Rule::direct(target);
    Ok((input, rule))
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let (input, _) = tag("{x=")(input)?;
    let (input, x) = nomu64(input)?;
    let (input, _) = tag(",m=")(input)?;
    let (input, m) = nomu64(input)?;
    let (input, _) = tag(",a=")(input)?;
    let (input, a) = nomu64(input)?;
    let (input, _) = tag(",s=")(input)?;
    let (input, s) = nomu64(input)?;
    let (input, _) = tag("}")(input)?;
    let (x, m, a, s) = (x as usize, m as usize, a as usize, s as usize);
    let part = Part::new(x, m, a, s);
    Ok((input, part))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        let result = process_part1(input);
        assert_eq!(result, "19114");
    }

    #[test]
    fn part2() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        let result = process_part2(input);
        assert_eq!(result, "167409079868000");
    }
}
