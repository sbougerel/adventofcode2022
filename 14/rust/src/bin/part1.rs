use geo::geometry::{Coord, Line, LineString, Rect};
use geo::{coord, BoundingRect};
use ndarray::Array2;
use std::cmp::{max, min};
use std::io;

fn to_map(x: i32, y: i32) -> [usize; 2] {
    [usize::try_from(x).unwrap(), usize::try_from(y).unwrap()]
}

fn coord_to_map(coord: Coord<i32>) -> [usize; 2] {
    [
        usize::try_from(coord.x).unwrap(),
        usize::try_from(coord.y).unwrap(),
    ]
}

fn drop_sand(map: &Array2<u8>, mut sand: Coord<i32>) -> Coord<i32> {
    let mut last: Coord<i32> = coord! {x: -1, y: -1};
    // stop when sand is at rest
    while (sand.y + 1) < (map.ncols() as i32) && last != sand {
        last = sand;
        if map[to_map(sand.x, sand.y + 1)] == 0 {
            sand.y += 1;
        } else if map[to_map(sand.x - 1, sand.y + 1)] == 0 {
            // down and to the left first
            sand.y += 1;
            sand.x -= 1;
        } else if map[to_map(sand.x + 1, sand.y + 1)] == 0 {
            // down and to the right next
            sand.y += 1;
            sand.x += 1;
        }
    }
    sand
}

fn main() {
    let lines: Vec<Line<i32>> = io::stdin()
        .lines()
        .map(|str_line| {
            str_line
                .unwrap()
                .trim()
                .split("-")
                .map(|str_point| {
                    let mut iter = str_point.split(",");
                    coord! {
                        x: iter.next().unwrap().parse::<i32>().unwrap(),
                        y: iter.next().unwrap().parse::<i32>().unwrap(),
                    }
                })
                .collect::<LineString<i32>>()
                .lines()
                .collect::<Vec<Line<i32>>>()
        })
        .flatten()
        .collect();

    let sand_start = coord! { x: 500, y: 0 };

    let bounding_rect = lines
        .iter()
        .fold(sand_start.bounding_rect(), |mut rect, line| {
            let line_rect = line.bounding_rect();
            rect = Rect::new(
                coord! {
                    x: min(rect.min().x, line_rect.min().x),
                    y: min(rect.min().y, line_rect.min().y),
                },
                coord! {
                    x: max(rect.max().x, line_rect.max().x),
                    y: max(rect.max().y, line_rect.max().y),
                },
            );
            rect
        });

    // Create a map with slightly larger area to find where the sand escaping
    let mut map = Array2::<u8>::zeros(to_map(
        bounding_rect.width() + 3,
        bounding_rect.height() + 2,
    ));
    let origin = coord! { x: bounding_rect.min().x - 1, y: bounding_rect.min().y };

    // Populate map:
    // 0 - empty space
    // 1 - wall
    // 2 - sand
    for line in lines {
        let rect = line.bounding_rect();
        if rect.height() == 0 {
            for x in rect.min().x..(rect.max().x + 1) {
                map[to_map(x - origin.x, rect.min().y - origin.y)] = 1;
            }
        } else if rect.width() == 0 {
            for y in rect.min().y..(rect.max().y + 1) {
                map[to_map(rect.min().x - origin.x, y - origin.y)] = 1;
            }
        } else {
            panic!("Diagonal line?!")
        }
    }

    // for y in 0..map.ncols() {
    //     for x in 0..map.nrows() {
    //         match map[[x, y]] {
    //             0 => print!("."),
    //             1 => print!("#"),
    //             _ => panic!("Unknown value"),
    //         }
    //     }
    //     println!("")
    // }

    let mut sand_grains = 0;
    loop {
        let sand_end = drop_sand(&map, sand_start - origin);
        if sand_end.y == (map.ncols() - 1) as i32 {
            // touched the edge
            break;
        }
        map[coord_to_map(sand_end)] = 2;
        sand_grains += 1;
    }

    for y in 0..map.ncols() {
        for x in 0..map.nrows() {
            match map[[x, y]] {
                0 => print!("."),
                1 => print!("#"),
                2 => print!("o"),
                _ => panic!("Unknown value"),
            }
        }
        println!("")
    }
    println!("Sand grains dropped: {sand_grains}");
}
