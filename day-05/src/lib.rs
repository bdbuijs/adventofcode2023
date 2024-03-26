use nom::{
    bytes::complete::tag,
    character::complete::u64 as nom_u64,
    character::complete::{alpha1, newline, space1},
    multi::{many1, separated_list1},
    sequence::{delimited, tuple},
    IResult,
};
use std::{cmp::Ordering, fmt::Debug};

pub fn process_part1(input: &str) -> String {
    let (_, (seeds, maps)) = parse_input(input).unwrap();
    let map = maps
        .into_iter()
        .reduce(|acc, e| acc.chain(e))
        .expect("Should be at least one map!");
    seeds
        .into_iter()
        .map(|seed| map.map(seed))
        .min()
        .unwrap()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, (seeds, maps)) = parse_input(input).unwrap();
    let mut map = maps
        .into_iter()
        .reduce(|acc, e| acc.chain(e))
        .expect("Should be at least one map!");
    let mut seeds = Map {
        translations: seeds
            .chunks(2)
            .map(|s| {
                let r = Range::new(s[0], s[1]);
                Translation::new(r, r)
            })
            .collect(),
    };
    map.sort_source();
    seeds.sort_destination();
    let mut begin = seeds.translations.into_iter();
    let mut end = map.translations.into_iter();
    let mut a = begin.next().expect("Must be at least one translation!");
    let mut b = end.next().expect("Must be at least one translation!");
    let mut translations = Vec::new();
    loop {
        while b.source.end < a.destination.start {
            b = end.next().expect("map is larger than seeds");
        }
        let len_a = a.len();
        let len_b = b.len();
        match len_a.cmp(&len_b) {
            Ordering::Less => {
                // a is shorter than b
                let (first, last) = b.split(a.len());
                translations.push(Translation::new(a.source, first.destination));
                b = last;
                if let Some(c) = begin.next() {
                    a = c;
                } else {
                    break;
                }
            }
            Ordering::Equal => {
                translations.push(Translation::new(a.source, b.destination));
                if let Some(t) = begin.next() {
                    a = t;
                    b = end
                        .next()
                        .expect("Chained translations must both cover 0..u64::MAX");
                } else {
                    break;
                }
            }
            Ordering::Greater => {
                // b is shorter than a
                let (first, last) = a.split(b.len());
                translations.push(Translation::new(first.source, b.destination));
                b = end.next().expect("Destination translations list too short");
                a = last;
            }
        }
    }

    let mut final_map = Map { translations };
    dbg!(&final_map);
    final_map.sort_destination();
    final_map
        .translations
        .first()
        .expect("Must be at least one")
        .destination
        .start
        .to_string()
}

fn parse_input(input: &str) -> IResult<&str, (Vec<u64>, Vec<Map>)> {
    let (input, seeds) = delimited(
        tag("seeds: "),
        separated_list1(space1, nom_u64),
        tag("\n\n"),
    )(input)?;
    let (input, maps) = separated_list1(tag("\n\n"), parse_map)(input)?;
    Ok((input, (seeds, maps)))
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, _) = tuple((alpha1, tag("-to-"), alpha1, tag(" map:\n")))(input)?;
    let (input, translations) = separated_list1(many1(newline), parse_translation)(input)?;
    Ok((input, Map::new(translations)))
}

fn parse_translation(input: &str) -> IResult<&str, Translation> {
    let (input, (dest_range_start, _, source_range_start, _, range_len)) =
        tuple((nom_u64, space1, nom_u64, space1, nom_u64))(input)?;
    Ok((
        input,
        Translation::new(
            Range::new(source_range_start, range_len),
            Range::new(dest_range_start, range_len),
        ),
    ))
}

#[derive(Clone, Copy)]
struct Range {
    start: u64,
    end: u64, // non-inclusive!
}

impl Range {
    fn new(start: u64, len: u64) -> Self {
        assert_ne!(len, 0);
        Self {
            start,
            end: start + len,
        }
    }

