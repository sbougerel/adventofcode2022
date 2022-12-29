use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::io;

// Wao, that was tough, and I didn't solve it alone, to be frank.
//
// This is another kind of backtracking algorithm; in this case however, you can
// make several improvements on it, like added a lower_bound heuristic, which
// helps the algorithm to prune a lot of information.
//
// I initially decided to go with building a heuristic to avoid building bots:
// it was a noticeable speed up, but not _that_ significant. The major speed up
// was when I integrated an idea from [Jonhathan
// Paulson](https://www.youtube.com/channel/UCuWLIm0l4sDpEe28t41WITA):
//
//  - You do not need to build more robots than the spend rate
//
// This was a significant increase caused it caused a lot more pruning than my
// initial heuristic.
//
// To get the maximum speed up that I could I added a memoisation too (see the
// transposition table in the source). But it didn't turn out to be as
// significant as I expected (comparable to the first speed up).
//

#[non_exhaustive]
struct Robots;
impl Robots {
    const ORE: usize = 0;
    const CLAY: usize = 1;
    const OBSIDIAN: usize = 2;
    const GEODE: usize = 3;
    const ALL: usize = 4;
}

#[non_exhaustive]
struct Res;
impl Res {
    const ORE: usize = 0;
    const CLAY: usize = 1;
    const OBSIDIAN: usize = 2;
    const GEODE: usize = 3;
    const BLUEPRINT: usize = 3;
}

#[derive(Clone, Debug)]
struct Blueprint {
    // matrix:
    // row: Robots,
    // col: Resources needed to build them,
    id: usize,
    robots: [[usize; 3]; 4],
    max_spend_rate: [usize; 3],
}

