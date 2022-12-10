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

fn main() {
    let inputs: Vec<(Assignment, Assignment)> = std::io::stdin()
        .lines()
        .map(|l| {
            let mut iter = l.as_ref().unwrap().split(&['-', ',']);
            let a = iter.next().unwrap().parse().unwrap();
            let b = iter.next().unwrap().parse().unwrap();
            let c = iter.next().unwrap().parse().unwrap();
            let d = iter.next().unwrap().parse().unwrap();
            (
                Assignment { first: a, last: b },
                Assignment { first: c, last: d },
            )
        })
        .collect();

    println!("{}", part1(&inputs));
    println!("{}", part2(&inputs));
}

fn part1(inputs: &[(Assignment, Assignment)]) -> u32 {
    inputs
        .iter()
        .filter(|(a, b)| a.contains(b) || b.contains(a))
        .count() as u32
}

fn part2(inputs: &[(Assignment, Assignment)]) -> u32 {
    inputs.iter().filter(|(a, b)| a.overlaps(b)).count() as u32
}

#[derive(Clone, Copy, Debug)]
struct Assignment {
    first: u32,
    last: u32,
}

impl Assignment {
    fn contains(&self, other: &Self) -> bool {
        self.first >= other.first && self.last <= other.last
    }

    fn overlaps(&self, other: &Self) -> bool {
        (self.first >= other.first && self.first <= other.last)
            || (other.first >= self.first && other.first <= self.last)
    }
}
