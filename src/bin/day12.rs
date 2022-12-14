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

use std::collections::{HashSet, VecDeque};

fn main() {
    let lines: Vec<String> = std::io::stdin().lines().map(Result::unwrap).collect();
    for l in &lines[1..] {
        assert_eq!(l.len(), lines[0].len());
    }

    let extent = Pos {
        x: lines[0].len(),
        y: lines.len(),
    };
    let mut start = None;
    let mut end = None;
    let mut heights = vec![0; extent.x * extent.y];
    for (i, h) in lines.iter().map(|s| s.chars()).flatten().enumerate() {
        let pos = Pos {
            x: i % extent.x,
            y: i / extent.x,
        };

        heights[i] = match h {
            'S' => {
                start = Some(pos);
                0
            }
            'E' => {
                end = Some(pos);
                25
            }
            'a'..='z' => h as u32 - 'a' as u32,
            _ => unreachable!(),
        };
    }

    let map = Map {
        heights: Arr {
            vals: heights,
            extent,
        },
        start: start.unwrap(),
        end: end.unwrap(),
    };

    let costs = sssp(&map);
    println!("{}", costs.at(map.start).fin().unwrap());
    println!(
        "{}",
        map.heights
            .vals
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, h)| *h == 0)
            .map(|(i, _)| costs.vals[i])
            .min()
            .unwrap()
            .fin()
            .unwrap()
    );
}

fn sssp(map: &Map) -> Arr<Dist> {
    let mut costs = Arr {
        vals: vec![Dist::Inf; map.heights.vals.len()],
        extent: map.heights.extent,
    };

    // Breadth-first search since every edge has cost 1.
    let mut visited = HashSet::<Pos>::new();
    let mut queue = VecDeque::<(Pos, u32)>::new();
    queue.push_back((map.end, 0));
    while let Some((p, d)) = queue.pop_front() {
        let h = *map.heights.at(p);
        *costs.at_mut(p) = Dist::Fin(d);
        for (off_x, off_y) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let x = p.x as isize + off_x;
            let y = p.y as isize + off_y;
            if x < 0 || y < 0 {
                continue;
            }

            let x = x as usize;
            let y = y as usize;

            if x >= map.heights.extent.x || y >= map.heights.extent.y {
                continue;
            }

            let next = Pos { x, y };
            let h_next = *map.heights.at(next);
            if h_next + 1 < h {
                continue;
            }

            if *costs.at(next) < Dist::Inf {
                continue;
            }

            if visited.contains(&next) {
                continue;
            }

            visited.insert(next);
            queue.push_back((next, d + 1));
        }
    }

    costs
}

#[derive(Clone, Debug)]
struct Map {
    heights: Arr<u32>,
    start: Pos,
    end: Pos,
}

#[derive(Clone, Debug)]
struct Arr<T> {
    vals: Vec<T>,
    extent: Pos,
}

impl<T> Arr<T> {
    fn at(&self, pos: Pos) -> &T {
        &self.vals[pos.x + pos.y * self.extent.x]
    }

    fn at_mut(&mut self, pos: Pos) -> &mut T {
        &mut self.vals[pos.x + pos.y * self.extent.x]
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Dist {
    Fin(u32),
    Inf,
}

impl Dist {
    fn fin(self) -> Option<u32> {
        match self {
            Self::Fin(x) => Some(x),
            _ => None,
        }
    }
}
