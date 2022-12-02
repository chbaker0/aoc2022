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
    let rounds: Vec<(char, char)> = std::io::stdin()
        .lines()
        .map(|mut l| {
            let mut iter = l.as_mut().unwrap().split(' ');
            let fst = iter.next().unwrap().chars().next().unwrap();
            let snd = iter.next().unwrap().chars().next().unwrap();
            (fst, snd)
        })
        .collect();

    println!("{}", part1(&rounds));
    println!("{}", part2(&rounds));
}

fn part1(input: &[(char, char)]) -> u32 {
    let map_play = |c| char::from_u32(c as u32 - 'X' as u32 + 'A' as u32).unwrap();
    input
        .iter()
        .map(|(pred, play)| score(to_code(*pred), to_code(map_play(*play))))
        .sum()
}

fn part2(input: &[(char, char)]) -> u32 {
    input
        .iter()
        .map(|(pred, strat)| score(to_code(*pred), do_strat(to_code(*pred), *strat)))
        .sum()
}

fn score(opponent: u32, me: u32) -> u32 {
    me + 1 + ((me + 4 - opponent) % 3) * 3
}

fn to_code(play: char) -> u32 {
    assert!(play == 'A' || play == 'B' || play == 'C');
    play as u32 - 'A' as u32
}

fn do_strat(pred: u32, strat: char) -> u32 {
    assert!(strat == 'X' || strat == 'Y' || strat == 'Z');
    let strat = strat as u32 - 'X' as u32;
    (pred + strat + 2) % 3
}
