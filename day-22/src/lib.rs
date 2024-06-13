use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
};

use nom::{
    character::complete::{char as nomchar, newline, u16 as nomu16},
    multi::separated_list1,
    sequence::terminated,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, bricks) = parse_input(input).unwrap();
    assert!(input.is_empty());

    let (bricks, supporting_bricks_of, bricks_supported_by) = process_bricks(bricks);

    // count the ones that can be disintegrated (i.e. are not supporting anything that is not otherwise supported)
    let disintegrateable = (0..bricks.len())
        .filter_map(|i| {
            if let Some(supported) = bricks_supported_by.get(&i) {
                // supported = all bricks supported by brick i
                // check if they all have more than 1 supporter
                if supported.iter().all(|s| {
                    if let Some(supporting) = supporting_bricks_of.get(s) {
                        supporting.len() > 1
                    } else {
                        false
                    }
                }) {
                    Some(())
                } else {
                    None
                }
            } else {
                // brick i is not supporting any bricks
                Some(())
            }
        })
        .count();

    disintegrateable.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, bricks) = parse_input(input).unwrap();
    assert!(input.is_empty());

    let (bricks, supporting_bricks_of, bricks_supported_by) = process_bricks(bricks);
    let sum = (0..bricks.len())
        .filter(|i| {
            if let Some(supported) = bricks_supported_by.get(i) {
                // supported = all bricks supported by brick i
                // check if they all have more than 1 supporter
                !supported.iter().all(|s| {
                    if let Some(supporting) = supporting_bricks_of.get(s) {
                        supporting.len() > 1
                    } else {
                        false
                    }
                })
            } else {
                // brick i is not supporting any bricks
                false
            }
        })
        .map(|i| {
            let mut disintegrated = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back(i);
            while let Some(brick_to_disintegrate) = queue.pop_front() {
                disintegrated.insert(brick_to_disintegrate);
                if let Some(supported) = bricks_supported_by.get(&brick_to_disintegrate) {
                    supported.iter().for_each(|sup| {
                        // sup is supported by a disintegrated brick, should it fall?
                        if let Some(supporting) = supporting_bricks_of.get(sup) {
                            if supporting
                                .iter()
                                .filter(|&s| !disintegrated.contains(s))
                                .count()
                                == 0
                            {
                                // yes, it should
                                queue.push_back(*sup);
                            }
                        }
                    })
                }
            }
            disintegrated.len().saturating_sub(1)
        })
        .sum::<usize>();
    sum.to_string()
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Point {
    x: u16,
    y: u16,
    z: u16,
}

impl Point {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, x) = terminated(nomu16, nomchar(','))(input)?;
        let (input, y) = terminated(nomu16, nomchar(','))(input)?;
        let (input, z) = nomu16(input)?;

        Ok((input, Self { x, y, z }))
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {}, {})", self.x, self.y, self.z))
    }
}

#[derive(Debug, Clone)]
struct Brick(Point, Point);

impl Brick {
    fn new(point1: Point, point2: Point) -> Self {
        Self(point1, point2)
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, point1) = Point::parse(input)?;
        let (input, _) = nomchar('~')(input)?;
        let (input, point2) = Point::parse(input)?;
        Ok((input, Brick::new(point1, point2)))
    }

    fn one_z_lower(&self) -> Option<Self> {
        if self.0.z.min(self.1.z) <= 1 {
            None
        } else {
            Some(Brick(
                Point {
                    x: self.0.x,
                    y: self.0.y,
                    z: self.0.z - 1,
                },
                Point {
                    x: self.1.x,
                    y: self.1.y,
                    z: self.1.z - 1,
                },
            ))
        }
    }

    #[allow(dead_code)]
    fn size(&self) -> usize {
        ((self.0.x.abs_diff(self.1.x) + 1)
            * (self.0.y.abs_diff(self.1.y) + 1)
            * (self.0.z.abs_diff(self.1.z) + 1)) as usize
    }

    /// Iterator over all individual points that make up the brick
    fn cubes(&self) -> impl Iterator<Item = Point> {
        let Point {
            x: x1,
            y: y1,
            z: z1,
        } = self.0;
        let Point {
            x: x2,
            y: y2,
            z: z2,
        } = self.1;

        (x1.min(x2)..=x1.max(x2)).flat_map(move |x| {
            (y1.min(y2)..=y1.max(y2))
                .flat_map(move |y| (z1.min(z2)..=z1.max(z2)).map(move |z| Point { x, y, z }))
        })
    }

    fn bottom_cubes(&self) -> impl Iterator<Item = Point> {
        let min_z = self.0.z.min(self.1.z);
        self.cubes().filter(move |p| p.z == min_z)
    }

    fn cubes_below(&self) -> impl Iterator<Item = Point> {
        self.bottom_cubes().map(|p| Point {
            x: p.x,
            y: p.y,
            z: p.z - 1,
        })
    }
}

type BrickDump = (
    Vec<Brick>,
    HashMap<usize, HashSet<usize>>,
    HashMap<usize, HashSet<usize>>,
);

fn process_bricks(bricks: Vec<Brick>) -> BrickDump {
    let mut bricks = bricks;
    let mut space: HashMap<Point, usize> = bricks
        .iter()
        .enumerate()
        .flat_map(|(i, b)| b.cubes().map(move |p| (p, i)))
        .collect();

    // apply gravity
    loop {
        let mut changed = false;
        bricks.iter_mut().enumerate().for_each(|(i, b)| {
            if b.cubes_below().all(|point| !space.contains_key(&point)) {
                if let Some(lower) = b.one_z_lower() {
                    changed = true;
                    b.cubes().for_each(|p| {
                        space.remove(&p);
                    });
                    lower.cubes().for_each(|p| {
                        space.insert(p, i);
                    });
                    *b = lower;
                }
            }
        });

        if !changed {
            break;
        }
    }

    // find out what is supporting what
    let supporting_bricks_of: HashMap<usize, HashSet<usize>> = bricks
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let set = b
                .cubes_below()
                .filter_map(|p| space.get(&p))
                .cloned()
                .collect();
            (i, set)
        })
        .collect();
    let mut bricks_supported_by: HashMap<usize, HashSet<usize>> = HashMap::new();
    supporting_bricks_of.iter().for_each(|(k, v)| {
        v.iter().for_each(|supporter| {
            bricks_supported_by
                .entry(*supporter)
                .or_insert_with(HashSet::new)
                .insert(*k);
        });
    });

    (bricks, supporting_bricks_of, bricks_supported_by)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Brick>> {
    let (input, bricks) = separated_list1(newline, Brick::parse)(input)?;
    Ok((input, bricks))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        let result = process_part1(input);
        assert_eq!(result, "5");
    }

    #[test]
    fn part2() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        let result = process_part2(input);
        assert_eq!(result, "7");
    }
}
