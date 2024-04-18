use nom::{
    character::complete::u8 as nom_u8,
    character::complete::{alpha1, one_of},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let sequence: Vec<&str> = input.split(',').collect();
    sequence
        .iter()
        .map(|s| hash(s) as u64)
        .sum::<u64>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let sequence: Vec<&str> = input.split(',').collect();
    let sequence: Vec<Operation> = sequence
        .into_iter()
        .map(|s| {
            let (input, operation) = parse_operation(s).expect("Unparseable operation: {s}");
            assert!(input.is_empty());
            operation
        })
        .collect();
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];

    sequence.into_iter().for_each(|operation| {
        let b = hash(operation.label());
        match operation {
            Operation::Remove(label) => {
                if let Some((i, _)) = boxes[b as usize]
                    .iter()
                    .enumerate()
                    .find(|(_i, l)| l.label == label)
                {
                    boxes[b as usize].remove(i);
                }
            }
            Operation::Add(label, focal_length) => {
                if let Some(lens) = boxes[b as usize].iter_mut().find(|l| l.label == label) {
                    *lens = Lens {
                        label,
                        focal_length,
                    };
                } else {
                    boxes[b as usize].push(Lens {
                        label,
                        focal_length,
                    })
                }
            }
        }
    });

    boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            (i + 1)
                * b.iter()
                    .enumerate()
                    .map(|(l_i, l)| (l_i + 1) * l.focal_length as usize)
                    .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

fn hash(s: impl AsRef<str>) -> u8 {
    s.as_ref()
        .chars()
        .fold(0, |acc, el| acc.wrapping_add(el as u8).wrapping_mul(17))
}

enum Operation<'a> {
    Remove(&'a str),
    Add(&'a str, u8),
}

impl<'a> Operation<'a> {
    fn label(&'a self) -> &'a str {
        match self {
            Operation::Remove(label) => label,
            Operation::Add(label, _) => label,
        }
    }
}

#[derive(Debug, Clone)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u8,
}

fn parse_operation(input: &str) -> IResult<&str, Operation<'_>> {
    let (input, label) = alpha1(input)?;
    let (input, kind) = one_of("=-")(input)?;
    match kind {
        '=' => {
            let (input, focal_length) = nom_u8(input)?;
            Ok((input, Operation::Add(label, focal_length)))
        }
        '-' => Ok((input, Operation::Remove(label))),
        c => unreachable!("Unexpected character: {c}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let input = "HASH";
        let result = hash(input).to_string();
        assert_eq!(result, "52");
    }

    #[test]
    fn part1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let result = process_part1(input);
        assert_eq!(result, "1320");
    }

    #[test]
    fn part2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let result = process_part2(input);
        assert_eq!(result, "145");
    }
}