    fn len(&self) -> u64 {
        self.end - self.start
    }

    fn contains(&self, num: u64) -> bool {
        num >= self.start && num < self.end
    }

    fn from_zero(end: u64) -> Self {
        Self { start: 0, end }
    }

    fn to_max(start: u64) -> Self {
        Self {
            start,
            end: u64::MAX,
        }
    }
}

impl Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

struct Translation {
    source: Range,
    destination: Range,
}

impl Translation {
    fn new(source: Range, destination: Range) -> Self {
        assert_eq!(source.len(), destination.len());
        Self {
            source,
            destination,
        }
    }

    fn len(&self) -> u64 {
        self.source.len()
    }

    fn split(self, len: u64) -> (Self, Self) {
        assert!(len < self.len(), "Attempted to split with too large len");
        let start = Self::new(
            Range::new(self.source.start, len),
            Range::new(self.destination.start, len),
        );
        let end = Self::new(
            Range::new(self.source.start + len, self.len() - len),
            Range::new(self.destination.start + len, self.len() - len),
        );

        (start, end)
    }
}

impl Debug for Translation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}->{:?}", self.source, self.destination)
    }
}

#[derive(Debug)]
struct Map {
    translations: Vec<Translation>,
}

impl Map {
    fn new(given_translations: Vec<Translation>) -> Self {
        let mut given_translations = given_translations;
        given_translations.sort_by(|a, b| a.source.start.cmp(&b.source.start));
        let mut translations = Vec::new();
        let mut counter = 0;
        given_translations.into_iter().for_each(|t| {
            if t.source.start > counter {
                let len = t.source.start - counter;
                let range = Range::new(counter, len);
                translations.push(Translation::new(range, range));
                counter += len;
                assert_eq!(counter, t.source.start)
            }
            counter += t.len();
            translations.push(t);
        });
        if counter < u64::MAX {
            let range = Range::to_max(counter);
            translations.push(Translation::new(range, range))
        }
        Self { translations }
    }

    fn map(&self, seed: u64) -> u64 {
        for t in self.translations.iter() {
            if t.source.contains(seed) {
                let n = seed - t.source.start;
                return t.destination.start + n;
            }
        }
        unreachable!("Map should cover the whole range!!!")
    }

    fn sort_source(&mut self) {
        self.translations
            .sort_by(|a, b| a.source.start.cmp(&b.source.start))
    }

    fn sort_destination(&mut self) {
        self.translations
            .sort_by(|a, b| a.destination.start.cmp(&b.destination.start))
    }

    fn chain(mut self, mut other: Self) -> Self {
        self.sort_destination();
        other.sort_source();
        let mut begin = self.translations.into_iter();
        let mut end = other.translations.into_iter();
        let mut a = begin.next().expect("Must be at least one translation!");
        let mut b = end.next().expect("Must be at least one translation!");
        let mut translations = Vec::new();
        loop {
            let len_a = a.len();
            let len_b = b.len();
            match len_a.cmp(&len_b) {
                Ordering::Less => {
                    // a is shorter than b
                    let (first, last) = b.split(a.len());
                    translations.push(Translation::new(a.source, first.destination));
                    a = begin.next().expect("Source translations list too short");
                    b = last;
                }
                Ordering::Equal => {
                    translations.push(Translation::new(a.source, b.destination));
                    if let Some(t) = begin.next() {
                        a = t;
                        b = end
                            .next()
                            .expect("Chained translations must both cover 0..u64::MAX");
                    } else {
                        break;
                    }
                }
                Ordering::Greater => {
                    // b is shorter than a
                    let (first, last) = a.split(b.len());
                    translations.push(Translation::new(first.source, b.destination));
                    b = end.next().expect("Destination translations list too short");
                    a = last;
                }
            }
        }

        Self { translations }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = process_part1(input);
        assert_eq!(result, "35");
    }

    #[test]
    fn part2() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = process_part2(input);
        assert_eq!(result, "46");
    }
}
