use nom::{
    bytes::complete::tag,
    character::complete::{newline, space0, space1, u64 as nom_u64},
    multi::separated_list1,
    sequence::{delimited, preceded, terminated},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, races) = parse_input(input).unwrap();
    races
        .iter()
        .map(|&(time, record)| {
            let d = time * time - 4 * record;
            let sqrt_d = (d as f64).sqrt();
            assert!(!sqrt_d.is_nan());
            let max = ((-(time as f64) - sqrt_d) / -2.0).ceil() as u64 - 1;
            let min = ((-(time as f64) + sqrt_d) / -2.0).floor() as u64;
            max - min
        })
        .product::<u64>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut new_input = input.to_string();
    new_input.retain(|c| c != ' ');
    process_part1(&new_input)
}

type Race = (u64, u64); // (time, record)

fn parse_input(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, times) = delimited(
        terminated(tag("Time:"), space0),
        separated_list1(space1, nom_u64),
        newline,
    )(input)?;
    let (input, distances) = preceded(
        terminated(tag("Distance:"), space0),
        separated_list1(space1, nom_u64),
    )(input)?;
    let races = times.into_iter().zip(distances).collect();
    Ok((input, races))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = process_part1(input);
        assert_eq!(result, "288");
    }

    #[test]
    fn part2() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = process_part2(input);
        assert_eq!(result, "71503");
    }
}
