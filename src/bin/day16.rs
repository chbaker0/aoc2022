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

use core::num;
use std::collections::{HashMap, VecDeque};

use bitvec::{bitarr, BitArr};
use nalgebra::distance;

fn main() {
    use regex::Regex;
    let line_re =
        Regex::new(r#"Valve ([a-zA-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.*)"#)
            .unwrap();
    let list_re = Regex::new(r#"([a-zA-Z]{2})(?:, |$)"#).unwrap();
    let valves: Vec<ValveDesc> = std::io::stdin()
        .lines()
        .map(|l| {
            let caps = line_re.captures(l.as_ref().unwrap()).unwrap();
            let mut item = caps.iter().skip(1).flat_map(|m| Some(m?.as_str()));
            let name = item.next().unwrap().to_string();
            let flow = item.next().unwrap().parse().unwrap();
            let list_str = item.next().unwrap();
            let tunnels: Vec<String> = list_re
                .captures_iter(list_str)
                .map(|c| c.get(1).unwrap().as_str().to_string())
                .collect();
            ValveDesc {
                name,
                flow,
                tunnels,
            }
        })
        .collect();

    let num_valves = valves.len();
    let name_to_id: HashMap<String, usize> = valves
        .iter()
        .map(|v| v.name.clone())
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect();
    println!("{name_to_id:?}");
    let flows: Vec<u32> = valves.iter().map(|v| v.flow).collect();
    let connections: Vec<BitArr!(for MAX_VALVES)> = valves
        .iter()
        .map(|v| {
            let mut bits = bitarr![0; MAX_VALVES];
            println!("{}", v.name);
            for conn_name in v.tunnels.iter() {
                let conn_id = name_to_id.get(conn_name).unwrap();
                println!("{conn_name} {conn_id}");
                bits.set(*conn_id, true);
            }
            for conn in v.tunnels.iter().map(|name| name_to_id.get(name).unwrap()) {
                bits.set(*conn, true);
            }
            bits
        })
        .collect();

    for conn in connections.iter() {
        println!("{:016b}", conn.as_raw_slice()[0]);
    }

    let mut distances: [[Option<u32>; MAX_VALVES]; MAX_VALVES] = [[None; MAX_VALVES]; MAX_VALVES];
    for src in 0..num_valves {
        let mut queue = VecDeque::new();
        queue.push_back((0, src));
        while let Some((dist, cur)) = queue.pop_front() {
            if distances[src][cur] != None {
                continue;
            }
            distances[src][cur] = Some(dist);
            for next in connections[cur].iter_ones() {
                queue.push_back((dist + 1, next));
            }
        }
    }

    for src in 0..num_valves {
        for dst in 0..num_valves {
            print!("{} ", distances[src][dst].map(|x| x as i32).unwrap_or(-1));
        }
        println!();
    }

    // DP solution working backwards from end state (end of 30 minutes). Each
    // step back we go, compute the max released pressure from now onward and
    // account for which valves are opened now or later.
    let mut next_states: [State; MAX_VALVES] = [State {
        released: 0,
        opened: bitarr![0; MAX_VALVES],
    }; MAX_VALVES];

    for minute in (1..=30).rev() {
        let mut prev_states: [State; MAX_VALVES] = [State {
            released: 0,
            opened: bitarr![0; MAX_VALVES],
        }; MAX_VALVES];

        let multiplier = 30 - minute + 1;
        for (at_valve, state) in prev_states.iter_mut().enumerate() {
            // We must decide whether to open this valve, move to another valve
            // (for which we have computed the optimal strategy), or do nothing.
            let (max_move, max_move_state) = connections[at_valve]
                .iter_ones()
                .filter(|id| *id != at_valve)
                .map(|id| (id, &next_states[id]))
                .max_by_key(|(_, next_state)| next_state.released)
                .unwrap();

            // Check if opening this valve is possible and better than moving.
            // If so, do that.
            if !next_states[at_valve].opened[at_valve] {
                let open_value = flows[at_valve] * multiplier + next_states[at_valve].released;
                if open_value > max_move_state.released {
                    state.released += open_value;
                    state.opened.set(at_valve, true);
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct State {
    released: u32,
    opened: BitArr!(for MAX_VALVES),
}

#[derive(Clone, Debug)]
struct ValveDesc {
    name: String,
    flow: u32,
    tunnels: Vec<String>,
}

const MAX_VALVES: usize = 64;
