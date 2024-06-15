use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    error::Error,
    fmt::{Debug, Display},
    hash::Hash,
};

use nom::{
    branch::alt,
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, tiles) = parse_input(input).unwrap();
    assert!(input.is_empty());

    let height = tiles.len() - 1;

    let start_x = tiles[0]
        .iter()
        .enumerate()
        .fold(None, |acc, (i, t)| match t {
            &Tile::Path => {
                if acc.is_some() {
                    panic!("There should be only one path tile in the top row");
                }
                Some(i)
            }
            _ => acc,
        })
        .expect("There should be at least one path tile in the top row");
    let end_x = tiles[height]
        .iter()
        .enumerate()
        .fold(None, |acc, (i, t)| match t {
            &Tile::Path => {
                if acc.is_some() {
                    panic!("There should be only one path tile in the bottom row");
                }
                Some(i)
            }
            _ => acc,
        })
        .expect("There should be at least one path tile in the bottom row");

    // turn the forest into a Directed Acyclical Graph
    let (graph, mut edges) = derive_graph_and_distances(
        &tiles,
        Point { x: start_x, y: 0 },
        Point {
            x: end_x,
            y: height,
        },
    );
    // do a topological sort
    let state = State::from_graph(graph);
    let sorted = topological_sort(state).expect("forest should be sortable");
    // find the shortest path (using negative edge weights)
    edges.values_mut().for_each(|v| {
        v.iter_mut().for_each(|e| {
            *e = e.negate();
        })
    });
    let mut path_lengths: HashMap<Point, i32> = sorted.iter().map(|p| (*p, i32::MAX)).collect();
    *path_lengths
        .get_mut(&Point { x: start_x, y: 0 })
        .expect("start point should be in forest") = 0;
    sorted.iter().take(sorted.len() - 1).for_each(|p| {
        edges
            .get(p)
            .expect("Point must have at least one edge")
            .iter()
            .for_each(|edge| {
                let current_length = *path_lengths.get(&edge.from).expect("edge must exist");
                path_lengths
                    .entry(edge.to)
                    .and_modify(|length| *length = (*length).min(edge.length + current_length));
            })
    });
    let longest_path = path_lengths
        .get(&Point {
            x: end_x,
            y: height,
        })
        .expect("Path to the end must exist");

    // negate the result
    (-longest_path).to_string()
}

pub fn process_part2(input: &str) -> String {
    // it's not pretty and it takes its sweet time, but it gets there
    let (input, tiles) = parse_input(input).unwrap();
    assert!(input.is_empty());
    // get rid of the slopes
    let tiles: Vec<Vec<Tile>> = tiles
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|tile| match tile {
                    Tile::Slope(_) => Tile::Path,
                    other => other,
                })
                .collect()
        })
        .collect();

    let height = tiles.len() - 1;

    let start_x = tiles[0]
        .iter()
        .enumerate()
        .fold(None, |acc, (i, t)| match t {
            &Tile::Path => {
                if acc.is_some() {
                    panic!("There should be only one path tile in the top row");
                }
                Some(i)
            }
            _ => acc,
        })
        .expect("There should be at least one path tile in the top row");
    let end_x = tiles[height]
        .iter()
        .enumerate()
        .fold(None, |acc, (i, t)| match t {
            &Tile::Path => {
                if acc.is_some() {
                    panic!("There should be only one path tile in the bottom row");
                }
                Some(i)
            }
            _ => acc,
        })
        .expect("There should be at least one path tile in the bottom row");

    let (graph, distances) = derive_graph_and_distances_part_2(
        &tiles,
        Point { x: start_x, y: 0 },
        Point {
            x: end_x,
            y: height,
        },
    );

    let visited = HashSet::new();
    let longest = longest_path(
        &graph,
        &distances,
        Point { x: start_x, y: 0 },
        Point {
            x: end_x,
            y: tiles.len() - 1,
        },
        0,
        visited,
    );

    longest.to_string()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum Slope {
    North,
    South,
    East,
    West,
}

