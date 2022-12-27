use core::ops::{Add, Sub};
use std::cmp::{max, min};
use std::convert::From;
use std::{
    collections::{HashMap, HashSet},
    io,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coord(i32, i32, i32);

impl Coord {
    fn fill(val: i32) -> Coord {
        Coord(val, val, val)
    }
}

impl Add for Coord {
    type Output = Coord;
    fn add(self, other: Coord) -> Coord {
        Coord(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Coord {
    type Output = Coord;
    fn sub(self, other: Coord) -> Coord {
        Coord(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Voxel(Coord);

impl From<Coord> for Voxel {
    fn from(coord: Coord) -> Voxel {
        Voxel(coord)
    }
}

impl From<Voxel> for Coord {
    fn from(voxel: Voxel) -> Coord {
        voxel.0
    }
}

struct Neighbors {
    index: usize,
    voxel: Voxel,
}

impl Iterator for Neighbors {
    type Item = Voxel;
    fn next(&mut self) -> Option<Voxel> {
        let last = self.index;
        self.index += 1;
        match last {
            0 => Some(Voxel(Coord(
                self.voxel.0 .0 + 1,
                self.voxel.0 .1,
                self.voxel.0 .2,
            ))),
            1 => Some(Voxel(Coord(
                self.voxel.0 .0 - 1,
                self.voxel.0 .1,
                self.voxel.0 .2,
            ))),
            2 => Some(Voxel(Coord(
                self.voxel.0 .0,
                self.voxel.0 .1 + 1,
                self.voxel.0 .2,
            ))),
            3 => Some(Voxel(Coord(
                self.voxel.0 .0,
                self.voxel.0 .1 - 1,
                self.voxel.0 .2,
            ))),
            4 => Some(Voxel(Coord(
                self.voxel.0 .0,
                self.voxel.0 .1,
                self.voxel.0 .2 + 1,
            ))),
            5 => Some(Voxel(Coord(
                self.voxel.0 .0,
                self.voxel.0 .1,
                self.voxel.0 .2 - 1,
            ))),
            _ => None,
        }
    }
}

impl Voxel {
    fn neighbors(&self) -> Neighbors {
        Neighbors {
            index: 0,
            voxel: *self,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Face(Coord, Coord);

const FACES: [(Coord, Coord); 6] = [
    (Coord(0, 0, 0), Coord(1, 1, 0)),
    (Coord(0, 0, 0), Coord(1, 0, 1)),
    (Coord(0, 0, 0), Coord(0, 1, 1)),
    (Coord(1, 0, 0), Coord(1, 1, 1)),
    (Coord(0, 1, 0), Coord(1, 1, 1)),
    (Coord(0, 0, 1), Coord(1, 1, 1)),
];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct BoundingBox(Coord, Coord);

impl From<Voxel> for BoundingBox {
    fn from(voxel: Voxel) -> BoundingBox {
        BoundingBox(voxel.0, voxel.0 + Coord::fill(1))
    }
}

impl BoundingBox {
    fn extend(&mut self, bb: &BoundingBox) {
        self.0 = Coord(
            min(self.0 .0, bb.0 .0),
            min(self.0 .1, bb.0 .1),
            min(self.0 .2, bb.0 .2),
        );
        self.1 = Coord(
            max(self.1 .0, bb.1 .0),
            max(self.1 .1, bb.1 .1),
            max(self.1 .2, bb.1 .2),
        );
    }

    fn intersect(&self, voxel: &Voxel) -> bool {
        self.0 .0 <= voxel.0 .0
            && voxel.0 .0 < self.1 .0
            && self.0 .1 <= voxel.0 .1
            && voxel.0 .1 < self.1 .1
            && self.0 .2 <= voxel.0 .2
            && voxel.0 .2 < self.1 .2
    }

    fn dim(&self) -> Coord {
        Coord(
            self.1 .0 - self.0 .0,
            self.1 .1 - self.0 .1,
            self.1 .2 - self.0 .2,
        )
    }
}

struct Faces<'a> {
    coord: &'a Coord,
    index: usize,
}

impl Iterator for Faces<'_> {
    type Item = Face;
    fn next(&mut self) -> Option<Face> {
        let last = self.index;
        self.index += 1;
        if last < 6 {
            Some(Face(
                *self.coord + FACES[last].0,
                *self.coord + FACES[last].1,
            ))
        } else {
            None
        }
    }
}

impl Voxel {
    fn faces(&self) -> Faces {
        Faces {
            coord: &self.0,
            index: 0,
        }
    }
}

fn total_surface_area(cubes: &[Voxel]) -> usize {
    let mut face_set: HashSet<Face> = HashSet::new();
    for voxel in cubes {
        for face in voxel.faces() {
            if face_set.contains(&face) {
                face_set.remove(&face);
            } else {
                face_set.insert(face);
            }
        }
    }
    face_set.len()
}

struct Shells<'a> {
    index: usize,
    bb: &'a BoundingBox,
}

impl Iterator for Shells<'_> {
    type Item = BoundingBox;
    fn next(&mut self) -> Option<BoundingBox> {
        let last = self.index;
        self.index += 1;
        // The iterator walk through the 6 (max) bounding boxes forming the
        // shell around a bounding box. Given a bounding box (a, b, c), (A, B,
        // C), that's:
        //
        // - bounding box (a, b, c - 1), (A, B, c)
        // - bounding box (a, b - 1, c), (A, b, C)
        // - bounding box (a - 1, b, c), (a, B, C)
        // - bounding box (a, b, C), (A, B, C + 1)
        // - bounding box (a, B, c), (A, B + 1, C)
        // - bounding box (A, b, c), (A + 1, B, C)
        match last {
            0 => Some(BoundingBox(
                Coord(self.bb.0 .0, self.bb.0 .1, self.bb.0 .2 - 1),
                Coord(self.bb.1 .0, self.bb.1 .1, self.bb.0 .2),
            )),
            1 => Some(BoundingBox(
                Coord(self.bb.0 .0, self.bb.0 .1 - 1, self.bb.0 .2),
                Coord(self.bb.1 .0, self.bb.0 .1, self.bb.1 .2),
            )),
            2 => Some(BoundingBox(
                Coord(self.bb.0 .0 - 1, self.bb.0 .1, self.bb.0 .2),
                Coord(self.bb.0 .0, self.bb.1 .1, self.bb.1 .2),
            )),
            3 => Some(BoundingBox(
                Coord(self.bb.0 .0, self.bb.0 .1, self.bb.1 .2),
                Coord(self.bb.1 .0, self.bb.1 .1, self.bb.1 .2 + 1),
            )),
            4 => Some(BoundingBox(
                Coord(self.bb.0 .0, self.bb.1 .1, self.bb.0 .2),
                Coord(self.bb.1 .0, self.bb.1 .1 + 1, self.bb.1 .2),
            )),
            5 => Some(BoundingBox(
                Coord(self.bb.1 .0, self.bb.0 .1, self.bb.0 .2),
                Coord(self.bb.1 .0 + 1, self.bb.1 .1, self.bb.1 .2),
            )),
            _ => None,
        }
    }
}

struct Voxels {
    index: usize,
    max: usize,
    origin: Coord,
    dim: Coord,
}

impl Iterator for Voxels {
    type Item = Voxel;
    fn next(&mut self) -> Option<Voxel> {
        if self.index >= self.max {
            None
        } else {
            let dt: i32 = self.index as i32;
            self.index += 1;
            Some(Voxel(
                self.origin
                    + Coord(
                        dt % self.dim.0,
                        (dt / self.dim.0) % self.dim.1,
                        (dt / self.dim.0 / self.dim.1) % self.dim.2,
                    ),
            ))
        }
    }
}

impl BoundingBox {
    fn voxels(&self) -> Voxels {
        let dim = self.dim();
        Voxels {
            index: 0,
            max: (dim.0 * dim.1 * dim.2) as usize,
            origin: self.0,
            dim,
        }
    }

    fn shells(&self) -> Shells {
        Shells { index: 0, bb: self }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum VoxelType {
    Lava,
    Air,
}

fn external_surface_area(bb: &BoundingBox, lava: &[Voxel]) -> usize {
    let mut open: HashSet<Voxel> = bb.shells().flat_map(|bb| bb.voxels()).collect();
    let mut visited: HashMap<Voxel, VoxelType> =
        lava.iter().map(|v| (*v, VoxelType::Lava)).collect();
    let mut count_faces = 0;
    while !open.is_empty() {
        let voxel = *open.iter().next().unwrap();
        open.remove(&voxel);
        visited.insert(voxel, VoxelType::Air);
        for neighbor in voxel.neighbors() {
            if !bb.intersect(&neighbor) {
                continue;
            }
            if let Some(voxel_type) = visited.get(&neighbor) {
                if *voxel_type == VoxelType::Lava {
                    count_faces += 1;
                }
                continue;
            }
            if !open.contains(&neighbor) {
                open.insert(neighbor);
            }
        }
    }
    count_faces
}

fn main() {
    let lava: Vec<Voxel> = io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut iter = line.trim().split(',').map(|x| x.parse::<i32>().unwrap());
            Coord(
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            )
            .into()
        })
        .collect();

    println!("Total surface area {}", total_surface_area(&lava));

    let bb = lava[1..]
        .iter()
        .fold(BoundingBox::from(lava[0]), |mut bb, &cube| {
            bb.extend(&cube.into());
            bb
        });

    println!("bounds {:?}", bb);
    println!(
        "External surface area {}",
        external_surface_area(&bb, &lava)
    );
}
