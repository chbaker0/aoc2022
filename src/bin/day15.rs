// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashSet;

fn main() {
    use regex::Regex;
    let re = Regex::new(
        r#"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$"#,
    )
    .unwrap();
    let pings: Vec<Ping> = std::io::stdin()
        .lines()
        .map(|l| {
            let caps = re.captures(l.as_ref().unwrap()).unwrap();
            let mut item = caps
                .iter()
                .skip(1)
                .map(|m| m.unwrap().as_str().parse::<i32>().unwrap());
            let s_x = item.next().unwrap();
            let s_y = item.next().unwrap();
            let b_x = item.next().unwrap();
            let b_y = item.next().unwrap();
            Ping {
                sensor: Pos { x: s_x, y: s_y },
                beacon: Pos { x: b_x, y: b_y },
            }
        })
        .collect();

    println!(
        "y=10: {}\ny=2000000: {}",
        part1(&pings, 10),
        part1(&pings, 2000000)
    );

    println!(
        "search space 20: {:?}\nsearch space 4000000: {:?}",
        part2(&pings, 20),
        part2(&pings, 4000000)
    );
}

fn part1(pings: &[Ping], y: i32) -> i32 {
    scan_row(pings, y).0
}

fn part2(pings: &[Ping], max_coord: i32) -> Option<i64> {
    for y in 0..=max_coord {
        if let Some(x) = scan_row(pings, y).1 {
            println!("({x}, {y})");
            return Some(y as i64 + (x as i64 * 4000000));
        }
    }

    None
}

fn scan_row(pings: &[Ping], y: i32) -> (i32, Option<i32>) {
    let beacons: HashSet<i32> = pings
        .iter()
        .filter(|p| p.beacon.y == y)
        .map(|p| p.beacon.x)
        .collect();
    let mut zones: Vec<(i32, i32)> = pings.iter().flat_map(|p| p.exclusion_zone(y)).collect();
    zones.sort_unstable();
    zones.reverse();

    let mut total = 0;
    let mut one_hole = false;
    let mut hole = None;
    let mut cur_zone = match zones.pop() {
        Some(z) => z,
        None => return (0, None),
    };
    while let Some(z) = zones.pop() {
        if cur_zone.1 < z.0 {
            if cur_zone.1 + 2 == z.0
                && cur_zone.1 + 1 <= 4000000
                && !beacons.contains(&(cur_zone.1 + 1))
            {
                if hole == None {
                    one_hole = true;
                    hole = Some(cur_zone.1 + 1);
                } else {
                    one_hole = false;
                }
            }
            total += cur_zone.1 - cur_zone.0 + 1;
            cur_zone = z;
            continue;
        }

        cur_zone.0 = std::cmp::min(cur_zone.0, z.0);
        cur_zone.1 = std::cmp::max(cur_zone.1, z.1);
    }

    total += cur_zone.1 - cur_zone.0 + 1;
    (total, if one_hole { hole } else { None })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Ping {
    sensor: Pos,
    beacon: Pos,
}

impl Ping {
    fn exclusion_zone(&self, y: i32) -> Option<(i32, i32)> {
        let dist = (self.beacon.x - self.sensor.x).abs() + (self.beacon.y - self.sensor.y).abs();
        if y > self.sensor.y + dist || y < self.sensor.y - dist {
            return None;
        }

        let zone_side_width = dist - (self.sensor.y - y).abs();
        assert!(zone_side_width >= 0, "{zone_side_width}");
        let mut zone = (
            self.sensor.x - zone_side_width,
            self.sensor.x + zone_side_width,
        );
        assert!(zone.0 <= zone.1, "{} > {}", zone.0, zone.1);

        if self.beacon.y != y {
            return Some(zone);
        }

        if self.beacon.x == zone.0 {
            zone.0 += 1;
        } else if self.beacon.x == zone.1 {
            zone.1 -= 1;
        }

        if zone.0 <= zone.1 {
            if self.beacon.y == y {
                assert!(self.beacon.x < zone.0 || self.beacon.x > zone.1);
            }
            Some(zone)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_exclusion_zone() {
        let ping = Ping {
            sensor: Pos { x: 8, y: 7 },
            beacon: Pos { x: 2, y: 10 },
        };
        assert_eq!(ping.exclusion_zone(7), Some((-1, 17)));
        assert_eq!(ping.exclusion_zone(-2), Some((8, 8)));
        assert_eq!(ping.exclusion_zone(16), Some((8, 8)));
        assert_eq!(ping.exclusion_zone(-3), None);
        assert_eq!(ping.exclusion_zone(17), None);
        assert_eq!(ping.exclusion_zone(8), Some((0, 16)));
        assert_eq!(ping.exclusion_zone(10), Some((3, 14)));
    }
}