impl Slope {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, c) = one_of("<>^v")(input)?;
        let slope = match c {
            '<' => Self::West,
            '>' => Self::East,
            '^' => Self::North,
            'v' => Self::South,
            x => unreachable!("Impossible character: {}", x),
        };
        Ok((input, slope))
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
    Slope(Slope),
}

impl Tile {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, tile) = alt((Self::parse_path_or_forest, Self::parse_slope))(input)?;
        Ok((input, tile))
    }

    fn parse_path_or_forest(input: &str) -> IResult<&str, Self> {
        let (input, c) = one_of("#.")(input)?;
        let tile = match c {
            '#' => Self::Forest,
            '.' => Self::Path,
            x => unreachable!("Impossible character: {}", x),
        };
        Ok((input, tile))
    }

    fn parse_slope(input: &str) -> IResult<&str, Self> {
        let (input, slope) = Slope::parse(input)?;
        Ok((input, Self::Slope(slope)))
    }
}

type Graph<T> = HashMap<T, HashSet<T>>;

#[derive(Debug)]
struct State<T>
where
    T: Copy + Eq + Hash,
{
    dependents: Graph<T>,
    dependencies: Graph<T>,
    no_deps: VecDeque<T>,
}

#[allow(dead_code)]
impl<T> State<T>
where
    T: Copy + Eq + Hash + Debug,
{
    fn from_graph(graph: Graph<T>) -> Self {
        let dependents = graph;

        let mut dependencies = HashMap::new();
        dependents.iter().for_each(|(dependency, dependents)| {
            dependents.iter().for_each(|dependent| {
                dependencies
                    .entry(*dependent)
                    .or_insert_with(HashSet::new)
                    .insert(*dependency);
            });
        });
        let no_deps = dependencies
            .iter()
            .flat_map(|(_, v)| v.iter())
            .filter(|&k| !dependencies.contains_key(k))
            .cloned()
            .collect();

        Self {
            dependents,
            dependencies,
            no_deps,
        }
    }

    fn get_dependents(&self, dependency: &T) -> Option<&HashSet<T>> {
        self.dependents.get(dependency)
    }

    fn is_resolved(&self) -> bool {
        self.dependencies.is_empty()
    }

    fn resolve(&mut self, dependent: &T, dependency: &T) {
        // depedency = from, dependent = to
        if let Some(dpndncies) = self.dependencies.get_mut(dependent) {
            dpndncies.remove(dependency);
            if dpndncies.is_empty() {
                self.dependencies.remove(dependent);
                self.no_deps.push_back(*dependent);
            }
        }
    }

    fn unresolved(&self) -> impl Iterator<Item = &T> {
        self.dependencies.keys()
    }
}

#[derive(Debug)]
enum SortingError {
    GraphIsCyclical,
}

impl Display for SortingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortingError::GraphIsCyclical => {
                f.write_str("Graph is cyclical, topological sort cannot be completed")
            }
        }
    }
}

impl Error for SortingError {}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Edge {
    from: Point,
    to: Point,
    length: i32,
}

impl Edge {
    fn negate(&self) -> Self {
        Self {
            from: self.from,
            to: self.to,
            length: -self.length,
        }
    }
}

/// for Dijkstra
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Node {
    point: Point,
    value: usize,
}

