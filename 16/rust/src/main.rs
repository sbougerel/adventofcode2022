#[allow(unused_imports)]
use petgraph::{
    dot::Dot,
    graph::{Graph, NodeIndex, UnGraph},
    Undirected,
};
use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;
use std::fmt::Display;
use std::io;

// This problem can be conceptualised as a backtracking algorithms. In game
// theory, backtracking algorithms is the building block to minmax. The way to
// understand how it works is to ask: "Given a particular state in the game,
// what is my next best move?", or in this particular problem, given a current
// position a set of open valves, what is the next move the maximizes the amount
// of pressure that can be released?
//
// Backtracking algorithm compute the solution to this answer by starting from
// the deepest possible move and working backward. By working backwards from the
// tree of all possible moves, the algorithm eventually builds up to the best
// move starting from the root position.
//
// In this solution, we also add a transposition table, which stores the best
// pressure that we've seen so far for a given position (at), past valves
// visited, time and number of players remaining. This saves _a lot_ of
// computation cycles.
//
// We could go even further by using the transposition table to reconstruct the
// best set of moves (also called Principal variation), however it would require
// that we change Entry and it's Hashing function. Maybe I'll come back to it if
// I want to refactor this further.
//
// This should compute solutions in a matter of seconds.
//
// Resources:
// - https://en.wikipedia.org/wiki/Backtracking
// - https://en.wikipedia.org/wiki/Transposition_table

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

#[derive(Clone, PartialEq, Eq, Hash)]
struct Entry {
    // Entry in the transposition table, is represented by:
    // - time
    // - current position
    // - number of players (part 2)
    // - past visited (and opened) valves compressed to 64bit vector.
    //
    // Transposition tables entry take most of the memory of the program, so to
    // make it more compact, I used a 64 bit vector to represent set of opened
    // valves. I could compress `time_left` and `at` further into `i16` each,
    // but not really worth the complexity.
    //
    // It can't store more than 64 different valves; but there's a lot of room
    // since the graph is reduced to valves with flow_rates > 0 only (save for
    // "AA")
    time_left: i32,
    at: NodeIndex,
    players: usize,
    valve_bitset: u64,
}

impl Entry {
    fn from_state(path: &Vec<NodeIndex>, time_left: i32, players: usize) -> Entry {
        let mut valve_bitset: u64 = 0;
        for valve in path {
            valve_bitset |= 1 << valve.index();
        }
        Entry {
            time_left,
            at: path[path.len() - 1],
            players,
            valve_bitset,
        }
    }
}

// Transposition table maps an Entry to a max pressure (i32). We store only
// exact solutions here.
type TTable = HashMap<Entry, i32>;

type ValveNetwork = Graph<Valve, i32, Undirected>;

struct Solver<'a> {
    graph: &'a ValveNetwork,
    ttable: &'a mut TTable,
    max_time: i32,
    start: NodeIndex,
}

impl Solver<'_> {
    fn max_pressure(&mut self) -> i32 {
        self.max_pressure_multiplayer(1)
    }

    fn max_pressure_multiplayer(&mut self, players: usize) -> i32 {
        let mut path = Vec::<NodeIndex>::new();
        // _max_pressure_impl will initialize variables from &self if we give a
        // remaining time value of 0.
        self._max_pressure_impl(&mut path, 0, players)
    }

    fn _max_pressure_impl(
        &mut self,
        path: &mut Vec<NodeIndex>,
        time_left: i32,
        players: usize,
    ) -> i32 {
        if time_left <= 0 {
            if players == 0 {
                return 0;
            }
            // Make next player play from start, but skip already visited nodes
            path.push(self.start);
            let pressure = self._max_pressure_impl(path, self.max_time, players - 1);
            path.pop();
            return pressure;
        }

        // Check transposition tables for known exact entry
        let entry = Entry::from_state(path, time_left, players);
        if let Some(pressure) = self.ttable.get(&entry) {
            return *pressure;
        }

        let at = path[path.len() - 1];
        let mut pressure = 0;

        for next in self.graph.node_indices() {
            if path.contains(&next) {
                continue;
            }
            path.push(next);
            pressure = max(
                pressure,
                self._max_pressure_impl(
                    path,
                    // Open valve here before moving down
                    time_left - 1 - self.graph[self.graph.find_edge(at, next).unwrap()],
                    players,
                ),
            );
            path.pop(); // restore state
        }

        // Store transposition
        pressure += time_left * self.graph[at].flow_rate;
        self.ttable.insert(entry, pressure);
        pressure
    }
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
    let mut graph = ValveNetwork::default();
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
    // then remove then (saves cycles for next step)
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

    // Finally, fully-connect the graph (save cycles later) with minimum weights, so
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

    // Optional tree print if you want to see what it looks like:
    // println!("{:?}", Dot::with_config(&graph, &[]));

    // Time to compute the solution!
    //
    // Initialise the transposition table (empty) and path (state of the
    // exploration) then run the solution!

    // Initial conditions
    let start = graph
        .node_indices()
        .find(|n| graph[*n].name == "AA")
        .unwrap();
    let mut ttable = TTable::new();
    let mut solver = Solver {
        graph: &graph,
        ttable: &mut ttable,
        max_time: 30,
        start,
    };

    // Part 1.
    //
    // Prep is done, time to compute some permutations and valve rates! We
    // simply generate the full combinatorial sequence while maintaining the
    // best one starting from AA and never exceeding 30 minutes (valve opening
    // included)
    println!("Part 1: max pressure released: {}", solver.max_pressure());

    // Part 2.
    //
    // Here we simply alternate between player 1 and player 2, if you will,
    // knowing that the time remaining is always based on what they do
    // separately, as if player 2 (elephant) only played when player 1 had
    // exhausted his time.
    solver.max_time = 26;
    println!(
        "Part 2: max pressure released with 2 players: {}",
        solver.max_pressure_multiplayer(2)
    );
}
