use std::{fmt::Display, io};

#[derive(Debug)]
struct Link {
    index: usize,
    left: usize,
    right: usize,
}

struct Decypher {
    cypher: Vec<i64>,
    links: Vec<Link>,
    zero_index: usize,
}

impl Decypher {
    fn from_slice(cypher: &[i64]) -> Decypher {
        let mut zero_index = 0;
        let links: Vec<Link> = cypher
            .iter()
            .enumerate()
            .map(|(i, v)| {
                if *v == 0 {
                    zero_index = i;
                }
                Link {
                    index: i,
                    right: if i == cypher.len() - 1 { 0 } else { i + 1 },
                    left: if i == 0 { cypher.len() - 1 } else { i - 1 },
                }
            })
            .collect();
        Decypher {
            cypher: cypher.to_vec(),
            links,
            zero_index,
        }
    }

    fn encrypt(&mut self, key: i64) {
        self.cypher.iter_mut().for_each(|x| *x *= key);
    }

    fn mix(&mut self) {
        for (index, value) in self.cypher.iter().enumerate() {
            // locate destination first, shorten traversal if possible
            let mut offset = if *value >= 0 {
                *value % ((self.cypher.len() - 1) as i64)
            } else {
                *value % -((self.cypher.len() - 1) as i64)
            };
            let mut dest = if offset > 0 {
                self.links[index].right
            } else {
                index
            };
            while offset != 0 {
                if offset > 0 {
                    dest = self.links[dest].right;
                    offset -= 1;
                } else {
                    dest = self.links[dest].left;
                    offset += 1;
                }
            }
            if index == dest || self.links[index].right == dest {
                // Ok, swapping in place
                continue;
            }

            // Some useful debugging for now:
            //
            // println!("Before move {}: {:}", value, self);

            // Insert the node `y` in between `n` and `m`
            //
            //   x      y        z     <  n      m
            //   ---+   +----+   +---  >  ---+   +--
            //     a|-->|   c|-->|     <    e|-->|
            //      |<--|b   |<--|d    >     |<--|f
            //   ---+   +----+   +---  <  ---+   +--
            //
            let x = self.links[index].left;
            let z = self.links[index].right;
            let n = self.links[dest].left;

            let a = self.links[x].right;
            let b = self.links[index].left;
            let c = self.links[index].right;
            let d = self.links[z].left;
            let e = self.links[n].right;
            let f = self.links[dest].left;

            // Some useful debugging for now:
            //
            // println!("{} {} {} {} {} {}", a, b, c, d, e, f);
            assert!(a == d);

            self.links[x].right = c;
            self.links[index].left = f;
            self.links[index].right = e;
            self.links[z].left = b;
            self.links[n].right = a;
            self.links[dest].left = d;

            // Some useful debugging for now:
            //
            // let a = self.links[x].right;
            // let b = self.links[index].left;
            // let c = self.links[index].right;
            // let d = self.links[z].left;
            // let e = self.links[n].right;
            // let f = self.links[dest].left;
            // println!("{} {} {} {} {} {}", a, b, c, d, e, f);

            // Some useful debugging for now:
            //
            // println!("After move {}: {:}", value, self);
        }
    }

    fn mix_with_cycles(&mut self, cycles: usize) {
        for _ in 0..cycles {
            self.mix();
        }
    }

    fn extract_secrets(&self) -> (i64, i64, i64) {
        let mut cursor: usize = self.zero_index;
        let numbers_offset = 1000 % self.cypher.len();

        for _ in 0..numbers_offset {
            cursor = self.links[cursor].right;
        }
        let a = self.cypher[self.links[cursor].index];

        for _ in 0..numbers_offset {
            cursor = self.links[cursor].right;
        }
        let b = self.cypher[self.links[cursor].index];
        for _ in 0..numbers_offset {
            cursor = self.links[cursor].right;
        }
        let c = self.cypher[self.links[cursor].index];

        (a, b, c)
    }
}

impl Display for Decypher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cursor = self.zero_index;
        let mut join = String::new();
        for i in 0..self.cypher.len() {
            let tok = self.links[cursor].index.to_string();
            join.reserve(tok.len() + 1);
            join.push_str(&tok);
            if i < self.cypher.len() - 1 {
                join.push_str(", ");
            }
            cursor = self.links[cursor].right;
        }
        write!(f, "[{}]", join)
    }
}

fn main() {
    let cypher: Vec<i64> = io::stdin()
        .lines()
        .map(|f| f.unwrap().trim().parse::<i64>().unwrap())
        .collect();

    // Part 1.
    //
    //
    let mut decypher = Decypher::from_slice(&cypher);
    decypher.mix();
    let (a, b, c) = decypher.extract_secrets();
    println!("Part1: {} + {} + {} = {}", a, b, c, a + b + c);

    // Part 2.
    //
    //
    let mut decypher = Decypher::from_slice(&cypher);
    decypher.encrypt(811589153);
    decypher.mix_with_cycles(10);
    let (a, b, c) = decypher.extract_secrets();
    println!("Part2: {} + {} + {} = {}", a, b, c, a + b + c);
}
