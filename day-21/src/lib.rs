use std::{collections::VecDeque, fmt::Debug};

pub fn process_part1(input: &str) -> String {
    let target_steps = if input.len() == 131 { 6 } else { 64 };
    let (garden, (start_x, start_y)) = parse_input(input);
    // print_garden(&garden);
    let width = garden[0].len();
    let height = garden.len();
    let mut steps_to_reach = vec![vec![0_usize; width]; height];
    let mut queue = VecDeque::new();
    neighbours(start_x, start_y, &garden, &steps_to_reach)
        .for_each(|(x, y)| queue.push_back(Node::new(x, y, 1)));
    while let Some(Node { x, y, steps }) = queue.pop_front() {
        if steps_to_reach[y][x] != 0 {
            continue;
        }
        for (new_x, new_y) in neighbours(x, y, &garden, &steps_to_reach) {
            queue.push_back(Node::new(new_x, new_y, steps + 1));
        }
        steps_to_reach[y][x] = steps;
    }
    let plots_after_target_steps = count_plots(&steps_to_reach, target_steps);
    plots_after_target_steps.to_string()
}

pub fn process_part2(input: &str) -> String {
    // The input has a diamond shape to it and the amount of steps is of the form ((65*2+1)*n)+65.
    // This insight derives from that we can reach the edge from the start point in 65 steps (thereby
    // covering the entire diamond) and we can reach the entire plot in 131 steps.
    // From that point we are - for every amount of steps of this form - going to be in a shape that
    // consists of diamonds and 'lopped-off' corners. The grids that are expanding outwards are going
    // to be alternating being in a state of reaching all 'odd' squares and all 'even' squares.
    // With some math we can then figure out that for an even case (202300*131+65) the number of even
    // grids that we're covering is n^2, the number of odd grids is (n+1)^2, the number of odd grids
    // where we're only covering the corners is n+1 and the number of even grids where we're only
    // covering the corners is n. So the total formula is n^2*evens + (n+1)^2*odds + n*even_corners
    // + (n+1)*odd_corners.
    let (garden, (start_x, start_y)) = parse_input(input);
    let width = garden[0].len();
    let height = garden.len();
    let mut steps_to_reach = vec![vec![0_usize; width]; height];
    let mut queue = VecDeque::new();
    neighbours(start_x, start_y, &garden, &steps_to_reach)
        .for_each(|(x, y)| queue.push_back(Node::new(x, y, 1)));
    while let Some(Node { x, y, steps }) = queue.pop_front() {
        if steps_to_reach[y][x] != 0 {
            continue;
        }
        for (new_x, new_y) in neighbours(x, y, &garden, &steps_to_reach) {
            queue.push_back(Node::new(new_x, new_y, steps + 1));
        }
        steps_to_reach[y][x] = steps;
    }

    // see logic above for the origins of the magic numbers
    let even = count_plots(&steps_to_reach, 132);
    let odd = count_plots(&steps_to_reach, 131);
    let even_corners = steps_to_reach
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&steps| steps > 65 && steps % 2 == 0)
        .count();
    let odd_corners = steps_to_reach
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&steps| steps > 65 && steps % 2 == 1)
        .count();
    let target_steps = 26501365_usize;
    let n = (target_steps - 65) / 131;
    let reachable =
        n.pow(2) * even + (n + 1).pow(2) * odd + n * even_corners - (n + 1) * odd_corners;

    reachable.to_string()
}

fn neighbours(
    x: usize,
    y: usize,
    garden: &[Vec<Garden>],
    steps_to_reach: &[Vec<usize>],
) -> std::vec::IntoIter<(usize, usize)> {
    let mut nbours: Vec<(usize, usize)> = Vec::new();
    if x > 0 && steps_to_reach[y][x - 1] == 0 && garden[y][x - 1].is_plot() {
        nbours.push((x - 1, y));
    }
    if x < (garden[0].len() - 1) && steps_to_reach[y][x + 1] == 0 && garden[y][x + 1].is_plot() {
        nbours.push((x + 1, y));
    }
    if y > 0 && steps_to_reach[y - 1][x] == 0 && garden[y - 1][x].is_plot() {
        nbours.push((x, y - 1));
    }
    if y < (garden.len() - 1) && steps_to_reach[y + 1][x] == 0 && garden[y + 1][x].is_plot() {
        nbours.push((x, y + 1));
    }

    nbours.into_iter()
}

fn count_plots(steps_to_reach: &[Vec<usize>], target_steps: usize) -> usize {
    steps_to_reach
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&steps| steps > 0 && steps <= target_steps && ((target_steps % 2) == steps % 2))
        .count()
}

#[derive(PartialEq, Eq)]
enum Garden {
    Plot,
    Rock,
}

impl Garden {
    #[inline(always)]
    fn is_plot(&self) -> bool {
        self == &Self::Plot
    }
}

impl Debug for Garden {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plot => write!(f, "."),
            Self::Rock => write!(f, "#"),
        }
    }
}

struct Node {
    x: usize,
    y: usize,
    steps: usize,
}

impl Node {
    fn new(x: usize, y: usize, steps: usize) -> Self {
        Self { x, y, steps }
    }
}

#[allow(dead_code)]
fn print_garden(garden: &[Vec<Garden>]) {
    garden.iter().for_each(|row| {
        row.iter().for_each(|g| print!("{:?}", g));
        println!();
    })
}

#[allow(dead_code)]
fn print_reachable_garden(garden: &[Vec<Garden>], steps_to_reach: &[Vec<usize>], steps: usize) {
    garden
        .iter()
        .zip(steps_to_reach.iter())
        .for_each(|(row_garden, row_steps)| {
            row_garden
                .iter()
                .zip(row_steps)
                .for_each(|(g, &s)| match g {
                    Garden::Rock => print!("#"),
                    Garden::Plot => {
                        if s != 0 && s <= steps && (s % 2 == steps % 2) {
                            print!("0")
                        } else {
                            print!(".")
                        }
                    }
                });
            println!();
        })
}

fn parse_input(input: &str) -> (Vec<Vec<Garden>>, (usize, usize)) {
    let (mut x, mut y) = (0, 0);
    let (mut start_x, mut start_y) = (x, y);
    let mut garden = Vec::new();
    let mut current_row = Vec::new();
    input.chars().for_each(|c| match c {
        'S' => {
            current_row.push(Garden::Plot);
            (start_x, start_y) = (x, y);
            x += 1;
        }
        '.' => {
            current_row.push(Garden::Plot);
            x += 1;
        }
        '#' => {
            current_row.push(Garden::Rock);
            x += 1;
        }
        '\n' => {
            y += 1;
            x = 0;
            let mut complete_row = Vec::new();
            std::mem::swap(&mut current_row, &mut complete_row);
            garden.push(complete_row);
        }
        x => unreachable!("Unexpected character: {x}"),
    });
    garden.push(current_row);

    (garden, (start_x, start_y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        let result = process_part1(input);
        assert_eq!(result, "16");
    }
}
