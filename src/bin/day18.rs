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

use std::collections::VecDeque;

use nalgebra::{point, vector, Point3, Vector3};

fn main() {
    let drops = std::io::stdin()
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let mut iter = l.split(',').map(|n| n.parse::<isize>().unwrap());
            let x = iter.next().unwrap();
            let y = iter.next().unwrap();
            let z = iter.next().unwrap();
            Point3::new(x, y, z)
        })
        .collect::<Vec<_>>();

    let mut grid = Grid::new();
    for drop in drops.iter() {
        grid.set(drop);
    }

    let mut surface_area = 0;
    for drop in drops.iter() {
        for offset in ADJACENT_OFFSETS {
            if !grid.get(&(drop + offset)) {
                surface_area += 1;
            }
        }
    }

    println!("{surface_area}");

    let mut enqueued = Grid::new();
    let mut visited = Grid::new();
    let mut cube_queue = VecDeque::<Point3<isize>>::new();

    // Enqueue all the outer surface cubes.
    for fixed_coord in 0..2 {
        for extreme in [0, MAX_DIM as isize - 2] {
            for u in 0..MAX_DIM as isize {
                for v in 0..MAX_DIM as isize {
                    let mut coords = [extreme, u, v];
                    coords.rotate_left(fixed_coord);
                    let p = Point3::from_slice(&coords);
                    enqueued.set(&p);
                    cube_queue.push_back(p);
                }
            }
        }
    }

    let mut outer_surface_area = 0;
    while let Some(cur) = cube_queue.pop_front() {
        if visited.get(&cur) {
            continue;
        }
        visited.set(&cur);

        for offset in ADJACENT_OFFSETS {
            let adj = cur + offset;
            if !Grid::in_range(&adj) {
                continue;
            }
            if grid.get(&adj) {
                outer_surface_area += 1;
                continue;
            }
            if enqueued.get(&adj) {
                continue;
            }
            enqueued.set(&adj);
            cube_queue.push_back(adj);
        }
    }

    println!("{outer_surface_area}");
}

struct Grid {
    grid: [[[bool; MAX_DIM]; MAX_DIM]; MAX_DIM],
}

impl Grid {
    fn new() -> Self {
        Self {
            grid: [[[false; MAX_DIM]; MAX_DIM]; MAX_DIM],
        }
    }

    fn in_range(p: &Point3<isize>) -> bool {
        for component in p.iter() {
            if *component < 0 || *component >= MAX_DIM as isize {
                return false;
            }
        }
        true
    }

    fn set(&mut self, p: &Point3<isize>) {
        assert!(Self::in_range(p));
        let p: Point3<usize> = p.map(|c| c.try_into().unwrap());
        self.grid[p.x][p.y][p.z] = true;
    }

    fn get(&self, p: &Point3<isize>) -> bool {
        if !Self::in_range(p) {
            return false;
        }
        let p: Point3<usize> = p.map(|c| c.try_into().unwrap());
        self.grid[p.x][p.y][p.z]
    }
}

const MAX_DIM: usize = 32;

const ADJACENT_OFFSETS: [Vector3<isize>; 6] = [
    vector![-1, 0, 0],
    vector![1, 0, 0],
    vector![0, -1, 0],
    vector![0, 1, 0],
    vector![0, 0, -1],
    vector![0, 0, 1],
];
