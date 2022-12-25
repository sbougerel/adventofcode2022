use itertools::Itertools;
use petgraph::graph::{Graph, NodeIndex, UnGraph};
use petgraph::Undirected;
use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;
use std::fmt::Display;
use std::io;

const MAX_TIME: i32 = 30;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Valve {
    name: String,
    flow_rate: i32,
}

impl Display for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} rate:{}", self.name, self.flow_rate)
    }
}

fn plan(
    graph: &Graph<Valve, i32, Undirected>,
    path: &mut Vec<NodeIndex>,
    time_taken: i32,
    pressure: i32,
) -> (Vec<NodeIndex>, i32, i32) {
    if time_taken > MAX_TIME {
        return (Vec::new(), time_taken, 0); // busted!
    }
    let mut best_path = path.clone();
    let mut best_time = time_taken;
    let mut best_pressure = pressure;

    for neighbor in graph.node_indices() {
        if path.contains(&neighbor) {
            continue;
        }
        let node = path[path.len() - 1];
        path.push(neighbor);
        // Go there (edge weight) and open valve (1)
        let neighbor_time = time_taken + graph[graph.find_edge(node, neighbor).unwrap()] + 1;
        let neighbor_pressure = pressure + (MAX_TIME - neighbor_time) * graph[neighbor].flow_rate;
        let (new_path, new_time, new_pressure) =
            plan(graph, path, neighbor_time, neighbor_pressure);
        path.pop();
        if new_pressure > best_pressure {
            best_pressure = new_pressure;
            best_time = new_time;
            best_path = new_path;
        }
    }
    (best_path, best_time, best_pressure)
}

fn plan2(
    graph: &Graph<Valve, i32, Undirected>,
    paths: &mut [Vec<NodeIndex>; 2],
    times_taken: [i32; 2],
    pressure: i32,
) -> ([Vec<NodeIndex>; 2], [i32; 2], i32) {
    if max(times_taken[0], times_taken[1]) > MAX_TIME {
        return ([Vec::new(), Vec::new()], times_taken, 0); // busted!
    }
    let mut best_pressure = pressure;
    let mut best_times = times_taken;
    let mut best_paths = [paths[0].clone(), paths[1].clone()];

    for neighbor in graph.node_indices() {
        if paths[0].contains(&neighbor) || paths[1].contains(&neighbor) {
            continue;
        }
        // for each neighbor, pick the closest actor
        let actor = usize::from(
            (times_taken[0]
                + graph[graph
                    .find_edge(paths[0][paths[0].len() - 1], neighbor)
                    .unwrap()])
                >= (times_taken[1]
                    + graph[graph
                        .find_edge(paths[1][paths[1].len() - 1], neighbor)
                        .unwrap()]),
        );
        let node = paths[actor][paths[actor].len() - 1];
        paths[actor].push(neighbor);
        // Find the closest to
        // Go there (edge weight) and open valve (1)
        let mut neighbor_times = times_taken;
        neighbor_times[actor] += graph[graph.find_edge(node, neighbor).unwrap()] + 1;
        let neighbor_pressure =
            pressure + (MAX_TIME - neighbor_times[actor]) * graph[neighbor].flow_rate;
        let (new_paths, new_times, new_pressure) =
            plan2(graph, paths, neighbor_times, neighbor_pressure);
        paths[actor].pop();
        if new_pressure > best_pressure {
            best_pressure = new_pressure;
            best_times = new_times;
            best_paths = new_paths;
        }
    }
    (best_paths, best_times, best_pressure)
}