impl Blueprint {
    fn from_capture(cap: &(usize, usize, usize, usize, usize, usize, usize)) -> Blueprint {
        Blueprint {
            id: cap.0,
            robots: [
                [cap.1, 0, 0],
                [cap.2, 0, 0],
                [cap.3, cap.4, 0],
                [cap.5, 0, cap.6],
            ],
            max_spend_rate: [max(cap.1, max(cap.2, max(cap.3, cap.5))), cap.4, cap.6],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    res: [usize; 4],
    robots: [usize; 4],
}

impl Default for State {
    fn default() -> State {
        State {
            res: [0, 0, 0, 0],
            robots: [1, 0, 0, 0],
        }
    }
}

impl State {
    fn harvest_time(&self, blueprint: &Blueprint, robot: usize) -> i32 {
        let mut harvest_time: i32 = 0;
        for res in 0..Res::BLUEPRINT {
            if blueprint.robots[robot][res] == 0 {
                continue;
            }
            harvest_time = max(
                harvest_time,
                (((blueprint.robots[robot][res] as i32) - (self.res[res] as i32))
                 + (self.robots[res] as i32) - 1) // force round up
                    / (self.robots[res] as i32),
            )
        }
        harvest_time
    }

    fn build(&mut self, blueprint: &Blueprint, robot: usize) {
        for res in 0..Res::BLUEPRINT {
            self.res[res] -= blueprint.robots[robot][res];
        }
        self.robots[robot] += 1;
    }

    fn cancel_build(&mut self, blueprint: &Blueprint, robot: usize) {
        self.robots[robot] -= 1;
        for res in 0..Res::BLUEPRINT {
            self.res[res] += blueprint.robots[robot][res];
        }
    }

    fn harvest(&mut self, time: i32) {
        assert!(time >= 0);
        for i in 0..Robots::ALL {
            self.res[i] += self.robots[i] * (time as usize);
        }
    }

    fn cancel_harvest(&mut self, time: i32) {
        assert!(time >= 0);
        for i in 0..Robots::ALL {
            self.res[i] -= self.robots[i] * (time as usize);
        }
    }

    fn tech_level(&self) -> usize {
        for robot in [Robots::GEODE, Robots::OBSIDIAN, Robots::CLAY] {
            if self.robots[robot] > 0 {
                return robot;
            }
        }
        Robots::ORE
    }
}

impl State {
    fn builds(&self) -> Builds {
        Builds {
            index: Robots::ALL,
            tech_level: self.tech_level(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Entry {
    // Entry in the transposition table
    state: State,
    time_left: i32,
}

impl Entry {
    fn from_state(blueprint: &Blueprint, state: &State, time_left: i32) -> Entry {
        // We can discard any exceess resources to increase state collision:
        // Excess resources are when `resources > (time_left + 1) * spend_rate`
        let mut stored_state = state.clone();
        for res in 0..Res::BLUEPRINT {
            stored_state.res[res] = min(
                stored_state.res[res],
                ((time_left as usize) + 1) * blueprint.max_spend_rate[res],
            );
        }
        Entry {
            time_left,
            state: stored_state,
        }
    }
}

// Transposition table maps an Entry to a max pressure (i32). We store only
// exact solutions here.
type TTable = HashMap<Entry, usize>;

struct BlueprintSolver<'a> {
    blueprint: &'a Blueprint,
    t_table: TTable,
    tt_hits: usize,
}

impl BlueprintSolver<'_> {
    fn with(blueprint: &Blueprint) -> BlueprintSolver {
        BlueprintSolver {
            blueprint,
            t_table: TTable::default(),
            tt_hits: 0,
        }
    }
}

struct Solver {
    blueprints: Vec<Blueprint>,
}

struct Builds {
    index: usize,
    tech_level: usize,
}

impl Iterator for Builds {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        while self.index > 0 {
            self.index -= 1;
            if self.index == Robots::GEODE && self.tech_level >= Robots::OBSIDIAN {
                return Some(self.index);
            }
            if self.index == Robots::OBSIDIAN && self.tech_level >= Robots::CLAY {
                return Some(self.index);
            }
            if self.index == Robots::CLAY {
                return Some(self.index);
            }
            if self.index == Robots::ORE {
                return Some(self.index);
            }
        }
        None
    }
}

// Upper bound function available?
// Given a known - best, we can compute if we can reach it, by multiplying back and counting available resources.
impl Solver {
    fn print_part1(&self, time_left: i32) {
        let mut quality = 0;
        for bp in self.blueprints.iter() {
            let mut bps = BlueprintSolver::with(bp);
            let geodes = bps.max_geodes(&mut State::default(), time_left);

            // Optional display:
            // println!("Blueprint {} cracks at most {} geodes", bp.id, geodes);
            quality += bp.id * geodes;
        }
        println!("Part1: Quality level: {}", quality);
    }

    fn print_part2(&self, time_left: i32) {
        let mut all_geodes = 1;
        for bp in &self.blueprints[..3] {
            let mut bps = BlueprintSolver::with(bp);
            let geodes = bps.max_geodes(&mut State::default(), time_left);

            // Optional display:
            // println!("Blueprint {} cracks at most {} geodes", bp.id, geodes);
            all_geodes *= geodes;
        }
        println!("Part2: All geodes multiplied: {}", all_geodes);
    }
}

impl BlueprintSolver<'_> {
    fn build_heuristic(
        &self,
        state: &mut State,
        mut time_left: i32,
        robot: usize,
        lower_bound: usize,
    ) -> bool {
        // For any robot, is only necessary to build the robot if we could
        // theoritically catch up to the lower_bound in geode.
        //
        // With geode robots, run a quick simulation to check wether we can
        // overtake the lower bound.
        //
        // Other robots are alse capped by the spend_rate of a self.blueprint. They
        // are also capped by their time penatly in the sequence to a geode
        // robot.
        let mut geode_amount = state.res[Res::GEODE]; // start here
        let mut geode_robots = state.robots[Robots::GEODE];
        while time_left > 0 && geode_amount <= lower_bound {
            geode_amount += geode_robots;
            geode_robots += 1;
            time_left -= 1;
        }

        if geode_amount <= lower_bound {
            return false;
        }

        // time to build geode robots has been substracted from remaining time:
        match robot {
            Robots::ORE => state.robots[robot] < self.blueprint.max_spend_rate[Res::ORE],
            Robots::CLAY => {
                // Clay robots take 1 extra step to make any geode robot
                time_left > 0 && state.robots[robot] < self.blueprint.max_spend_rate[Res::CLAY]
            }
            Robots::OBSIDIAN => state.robots[robot] < self.blueprint.max_spend_rate[Res::OBSIDIAN],
            _ => true,
        }
    }

    fn max_geodes(&mut self, state: &mut State, time_left: i32) -> usize {
        if time_left < 0 {
            return 0;
        }
        // Check transposition tables for known exact entry
        let entry = Entry::from_state(self.blueprint, state, time_left);
        if let Some(geode) = self.t_table.get(&entry) {
            self.tt_hits += 1;
            return *geode;
        }

        // try harvesting geode for the rest of time first
        state.harvest(time_left);
        let mut geode = state.res[Res::GEODE];
        state.cancel_harvest(time_left);

        for build_robot in state.builds() {
            if !self.build_heuristic(state, time_left, build_robot, geode) {
                continue;
            }
            let harvest_time = state.harvest_time(self.blueprint, build_robot) + 1;
            state.harvest(harvest_time); // Harvest first, build complete after
            state.build(self.blueprint, build_robot);
            geode = max(geode, self.max_geodes(state, time_left - harvest_time));
            state.cancel_build(self.blueprint, build_robot);
            state.cancel_harvest(harvest_time);
        }

        self.t_table.insert(entry, geode);
        geode
    }
}

fn main() {
    let re: Regex = Regex::new(
          r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.",
    )
    .unwrap();

    let captures: Vec<(usize, usize, usize, usize, usize, usize, usize)> = io::stdin()
        .lines()
        .map(
            |line| -> (usize, usize, usize, usize, usize, usize, usize) {
                let line = line.unwrap();
                let caps = re
                    .captures_iter(&line)
                    .next()
                    .expect("Unexpected format on standard input");
                (
                    caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                    caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                    caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                    caps.get(4).unwrap().as_str().parse::<usize>().unwrap(),
                    caps.get(5).unwrap().as_str().parse::<usize>().unwrap(),
                    caps.get(6).unwrap().as_str().parse::<usize>().unwrap(),
                    caps.get(7).unwrap().as_str().parse::<usize>().unwrap(),
                )
            },
        )
        .collect();

    let blueprints: Vec<Blueprint> = captures.iter().map(Blueprint::from_capture).collect();

    let solver = Solver { blueprints };
    solver.print_part1(24);
    solver.print_part2(32);
}
