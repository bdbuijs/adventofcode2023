use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, space1},
    multi::separated_list1,
    sequence::terminated,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, named_nodes) = parse_input(input).unwrap();
    debug_assert!(input.is_empty());
    // named_nodes.iter().for_each(|n| n.mermaid());
    let mut graph: Graph = named_nodes.into();
    let solution = graph.solve();
    solution.to_string()
}

pub fn process_part2(_input: &str) -> String {
    "Merry Christmas!".to_string()
}

struct NamedNode<'a> {
    name: &'a str,
    connections: Vec<&'a str>,
}

#[derive(Debug, Clone)]
struct Edge {
    node: usize,
    weight: usize,
}

impl<'a> NamedNode<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, name) = terminated(alpha1, tag(": "))(input)?;
        let (input, connections) = separated_list1(space1, alpha1)(input)?;
        Ok((input, Self { name, connections }))
    }

    #[allow(dead_code)]
    fn mermaid(&self) {
        self.connections.iter().for_each(|&c| {
            println!("{} --- {}", self.name, c);
        });
    }
}

#[derive(Debug, Clone)]
struct Graph {
    nodes: Vec<Vec<Edge>>,
    node_count: usize,
}

impl Graph {
    fn solve(&mut self) -> usize {
        let mut cuts = Vec::new();
        while self.node_count > 2 {
            let cut = self.sw_phase();
            self.contract_nodes(&cut);
            cuts.push(cut);
        }
        let minimum_cut = cuts
            .iter()
            .min_by(|a, b| a.weight.cmp(&b.weight))
            .expect("There is a minimum cut")
            .clone();

        // let's see if we get lucky and can do this without DFS
        let mut dfs_graph = DFSGraph::new();
        cuts.iter()
            .take_while(|c| **c != minimum_cut)
            .for_each(|c| dfs_graph.add_edge(c.a, c.b));
        let component_size = if dfs_graph.nodes.contains_key(&minimum_cut.a) {
            dfs_graph.dfs(minimum_cut.a)
        } else {
            assert!(dfs_graph.nodes.contains_key(&minimum_cut.b));
            dfs_graph.dfs(minimum_cut.b)
        };
        (self.nodes.len() - component_size) * component_size
    }

    /// Runs one phase of Stoer-Wagner algorithm and returns the cut to be made
    fn sw_phase(&mut self) -> Cut {
        let len = self.nodes.len();
        let mut last_node = 0;
        let mut second_last_node = 0;
        let mut cut_set = vec![false; len];
        let mut weights: Vec<usize> = vec![0; len];
        for _ in 0..self.node_count {
            let next_node = if let Some((i, _)) = weights
                .iter()
                .enumerate()
                .filter(|&(i, w)| w > &0 && !cut_set[i])
                .max_by(|(_, a), (_, b)| a.cmp(b))
            {
                i
            } else {
                // first time around we pick the first node that is still in the graph
                self.nodes
                    .iter()
                    .enumerate()
                    .find(|(_, v)| !v.is_empty())
                    .expect("There is at least one node")
                    .0
            };
            cut_set[next_node] = true;
            self.nodes[next_node]
                .iter()
                .filter(|e| !cut_set[e.node])
                .for_each(|e| weights[e.node] += e.weight);

            (second_last_node, last_node) = (last_node, next_node);
        }
        debug_assert!(cut_set.iter().filter(|&&b| b).count() == self.node_count);
        Cut {
            a: second_last_node,
            b: last_node,
            weight: weights[last_node],
        }
    }