fn main() {
    let re: Regex = Regex::new(
        r"Valve (\w{2}) has flow rate=(\d+); tunnels? leads? to valves? (\w{2}(?:, (\w{2}))*)",
    )
    .unwrap();

    let captures: Vec<(String, i32, String)> = io::stdin()
        .lines()
        .map(|line| -> (String, i32, String) {
            let line = line.unwrap();
            let caps = re
                .captures_iter(&line)
                .next()
                .expect("Unexpected format on standard input");
            (
                caps.get(1).unwrap().as_str().to_string(),
                caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                caps.get(3).unwrap().as_str().to_string(),
            )
        })
        .collect();

    // This graphs contains all nodes (even those with flow rate = 0) with weight = 1
    let mut graph = UnGraph::<Valve, i32>::default();
    {
        // Add vertices (nodes) in a dictionary for construction only
        let node_map = captures
            .iter()
            .map(|capture| {
                (
                    capture.0.as_str(),
                    graph.add_node(Valve {
                        name: capture.0.clone(),
                        flow_rate: capture.1,
                    }),
                )
            })
            .collect::<HashMap<_, _>>();
        // Add edges
        for capture in &captures {
            for neighbor in capture.2.split(", ") {
                let (a, b) = (node_map[capture.0.as_str()], node_map[neighbor]);
                if !graph.contains_edge(a, b) {
                    graph.add_edge(a, b, 1);
                }
            }
        }
    }

    // Bypass nodes with "rate=0" unless it's "AA" by connecting their neighbor
    // then remove then (saves cycles later)
    while let Some(node) = graph
        .node_indices()
        .find(|node| graph[*node].name != "AA" && graph[*node].flow_rate == 0)
    {
        let mut neighbors_queue = graph.neighbors(node).collect::<Vec<_>>();
        while let Some(neighbor) = neighbors_queue.pop() {
            for other in neighbors_queue.iter() {
                if !graph.contains_edge(neighbor, *other) {
                    graph.add_edge(
                        neighbor,
                        *other,
                        graph[graph.find_edge(neighbor, node).unwrap()]
                            + graph[graph.find_edge(*other, node).unwrap()],
                    );
                }
            }
        }
        graph.remove_node(node);
    }

    // Now fully-connect the graph (save cycles later) with minimum weights, so
    // we know how long it takes to go from any valve to any other.
    for node in graph.node_indices() {
        let mut neighbors_queue = graph.neighbors(node).collect::<Vec<_>>();
        while let Some(neighbor) = neighbors_queue.pop() {
            for other in neighbors_queue.iter() {
                if !graph.contains_edge(neighbor, *other) {
                    graph.add_edge(
                        neighbor,
                        *other,
                        graph[graph.find_edge(neighbor, node).unwrap()]
                            + graph[graph.find_edge(*other, node).unwrap()],
                    );
                }
            }
        }
    }

    // Currently, part2 runs pretty slowly for the input (5-10 minutes).
    // Considerations to speed up the algorithm:
    //
    // TODO: Use a much simpler permutation algorithm; like Heap's algorithm
    // (iterative version) to eliminate copying with recursion. Prune
    // permutations by time; as done here.

    // Initial conditions
    let start = vec![graph
        .node_indices()
        .find(|n| graph[*n].name == "AA")
        .unwrap()];

    // Part 1.
    //
    // Prep is done, time to compute some permutations and valve rates!
    // We simply generate the full combinatorial sequence while maintaining the best one
    // starting from AA and never exceeding 30 minutes (valve opening included)
    let (best_path, best_time, best_pressure) = plan(&graph, &mut start.clone(), 0, 0);
    println!(
        "Most pressure released: {} in {} minutes by opening valves {}",
        best_pressure,
        best_time,
        best_path
            .iter()
            .map(|n| { graph[*n].name.as_str() })
            .join(", ")
    );

    // Part 2.
    //
    // We simply track 2 different time; plans become "rough" since they might
    // not exactly be executed in the same order as proposed.
    let (best_paths, best_times, best_pressure) =
        plan2(&graph, &mut [start.clone(), start.clone()], [4, 4], 0);
    println!(
        "Most pressure released: {} in [{}, {}] minutes by opening valves [{}], [{}]",
        best_pressure,
        best_times[0],
        best_times[1],
        best_paths[0]
            .iter()
            .map(|n| { graph[*n].name.as_str() })
            .join(", "),
        best_paths[1]
            .iter()
            .map(|n| { graph[*n].name.as_str() })
            .join(", ")
    );
}