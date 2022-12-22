use geo::coord;
use geo::geometry::{Coord, Rect};
#[allow(unused_imports)]
use std::cmp::{max, min};
use std::env;
use std::io::{self, Write};

struct Sensor {
    location: Coord<i32>,
    closest_beacon: Coord<i32>,
}

impl Sensor {
    fn coverage_radius(&self) -> i32 {
        let (a, b) = (self.location - self.closest_beacon).x_y();
        a.abs() + b.abs()
    }

    fn bounding_box(&self) -> Rect<i32> {
        Rect::new(self.location, self.location + coord! { x: 1, y: 1})
    }

    fn coverage_bounding_box(&self) -> Rect<i32> {
        let bounds = self.bounding_box();
        let coverage_dist = self.coverage_radius();
        let delta = coord! { x: coverage_dist, y: coverage_dist };
        Rect::new(bounds.min() - delta, bounds.max() + delta)
    }
}

fn main() {
    let upper_bound = 1 + env::args()
        .nth(1)
        .expect("Missing integer argument: y coordinate")
        .parse::<i32>()
        .expect("Malformed integer argument: y coordinate");

    let sensors: Vec<Sensor> = io::stdin()
        .lines()
        .map(|str_sensor| {
            str_sensor
                .unwrap()
                .trim()
                .split_once(':')
                .map(|(location, beacon)| {
                    let (location_x, location_y) = location.split_once(',').unwrap();
                    let (beacon_x, beacon_y) = beacon.split_once(',').unwrap();
                    Sensor {
                        location: coord! {
                                x: location_x.parse::<i32>().unwrap(),
                                y: location_y.parse::<i32>().unwrap(),

                        },
                        closest_beacon: coord! {
                                x: beacon_x.parse::<i32>().unwrap(),
                                y: beacon_y.parse::<i32>().unwrap(),

                        },
                    }
                })
                .unwrap()
        })
        .collect();

    // Brute-force: scan the entire range line-by-line to find a coverage that ends up with 2
    // intervals disjoint by 1 unit.
    let mut distress_beacon: Coord<i32> = coord! {x: 0, y: 0};
    for y_scan in 0..upper_bound {
        if y_scan % 10000 == 0 {
            print!(".");
            io::stdout().flush().unwrap();
        }

        // Compute all coverage intervals on row = y_scan for all sensors.
        // Intervals are [x, y), where all v: x <= v < y belong to the interval
        let mut intervals: Vec<(i32, i32)> = sensors
            .iter()
            .filter(|sensor| {
                let coverage_bounds = sensor.coverage_bounding_box();
                coverage_bounds.min().y <= y_scan && y_scan < coverage_bounds.max().y
            })
            .map(|sensor| {
                let bounds = sensor.bounding_box();
                let radius = sensor.coverage_radius();
                (
                    max(
                        0,
                        bounds.min().x - (radius - (y_scan - bounds.min().y).abs()),
                    ),
                    min(
                        upper_bound,
                        bounds.max().x + (radius - (y_scan - bounds.min().y).abs()),
                    ),
                )
            })
            .collect();

        intervals.sort_unstable_by(|&a, &b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        let mut merged_intervals = Vec::<(i32, i32)>::new();
        intervals.iter().fold(
            &mut merged_intervals,
            |merged, &elem| -> &mut Vec<(i32, i32)> {
                if elem.0 < elem.1 {
                    // discard closed intervals
                    if !merged.is_empty() && elem.0 < merged.last().unwrap().1 {
                        merged.last_mut().unwrap().1 = max(elem.1, merged.last().unwrap().1);
                    } else {
                        merged.push(elem);
                    }
                }
                merged
            },
        );

        // If we have obtained exactly 2 intervals; they should have a
        // difference of 1 (the distress beacon) and we should be done!
        match merged_intervals.len() {
            0 | 1 => (), // not interested
            2 => {
                assert!(merged_intervals[0].1 + 1 == merged_intervals[1].0);
                distress_beacon = coord! { x: merged_intervals[0].1, y: y_scan };
                break;
            }
            _ => panic!("More than 2 intervals?!"),
        }
    }
    println!();

    println!(
        "Distress beacon: {},{} signal: {}",
        distress_beacon.x,
        distress_beacon.y,
        (distress_beacon.x as i64) * 4000000 + (distress_beacon.y as i64),
    );
}
