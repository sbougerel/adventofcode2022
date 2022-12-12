use std::{io, io::prelude::*};
extern crate ndarray;
use ndarray::{arr1};


fn main() {
    let rope_len = std::env::args().nth(1).unwrap().parse().unwrap();
    let mut rope = Vec::new();
    for _ in 0..rope_len {
        rope.push(arr1(&[0, 0]));
    }
    let mut visited = std::collections::HashSet::new();
    visited.insert(rope[rope.len() - 1].clone());

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut iter = line.split(" ");
        let orient = iter.next().unwrap();
        let steps: i32 = iter.next().unwrap().parse().unwrap();
        for _ in 0..steps {
            match orient {
                "D" => rope[0][1] -= 1,
                "U" => rope[0][1] += 1,
                "L" => rope[0][0] -= 1,
                "R" => rope[0][0] += 1,
                &_ => todo!(),
            }
            for knot in 1..rope.len() {
                let mut dt = &rope[knot - 1] - &rope[knot];
                if 2 < (&dt * &dt).sum() {
                    dt.mapv_inplace(|e: i32| if e.abs() > 1 {e / 2} else {e});
                    rope[knot] = &rope[knot] + &dt;
                } else {
                    break;
                }
            }
            visited.insert(rope[rope.len() - 1].clone());
        }
    }
    println!("{}", visited.len());
}
