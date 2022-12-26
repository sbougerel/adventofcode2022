#[allow(unused_imports)]
use std::cmp::{max, min};
use std::collections::HashMap;
use std::env;
use std::io::{self, Read};

#[derive(Copy, Clone)]
struct Shape {
    width: usize,
    height: usize,
    stensil: u32, // Lsb-stye (mental inversion)
}

enum Gust {
    Left,
    Right,
}

const SHAPES: [Shape; 5] = [
    Shape {
        width: 4,
        height: 1,
        stensil: 0b_0000_0000_0000_0000_0000_0000_0000_1111,
    },
    Shape {
        width: 3,
        height: 3,
        stensil: 0b_0000_0000_0000_0010_0000_0111_0000_0010,
    },
    Shape {
        width: 3,
        height: 3,
        stensil: 0b_0000_0000_0000_0100_0000_0100_0000_0111,
    },
    Shape {
        width: 1,
        height: 4,
        stensil: 0b_0000_0001_0000_0001_0000_0001_0000_0001,
    },
    Shape {
        width: 2,
        height: 2,
        stensil: 0b_0000_0000_0000_0000_0000_0011_0000_0011,
    },
];
const SHAPE_START: usize = 2;
const SHAPE_GAP: usize = 3;
const SHAPE_MAX_HEIGHT: usize = 4;

type Chamber = Vec<u8>;
const TETRIS: u8 = 0b_0111_1111;
const CHAMBER_WIDTH: usize = 7;

#[allow(dead_code)]
fn print_chamber(chamber: &Chamber) {
    for (id, line) in chamber.iter().enumerate().rev() {
        print!("|");
        for x in 0..CHAMBER_WIDTH {
            match line & (1 << x) != 0 {
                true => print!("#"),
                false => print!("."),
            }
        }
        print!("| {}", id);
        println!();
    }
}

#[allow(dead_code)]
fn print_chamber_number(chamber: &Chamber, last_blockade: &u64) {
    for (id, line) in chamber.iter().enumerate().rev() {
        print!("{:6} |", id);
        for x in 0..CHAMBER_WIDTH {
            match line & (1 << x) != 0 {
                true => print!("#"),
                false => print!("."),
            }
        }
        print!("| {}", last_blockade + (id as u64));
        println!();
    }
}

fn colliding(chamber: &[u8], stensil: &u32) -> bool {
    let section: u32 = (chamber[3] as u32) << 24
        | (chamber[2] as u32) << 16
        | (chamber[1] as u32) << 8
        | (chamber[0] as u32);
    section & stensil != 0
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash)]
struct Pattern {
    gush: usize,
    section: [u8; 12],
    shape: u8,
}

impl Pattern {
    fn new(gush: usize, section: &[u8], shape: usize) -> Pattern {
        let mut copy_section = [0u8; 12];
        for (pos, val) in copy_section.iter_mut().zip(section) {
            *pos = *val;
        }
        Pattern {
            gush,
            section: copy_section,
            shape: shape as u8, // truncate ok
        }
    }
}

struct State {
    rocks: u64,
    last_blockade: u64,
}

fn main() -> io::Result<()> {
    let max_rocks: u64 = env::args()
        .nth(1)
        .expect("Missing 1st positional argument: target amount of rocks")
        .parse()
        .expect("Error 1st positional argument: expect u64");
    let mut rocks: u64 = 0;

    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf)?;
    let stream: Vec<Gust> = buf
        .into_iter()
        .filter_map(|ch| match ch {
            b'<' => Some(Gust::Left),
            b'>' => Some(Gust::Right),
            _ => None,
        })
        .collect();

    // Pattern hunter; gush, shape, rocks -> starting position
    let mut pattern_hunter: HashMap<Pattern, State> = HashMap::new();
    let mut disable_pattern = false;

    // Blockades are area impassable for rocks; everything behind the blockade
    // is drained from the chamber; keeping the memory usage low
    let mut last_blockade: u64 = 0;

    // Chamber grows upward (back of the vector) - Add floor!
    let mut chamber: Chamber = vec![TETRIS, 0, 0, 0, 0, 0, 0, 0];
    let mut x = SHAPE_START;
    let mut y = SHAPE_MAX_HEIGHT;
    let mut shape = 0;
    let mut stensil = SHAPES[shape].stensil;
    let mut gust = 0;
    let mut tower_height = 1;

    // Simulate now
    while rocks < max_rocks {
        // try push shape
        let try_x = match stream[gust] {
            Gust::Left => x - usize::from(x > 0),
            Gust::Right => x + usize::from((x + SHAPES[shape].width) < CHAMBER_WIDTH),
        };
        if !colliding(&chamber[y..y + SHAPE_MAX_HEIGHT], &(stensil << try_x)) {
            x = try_x
        }
        // try lower shape
        let try_y = y - 1;
        if !colliding(&chamber[try_y..try_y + SHAPE_MAX_HEIGHT], &(stensil << x)) {
            y = try_y;
        } else {
            // Shape gets stuck, land rock, a set new rock
            stensil <<= x;
            chamber[y + 3] |= (stensil >> 24) as u8;
            chamber[y + 2] |= (stensil >> 16) as u8;
            chamber[y + 1] |= (stensil >> 8) as u8;
            chamber[y] |= stensil as u8;
            rocks += 1;
            shape = (shape + 1) % SHAPES.len();
            x = SHAPE_START;
            stensil = SHAPES[shape].stensil;
            // Resize chamber if needed
            tower_height = max(y + SHAPES[shape].height, tower_height);
            chamber.resize(
                max(
                    chamber.len(),
                    1 + tower_height + SHAPE_GAP + SHAPE_MAX_HEIGHT,
                ),
                0,
            );
            let mut new_blockade: usize = 0;
            if chamber[y] | chamber[y + 1] | chamber[y + 2] | chamber[y + 3] == TETRIS {
                new_blockade = y;
            }
            y = tower_height + SHAPE_GAP;
            // A new blockade is detected, restart buffer from there
            if new_blockade != 0 {
                last_blockade += new_blockade as u64;
                tower_height -= new_blockade;
                y -= new_blockade;
                chamber.drain(..new_blockade);
                if !disable_pattern {
                    let pattern = Pattern::new(gust, &chamber, shape);
                    if let Some(state) = pattern_hunter.get(&pattern) {
                        // Pattern found! Fast-forwarding!
                        let cycles = (max_rocks - rocks) / (rocks - state.rocks);
                        rocks += cycles * (rocks - state.rocks);
                        last_blockade += cycles * (last_blockade - state.last_blockade);
                        disable_pattern = true;
                    } else {
                        pattern_hunter.insert(
                            pattern,
                            State {
                                rocks,
                                last_blockade,
                            },
                        );
                    }
                }
            }
        }
        gust = (gust + 1) % stream.len();
    }

    //print_chamber(&chamber);
    println!(
        "Tower height is {}",
        last_blockade + ((tower_height - 1) as u64)
    );
    Ok(())
}