    fn contract_nodes(&mut self, cut: &Cut) {
        debug_assert!(cut.a < self.nodes.len());
        debug_assert!(cut.b < self.nodes.len());
        debug_assert!(cut.a != cut.b);
        debug_assert!(!self.nodes[cut.a].is_empty());
        debug_assert!(!self.nodes[cut.b].is_empty());
        // merge into the lowest id node
        let (source, destination) = (cut.a.max(cut.b), cut.a.min(cut.b));

        // Take the edges from source and destination
        let source_edges = std::mem::take(&mut self.nodes[source]);
        let destination_edges = std::mem::take(&mut self.nodes[destination]);

        // Remove all edges to the source and destination nodes
        self.nodes.iter_mut().for_each(|neighbours| {
            neighbours.retain(|e| e.node != source && e.node != destination);
        });

        // Merge the edges
        let mut new_edges = source_edges
            .into_iter()
            .filter(|e| e.node != destination)
            .collect::<Vec<_>>();
        destination_edges
            .into_iter()
            .filter(|e| e.node != source)
            .for_each(|destination_edge| {
                if let Some(existing_edge) = new_edges
                    .iter_mut()
                    .find(|new_edge| new_edge.node == destination_edge.node)
                {
                    existing_edge.weight += destination_edge.weight;
                } else {
                    new_edges.push(destination_edge)
                }
            });

        // Put the combined edges back to the destination node
        new_edges.iter().for_each(|new_edge| {
            self.nodes[new_edge.node].push(Edge {
                node: destination,
                weight: new_edge.weight,
            });
        });

        self.nodes[destination] = new_edges;
        self.node_count -= 1;
    }

    #[allow(dead_code)]
    fn mermaid(&self, prefix: Option<&'static str>) {
        let prefix = prefix.unwrap_or_default();
        self.nodes
            .iter()
            .enumerate()
            .flat_map(|(i, v)| v.iter().map(move |e| (i, e)))
            .filter(|&(i, e)| i <= e.node)
            .for_each(|(i, e)| println!("{}{} ---|{}| {}{}", prefix, i, e.weight, prefix, e.node));
    }
}

impl<'a> From<Vec<NamedNode<'a>>> for Graph {
    fn from(value: Vec<NamedNode>) -> Self {
        let node_names: Vec<&str> = value
            .iter()
            .flat_map(|n| n.connections.iter().cloned().chain(std::iter::once(n.name)))
            .collect::<HashSet<&str>>()
            .into_iter()
            .collect();
        let node_count = node_names.len();
        let mut nodes = vec![vec![]; node_count];
        value.iter().for_each(|n| {
            let idx = node_names
                .iter()
                .position(|&e| e == n.name)
                .expect("Node exists");
            n.connections.iter().for_each(|&c| {
                let node = node_names
                    .iter()
                    .position(|&e| e == c)
                    .expect("Node exists");
                nodes[idx].push(Edge { node, weight: 1 });
                nodes[node].push(Edge {
                    node: idx,
                    weight: 1,
                })
            });
        });
        Self { nodes, node_count }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cut {
    a: usize,
    b: usize,
    weight: usize,
}

struct DFSGraph {
    nodes: HashMap<usize, Vec<usize>>,
}

impl DFSGraph {
    fn new() -> Self {
        Self {
            nodes: Default::default(),
        }
    }
    fn add_edge(&mut self, a: usize, b: usize) {
        self.nodes.entry(a).or_default().push(b);
        self.nodes.entry(b).or_default().push(a);
    }

    /// returns the size of the component connected to start
    fn dfs(&mut self, start: usize) -> usize {
        let mut visited = HashSet::new();
        self.dfs_rec(start, &mut visited);

        visited.len()
    }

    fn dfs_rec(&self, node: usize, visited: &mut HashSet<usize>) {
        if visited.contains(&node) {
            return;
        }
        visited.insert(node);
        if let Some(neighbours) = self.nodes.get(&node) {
            neighbours.iter().for_each(|&n| self.dfs_rec(n, visited));
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<NamedNode>> {
    let (input, lines) = separated_list1(newline, NamedNode::parse)(input)?;
    Ok((input, lines))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
        let result = process_part1(input);
        assert_eq!(result, "54");
    }
}