impl Node {
    fn neighbours(&self, tiles: &[Vec<Tile>], visited: &[Vec<bool>]) -> Vec<Self> {
        let &Self {
            point: Point { x, y },
            value,
        } = self;
        match &tiles[y][x] {
            Tile::Path => {
                let mut neighbours = Vec::new();
                if x > 0
                    && !visited[y][x - 1]
                    && !matches!(tiles[y][x - 1], Tile::Forest | Tile::Slope(Slope::East))
                {
                    neighbours.push(Node {
                        point: Point { x: x - 1, y },
                        value: value + 1,
                    });
                }
                if x < (visited[0].len() - 1)
                    && !visited[y][x + 1]
                    && !matches!(tiles[y][x + 1], Tile::Forest | Tile::Slope(Slope::West))
                {
                    neighbours.push(Node {
                        point: Point { x: x + 1, y },
                        value: value + 1,
                    });
                }
                if y > 0
                    && !visited[y - 1][x]
                    && !matches!(tiles[y - 1][x], Tile::Forest | Tile::Slope(Slope::South))
                {
                    neighbours.push(Node {
                        point: Point { x, y: y - 1 },
                        value: value + 1,
                    });
                }
                if y < (visited.len() - 1)
                    && !visited[y + 1][x]
                    && !matches!(tiles[y + 1][x], Tile::Forest | Tile::Slope(Slope::North))
                {
                    neighbours.push(Node {
                        point: Point { x, y: y + 1 },
                        value: value + 1,
                    });
                }
                // eprintln!("Neighbours of ({x}, {y}) are: {:?}", &neighbours);

                neighbours
            }
            Tile::Forest => Vec::new(),
            Tile::Slope(slope) => match slope {
                Slope::North => vec![Self {
                    point: Point { x, y: y - 1 },
                    value: value + 1,
                }],
                Slope::South => vec![Self {
                    point: Point { x, y: y + 1 },
                    value: value + 1,
                }],
                Slope::East => vec![Self {
                    point: Point { x: x + 1, y },
                    value: value + 1,
                }],
                Slope::West => vec![Self {
                    point: Point { x: x - 1, y },
                    value: value + 1,
                }],
            },
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match other.value.partial_cmp(&self.value) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        other.point.partial_cmp(&self.point)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("self.value implements Ord")
    }
}

#[allow(dead_code)]
fn print_graph(graph: &Graph<Point>) {
    graph.iter().for_each(|(k, v)| {
        print!("({}, {}) -> [", k.x, k.y);
        let mut it = v.iter().peekable();
        while let Some(p) = it.next() {
            print!("({}, {})", p.x, p.y);
            if it.peek().is_some() {
                print!(", ");
            }
        }
        println!("]");
    })
}

fn reset_visited(visited: &mut [Vec<bool>]) {
    visited.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|v| {
            *v = false;
        })
    })
}

fn derive_graph_and_distances(
    tiles: &[Vec<Tile>],
    start: Point,
    end: Point,
) -> (Graph<Point>, HashMap<Point, Vec<Edge>>) {
    let width = tiles[0].len();
    let height = tiles.len();
    let mut vertex_queue = VecDeque::new();
    vertex_queue.push_back(start);
    let mut edges = HashMap::new();
    let mut visited = vec![vec![false; width]; height];
    let mut dijkstra_queue = BinaryHeap::new();
    while let Some(start_point) = vertex_queue.pop_front() {
        let Point { x, y } = start_point;
        visited[y][x] = true;
        let start_node = Node {
            point: start_point,
            value: 0,
        };
        assert!(dijkstra_queue.is_empty());
        dijkstra_queue.push(start_node);
        reset_visited(visited.as_mut_slice());
        while let Some(node) = dijkstra_queue.pop() {
            visited[node.point.y][node.point.x] = true;
            for neighbour in node.neighbours(tiles, &visited) {
                if neighbour.point == end {
                    edges
                        .entry(start_point)
                        .or_insert_with(Vec::new)
                        .push(Edge {
                            from: start_point,
                            to: end,
                            length: neighbour.value as i32,
                        })
                }
                match tiles[neighbour.point.y][neighbour.point.x] {
                    Tile::Path => dijkstra_queue.push(neighbour),
                    Tile::Forest => unreachable!("Node.neighbours should not return Forest tiles"),
                    Tile::Slope(_) => {
                        vertex_queue.push_back(neighbour.point);
                        edges
                            .entry(start_point)
                            .or_insert_with(Vec::new)
                            .push(Edge {
                                from: start_point,
                                to: neighbour.point,
                                length: neighbour.value as i32,
                            });
                    }
                }
            }
        }
    }

    let mut graph = HashMap::new();
    edges.iter().for_each(|(_, es)| {
        es.iter().for_each(|e| {
            add_edge(&mut graph, e.from, e.to);
        })
    });

    (graph, edges)
}

