use std::{
    collections::{HashMap, VecDeque},
    io::{self, Read},
};

fn find_marker(buf: &str, size: usize) -> usize {
    let mut four = VecDeque::<char>::new();
    let mut uniq: HashMap<char, usize> = HashMap::new();

    for (pos, chr) in buf.chars().enumerate() {
        // I could have use bytes (u8) indexing instead of chars, which are
        // UTF-8. This was more of an experiment. The upshot is I need an
        // additional queue to store `char`, given than accessing a string
        // location is O(n).
        four.push_back(chr);
        match uniq.get_mut(&chr) {
            Some(count) => {
                *count += 1;
            }
            None => {
                uniq.insert(chr, 1);
            }
        }
        if four.len() == size {
            if uniq.len() == size {
                return pos;
            }
            let bye = four.pop_front().unwrap();
            match uniq[&bye] {
                1 => {
                    uniq.remove(&bye);
                }
                _ => {
                    *uniq.get_mut(&bye).unwrap() -= 1;
                }
            }
        }
    }
    // println!("{:?}", four);
    usize::MAX
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    println!("Part 1: position {}", find_marker(&buf, 4) + 1);
    println!("Part 2: position {}", find_marker(&buf, 14) + 1);
}
