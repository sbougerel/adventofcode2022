use ndarray::Array2;
use rudac::heap::FibonacciHeap;
use std::cmp::Ordering;

// From https://doc.rust-lang.org/std/collections/binary_heap/index.html
// which has a straight up Dijskstra implementation.

pub type Pos = [usize; 2];
pub type HeightMap = Array2<usize>;

// Alternatively, I could have created my own type to implement some traits
type Coord = [isize; 2];

fn into_coord(pos: Pos) -> Coord {
    [
        isize::try_from(pos[0]).unwrap(),
        isize::try_from(pos[1]).unwrap(),
    ]
}

fn into_pos(coord: Coord) -> Pos {
    [
        usize::try_from(coord[0]).unwrap(),
        usize::try_from(coord[1]).unwrap(),
    ]
}

fn add(x: Coord, y: Coord) -> Coord {
    [x[0] + y[0], x[1] + y[1]]
}

fn diff(x: Coord, y: Coord) -> Coord {
    [x[0] - y[0], x[1] - y[1]]
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    coord: Coord,
}

// The priority queue depends on `Ord`. Explicitly implement the trait so the
// queue becomes a min-heap instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost
            .cmp(&other.cost)
            // Use taxicab metric for distance d = d|x| + d|y|
            .then_with(|| {
                (&self.coord[0].abs() + &self.coord[1].abs())
                    .cmp(&(other.coord[0].abs() + other.coord[1].abs()))
            })
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct MoveIterator {
    pos: Pos,
    bounds: Pos,
    index: u8,
}

impl Iterator for MoveIterator {
    type Item = Pos;
    fn next(&mut self) -> Option<Pos> {
        loop {
            self.index += 1;
            match self.index {
                1 => {
                    if self.pos[1] < self.bounds[1] - 1 {
                        return Some([self.pos[0], self.pos[1] + 1]);
                    }
                }
                2 => {
                    if self.pos[0] < self.bounds[0] - 1 {
                        return Some([self.pos[0] + 1, self.pos[1]]);
                    }
                }
                3 => {
                    if self.pos[1] > 0 {
                        return Some([self.pos[0], self.pos[1] - 1]);
                    }
                }
                4 => {
                    if self.pos[0] > 0 {
                        return Some([self.pos[0] - 1, self.pos[1]]);
                    }
                }
                _ => return None,
            }
        }
    }
}

fn iterate_moves(pos: Pos, bounds: Pos) -> MoveIterator {
    MoveIterator {
        pos: pos,
        bounds: bounds,
        index: 0,
    }
}

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
pub fn shortest_path(heightmap: HeightMap, starts: Vec<Pos>, end: Pos) -> Option<usize> {
    let bounds = heightmap.raw_dim();
    let mut heap = FibonacciHeap::init_min();
    let mut bestmap = HeightMap::from_elem(bounds, usize::MAX);

    for start in starts {
        bestmap[start] = 0;
        heap.push(State {
            cost: 0,
            coord: diff(into_coord(start), into_coord(end)),
        });
    }

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, coord }) = heap.pop() {
        let pos = into_pos(add(coord, into_coord(end)));
        // println!("Explore ({}, {})", pos[0], pos[1]);

        if pos == end {
            return Some(cost); // Goal!
        }

        if bestmap[pos] < cost {
            continue;
        }

        for next_pos in iterate_moves(pos, [bounds[0], bounds[1]]) {
            // Out of reach (too high)
            if heightmap[pos] + 1 < heightmap[next_pos] {
                continue;
            }

            // Already seen equivalent or better
            if bestmap[next_pos] <= (cost + 1) {
                continue;
            }

            // println!("Keep ({}, {})", next_pos[0], next_pos[1]);
            bestmap[next_pos] = cost + 1;
            heap.push(State {
                cost: cost + 1,
                coord: diff(into_coord(next_pos), into_coord(end)),
            });
        }
    }

    // Goal not reacheable
    None
}