fn derive_graph_and_distances_part_2(
    tiles: &[Vec<Tile>],
    start: Point,
    end: Point,
) -> (Graph<Point>, HashMap<(Point, Point), i32>) {
    // dijkstra but create node when at split
    let width = tiles[0].len();
    let height = tiles.len();
    let mut vertex_queue = VecDeque::new();
    // (origin, start_point)
    vertex_queue.push_back((start, start));
    let mut edges = HashMap::new();
    let mut visited = vec![vec![false; width]; height];
    let mut dijkstra_queue = BinaryHeap::new();
    while let Some((origin, start_point)) = vertex_queue.pop_front() {
        let Point { x, y } = start_point;
        visited[y][x] = true;
        let value = if origin == start_point { 0 } else { 1 };
        let start_node = Node {
            point: start_point,
            value,
        };
        assert!(dijkstra_queue.is_empty());
        dijkstra_queue.push(start_node);

        while let Some(node) = dijkstra_queue.pop() {
            visited[node.point.y][node.point.x] = true;
            let neighbours = node.neighbours(tiles, &visited);
            if neighbours.len() == 1 {
                let neighbour = neighbours[0];
                if neighbour.point == end {
                    edges.insert((origin, end), neighbour.value as i32);
                    edges.insert((end, origin), neighbour.value as i32);
                } else {
                    dijkstra_queue.push(neighbour);
                }
            } else {
                // we're at a split, end the current path here and create new ones
                edges.insert((origin, node.point), node.value as i32);
                edges.insert((node.point, origin), node.value as i32);
                vertex_queue.extend(neighbours.into_iter().map(|n| (node.point, n.point)));
            }
        }
    }

    let mut graph = HashMap::new();
    edges
        .keys()
        .for_each(|(from, to)| add_edge(&mut graph, *from, *to));

    (graph, edges)
}

fn add_edge<T>(graph: &mut Graph<T>, from: T, to: T)
where
    T: Copy + Eq + Hash,
{
    graph.entry(from).or_insert_with(HashSet::new).insert(to);
}

fn topological_sort<T>(state: State<T>) -> Result<Vec<T>, SortingError>
where
    T: Copy + Eq + Hash + Debug,
{
    let mut sorted = Vec::new();
    let mut state = state;
    while let Some(node) = state.no_deps.pop_front() {
        sorted.push(node);
        if let Some(dependents) = state.dependents.get(&node) {
            // which node depend on this node?
            for dependent in dependents.clone() {
                state.resolve(&dependent, &node)
            }
        }
    }
    if !state.is_resolved() {
        Err(SortingError::GraphIsCyclical)
    } else {
        Ok(sorted)
    }
}

fn longest_path<T>(
    graph: &Graph<T>,
    distances: &HashMap<(T, T), i32>,
    start: T,
    end: T,
    distance_so_far: i32,
    visited: HashSet<T>,
) -> i32
where
    T: Eq + Copy + Hash,
{
    fn possible_next_steps<'a, 'b, T>(
        graph: &'a Graph<T>,
        node: T,
        visited: &'b HashSet<T>,
    ) -> impl Iterator<Item = &'b T>
    where
        T: Eq + Copy + Hash,
        'a: 'b,
    {
        if let Some(v) = graph.get(&node) {
            v.iter().filter(|&n| !visited.contains(n))
        } else {
            unreachable!("node must exist in graph")
        }
    }

    if let Some(&distance) = distances.get(&(start, end)) {
        distance_so_far + distance
    } else {
        let mut next_visited = visited.clone();
        next_visited.insert(start);
        possible_next_steps(graph, start, &visited)
            .map(|&next_node| {
                let step = *distances
                    .get(&(start, next_node))
                    .expect("this is definitely an edge");
                longest_path(
                    graph,
                    distances,
                    next_node,
                    end,
                    distance_so_far + step,
                    next_visited.clone(),
                )
            })
            .max()
            .or(Some(0))
            .expect("duh")
    }
}

type Line<'a> = Vec<Tile>;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = many1(Tile::parse)(input)?;
    Ok((input, line))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        let result = process_part1(input);
        assert_eq!(result, "94");
    }

    #[test]
    fn part2() {
        let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        let result = process_part2(input);
        assert_eq!(result, "154");
    }
}
