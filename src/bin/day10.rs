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
    let input: Vec<Option<i64>> = std::io::stdin()
        .lines()
        .map(|l| {
            let l = l.as_ref().unwrap();
            if l == "noop" {
                return None;
            }

            let mut iter = l.split(' ');
            assert_eq!(iter.next(), Some("addx"));
            Some(iter.next().unwrap().parse().unwrap())
        })
        .collect();

    let mut processed: Vec<i64> = vec![1, 1];
    for i in input {
        let prev = *processed.last().unwrap();
        match i {
            None => processed.push(prev),
            Some(n) => {
                processed.push(prev);
                processed.push(prev + n);
            }
        }
    }

    let p1: i64 = processed
        .iter()
        .enumerate()
        .skip(20)
        .step_by(40)
        .map(|(c, n)| c as i64 * n)
        .sum();
    println!("{p1}");

    for pos in 0..40 * 6 {
        let hpos = pos % 40;
        let x = processed[pos as usize + 1];
        if x == hpos || x == hpos + 1 || x + 1 == hpos {
            print!("#")
        } else {
            print!(".");
        }

        if hpos == 39 {
            println!();
        }
    }
}
