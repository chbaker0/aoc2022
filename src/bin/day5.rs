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
    let lines: Vec<String> = std::io::stdin().lines().map(|l| l.unwrap()).collect();

    let mut first_command = 0;
    let mut num_stacks = 0;
    let mut crates = Vec::<char>::new();
    for (r, l) in lines.iter().enumerate() {
        let mut iter = l.chars().skip(1).step_by(4).peekable();
        if iter.peek().unwrap().is_digit(10) {
            first_command = r + 2;
            break;
        }
        crates.extend(iter);
        if r == 0 {
            num_stacks = crates.len();
        } else {
            assert_eq!(crates.len() % num_stacks, 0);
        }
    }

    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_stacks];
    for s in 0..num_stacks {
        for r in (0..crates.len() / num_stacks).rev() {
            let c = crates[r * num_stacks + s];
            if c == ' ' {
                break;
            }
            stacks[s].push(c);
        }
    }

    let cmds: Vec<Command> = lines
        .iter()
        .skip(first_command)
        .map(|l| {
            let mut iter = l.split(' ');
            assert_eq!(iter.next().unwrap(), "move");
            let cnt = iter.next().unwrap().parse().unwrap();
            assert_eq!(iter.next().unwrap(), "from");
            let src = iter.next().unwrap().parse().unwrap();
            assert_eq!(iter.next().unwrap(), "to");
            let dst = iter.next().unwrap().parse().unwrap();
            Command { cnt, src, dst }
        })
        .collect();

    for r in 0..crates.len() / num_stacks {
        for s in 0..num_stacks {
            print!("{} ", crates[r * num_stacks + s]);
        }
        println!();
    }
    println!();

    for stack in stacks.iter() {
        for c in stack.iter() {
            print!("{c} ");
        }
        println!();
    }
    println!();

    for cmd in cmds.iter() {
        println!("# {} {} -> {}", cmd.cnt, cmd.src, cmd.dst);
    }

    println!();

    // Part 1
    let mut p1_stacks = stacks.clone();
    for cmd in cmds.iter() {
        let Command { cnt, src, dst } = *cmd;
        for _i in 0..cnt {
            let c = p1_stacks[src - 1].pop().unwrap();
            p1_stacks[dst - 1].push(c);
        }
    }

    for s in p1_stacks {
        print!("{}", s.last().unwrap());
    }
    println!();

    // Part 2
    let mut p2_stacks = stacks.clone();
    for cmd in cmds.iter() {
        let Command { cnt, src, dst } = *cmd;
        let split_point = p2_stacks[src - 1].len() - cnt;
        let load = p2_stacks[src - 1].split_off(split_point);
        p2_stacks[dst - 1].extend(load);
    }

    for s in p2_stacks {
        print!("{}", s.last().unwrap());
    }
    println!();
}

#[derive(Clone, Copy, Debug)]
struct Command {
    cnt: usize,
    src: usize,
    dst: usize,
}
