use geo::coord;
use geo::geometry::{Coord, Rect};
#[allow(unused_imports)]
use std::cmp::{max, min};
use std::env;
use std::io;

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
    let y_coverage = env::args()
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

    let mut beacons = sensors
        .iter()
        .map(|x| x.closest_beacon)
        .collect::<Vec<Coord<i32>>>();
    beacons.sort_unstable_by(|&a, &b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));
    beacons.dedup();

    // Compute all coverage intervals on row = y_coverage for all sensors
    // Intervals are [x, y), where all v: x <= v < y belong to the interval
    let mut intervals: Vec<(i32, i32)> = sensors
        .iter()
        .filter(|sensor| {
            let coverage_bounds = sensor.coverage_bounding_box();
            coverage_bounds.min().y <= y_coverage && y_coverage < coverage_bounds.max().y
        })
        .map(|sensor| {
            // In manathan distance intersection is simplified, when we are in bounding box:
            // ------#-------
            // =====###======
            // ----##O##-----
            // -----###------
            // ------#-------
            // min = sensor_min_bound.x - (coverage_radius - abs(y_coverage - sensor_min_bound.y))
            // max = sensor_max_bound.x + (coverage_radius - abs(y_coverage - sensor_min_bound.y))
            let bounds = sensor.bounding_box();
            let radius = sensor.coverage_radius();
            (
                bounds.min().x - (radius - (y_coverage - bounds.min().y).abs()),
                bounds.max().x + (radius - (y_coverage - bounds.min().y).abs()),
            )
        })
        .collect();

    // Sort all intervals, then reduce overlapping intervals, then sum.
    intervals.sort_unstable_by(|&a, &b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    let mut sum = intervals[1..]
        .iter()
        .fold(&mut vec![intervals[0]], |merged, &elem| {
            if elem.0 < merged.last().unwrap().1 {
                merged.last_mut().unwrap().1 = max(elem.1, merged.last().unwrap().1);
            } else {
                merged.push(elem);
            }
            merged
        })
        .iter()
        .fold(0, |sum, &elem| sum + (elem.1 - elem.0));

    // Finally, remove all the beacons already at that coverage!
    sum -= beacons
        .iter()
        .filter(|&beacon| beacon.y == y_coverage)
        .count() as i32;

    println!("Positions that cannot contain beacon: {sum}")
}
