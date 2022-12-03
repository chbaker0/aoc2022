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

use itertools::Itertools;

fn main() {
    let sacks: Vec<(HashSet<u32>, HashSet<u32>)> = std::io::stdin()
        .lines()
        .map(|l| {
            let l: Vec<u32> = l
                .unwrap()
                .chars()
                .map(|c| {
                    let c = c as u32;
                    if c >= 'a' as u32 && c <= 'z' as u32 {
                        c - 'a' as u32 + 1
                    } else if c >= 'A' as u32 && c <= 'Z' as u32 {
                        c - 'A' as u32 + 27
                    } else {
                        unreachable!();
                    }
                })
                .collect();

            assert!(l.len() % 2 == 0);
            (
                l[0..l.len() / 2].iter().copied().collect(),
                l[l.len() / 2..].iter().copied().collect(),
            )
        })
        .collect();

    println!("{}", part1(&sacks));
    println!("{}", part2(&sacks));
}

fn part1(input: &[(HashSet<u32>, HashSet<u32>)]) -> u32 {
    input
        .iter()
        .map(|(left, right)| left.intersection(right).next().unwrap())
        .sum()
}

fn part2(input: &[(HashSet<u32>, HashSet<u32>)]) -> u32 {
    input
        .iter()
        .map(|(left, right)| {
            let mut b = left.clone();
            b.extend(right);
            b
        })
        .tuple_windows()
        .step_by(3)
        .map(|(b1, b2, b3)| {
            for typ in b1.iter() {
                if b2.contains(typ) && b3.contains(typ) {
                    return *typ;
                }
            }
            0
        })
        .sum()
}
