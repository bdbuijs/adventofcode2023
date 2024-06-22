use std::fmt::Debug;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use itertools::Itertools;

use nom::{
    bytes::complete::tag,
    character::complete::{i128 as nomi128, newline, space1},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (min, max) = if input.len() == 119 {
        (7_i128, 27_i128)
    } else {
        (200000000000000, 400000000000000)
    };
    let (input, hailstones) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let range = min..=max;

    let intersections = hailstones
        .iter()
        .combinations(2)
        .filter_map(|v| {
            let a = v[0];
            let b = v[1];
            a.intersect_xy(b)
        })
        .filter(|(x, y)| range.contains(x) && range.contains(y))
        .collect::<Vec<_>>();

    intersections.len().to_string()
}

pub fn process_part2(input: &str) -> String {
    let search_width = if input.len() == 119 {
        10_i128
    } else {
        300_i128
    };
    let (input, hailstones) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let sample_size = 4;
    let velocity;
    for x in -search_width..=search_width {
        for y in -search_width..=search_width {
            let dpoint = Point { x, y, z: 0 };
            if hailstones
                .iter()
                .take(sample_size)
                .map(|h| h - dpoint)
                .combinations(2)
                .filter_map(|v| {
                    let a = &v[0];
                    let b = &v[1];
                    b.intersect_xy(a)
                })
                .all_equal()
            {
                // now try xz;

                for z in -search_width..=search_width {
                    let dpoint = Point { x, y, z };
                    if hailstones
                        .iter()
                        .take(sample_size)
                        .map(|h| h - dpoint)
                        .combinations(2)
                        .filter_map(|v| {
                            let a = &v[0];
                            let b = &v[1];
                            b.intersect_xz(a)
                        })
                        .all_equal()
                        && hailstones
                            .iter()
                            .take(sample_size)
                            .map(|h| h - dpoint)
                            .combinations(2)
                            .filter_map(|v| {
                                let a = &v[0];
                                let b = &v[1];
                                b.intersect_yz(a)
                            })
                            .all_equal()
                    {
                        velocity = Point { x, y, z };
                        let a = &hailstones[0] - velocity;
                        let b = &hailstones[1] - velocity;
                        let (solx, soly) = a.intersect_xy(&b).expect("found solution");
                        let (solx2, solz) = a.intersect_xz(&b).expect("found solution");
                        assert_eq!(solx, solx2);
                        return (solx + soly + solz).to_string();
                    }
                }
            }
        }
    }
    unreachable!("Problem has no solution");
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i128,
    y: i128,
    z: i128,
}

impl Point {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, x) = nomi128(input)?;
        let (input, _) = preceded(tag(","), space1)(input)?;
        let (input, y) = nomi128(input)?;
        let (input, _) = preceded(tag(","), space1)(input)?;
        let (input, z) = nomi128(input)?;
        Ok((input, Self { x, y, z }))
    }
}

impl Add<Self> for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        Self { x, y, z }
    }
}

impl AddAssign<Self> for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<Self> for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        Self { x, y, z }
    }
}

impl SubAssign<Self> for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

#[derive(Clone)]
struct HailStone {
    position: Point,
    velocity: Point,
}

impl HailStone {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, position) = Point::parse(input)?;
        let (input, _) = preceded(tag(" @"), space1)(input)?;
        let (input, velocity) = Point::parse(input)?;

        Ok((input, Self { position, velocity }))
    }

    fn intersect_xy(&self, other: &Self) -> Option<(i128, i128)> {
        let eq_self = LinearEquation::from_point(
            self.position.x,
            self.position.y,
            self.velocity.x,
            self.velocity.y,
        );
        let eq_other = LinearEquation::from_point(
            other.position.x,
            other.position.y,
            other.velocity.x,
            other.velocity.y,
        );
        match eq_self.intersect(&eq_other) {
            Some((x, y)) => {
                if (x - self.position.x).signum() == self.velocity.x.signum()
                    && (y - self.position.y).signum() == self.velocity.y.signum()
                    && (x - other.position.x).signum() == other.velocity.x.signum()
                    && (y - other.position.y).signum() == other.velocity.y.signum()
                {
                    Some((x, y))
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn intersect_xz(&self, other: &Self) -> Option<(i128, i128)> {
        let eq_self = LinearEquation::from_point(
            self.position.x,
            self.position.z,
            self.velocity.x,
            self.velocity.z,
        );
        let eq_other = LinearEquation::from_point(
            other.position.x,
            other.position.z,
            other.velocity.x,
            other.velocity.z,
        );
        match eq_self.intersect(&eq_other) {
            Some((x, z)) => {
                if (x - self.position.x).signum() == self.velocity.x.signum()
                    && (z - self.position.z).signum() == self.velocity.z.signum()
                    && (x - other.position.x).signum() == other.velocity.x.signum()
                    && (z - other.position.z).signum() == other.velocity.z.signum()
                {
                    Some((x, z))
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn intersect_yz(&self, other: &Self) -> Option<(i128, i128)> {
        let eq_self = LinearEquation::from_point(
            self.position.y,
            self.position.z,
            self.velocity.y,
            self.velocity.z,
        );
        let eq_other = LinearEquation::from_point(
            other.position.y,
            other.position.z,
            other.velocity.y,
            other.velocity.z,
        );
        match eq_self.intersect(&eq_other) {
            Some((y, z)) => {
                if (y - self.position.y).signum() == self.velocity.y.signum()
                    && (z - self.position.z).signum() == self.velocity.z.signum()
                    && (y - other.position.y).signum() == other.velocity.y.signum()
                    && (z - other.position.z).signum() == other.velocity.z.signum()
                {
                    Some((y, z))
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

impl Debug for HailStone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Hailstone: {}, {}, {}, @ {}, {}, {}",
            self.position.x,
            self.position.y,
            self.position.z,
            self.velocity.x,
            self.velocity.y,
            self.velocity.z
        ))
    }
}

impl Add<Point> for HailStone {
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        let position = self.position;
        let velocity = self.velocity + rhs;
        Self { position, velocity }
    }
}

impl AddAssign<Point> for HailStone {
    fn add_assign(&mut self, rhs: Point) {
        self.velocity += rhs;
    }
}

impl Sub<Point> for &HailStone {
    type Output = HailStone;

    fn sub(self, rhs: Point) -> Self::Output {
        let position = self.position;
        let velocity = self.velocity - rhs;
        HailStone { position, velocity }
    }
}

impl SubAssign<Point> for HailStone {
    fn sub_assign(&mut self, rhs: Point) {
        self.velocity -= rhs;
    }
}

/// Equation of the form ax + by = c
struct LinearEquation {
    a: i128,
    b: i128,
    c: i128,
}

impl LinearEquation {
    fn from_point(x: i128, y: i128, dx: i128, dy: i128) -> Self {
        let a = dy;
        let b = -dx;
        let c = dy * x - dx * y;
        Self { a, b, c }
    }

    fn intersect(&self, other: &Self) -> Option<(i128, i128)> {
        let d = other.b * self.a - other.a * self.b;
        if d == 0 {
            return None;
        }
        let x = (other.b * self.c - self.b * other.c) / d;
        let y = (other.c * self.a - self.c * other.a) / d;
        Some((x, y))
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<HailStone>> {
    let (input, lines) = separated_list1(newline, HailStone::parse)(input)?;
    Ok((input, lines))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        let result = process_part1(input);
        assert_eq!(result, "2");
    }

    #[test]
    fn part2() {
        let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        let result = process_part2(input);
        assert_eq!(result, "47");
    }
}
