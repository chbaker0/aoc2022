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

use nom::Finish;

fn main() {
    let paths: Vec<RockPath> = std::io::stdin()
        .lines()
        .map(|line| parse_path(&line.unwrap()).finish().unwrap().1)
        .collect();

    let floor_y = paths
        .iter()
        .flat_map(|p| &p.points)
        .map(|pt| pt.y)
        .max()
        .unwrap()
        + 2;
    let mut map = draw_map(&paths);

    let mut p1_map = map.clone();
    let p1 = match compute(CavePoint { x: 500, y: 0 }, &mut p1_map, None) {
        ComputeResult::Flow(x) => x,
        _ => unreachable!(),
    };
    println!("{}", p1);

    let p2 = match compute(CavePoint { x: 500, y: 0 }, &mut map, Some(floor_y)) {
        ComputeResult::Rest(x) => x,
        _ => unreachable!(),
    };
    println!("{}", p2);
}

fn compute(pos: CavePoint, map: &mut CaveMap, floor_y: Option<u32>) -> ComputeResult {
    use ComputeResult::*;

    if let Some(floor_y) = floor_y {
        if pos.y == floor_y {
            return Rest(0);
        }
    } else if pos.x as usize >= CAVE_WIDTH || pos.y as usize >= CAVE_HEIGHT {
        return Flow(0);
    }

    if map.map[pos.x as usize][pos.y as usize] != CaveElem::Air {
        return Rest(0);
    }

    let mut total = 0;

    for x in [pos.x, pos.x - 1, pos.x + 1] {
        total += match compute(CavePoint { x, y: pos.y + 1 }, map, floor_y) {
            Rest(x) => x,
            Flow(x) => return Flow(total + x),
        };
    }

    map.map[pos.x as usize][pos.y as usize] = CaveElem::Sand;
    Rest(total + 1)
}

enum ComputeResult {
    /// Sand has come to rest after n units.
    Rest(u32),
    /// Sand is flowing into the abyss after n units.
    Flow(u32),
}

#[derive(Clone, Copy, Debug)]
struct CaveMap {
    map: [[CaveElem; CAVE_HEIGHT]; CAVE_WIDTH],
}

const CAVE_WIDTH: usize = 1000;
const CAVE_HEIGHT: usize = 200;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CaveElem {
    Air,
    Rock,
    Sand,
}

fn draw_map(paths: &[RockPath]) -> CaveMap {
    let mut map = [[CaveElem::Air; CAVE_HEIGHT]; CAVE_WIDTH];
    for path in paths {
        for win in path.points.windows(2) {
            let (mut p1, mut p2) = match win {
                [p1, p2] => (p1, p2),
                _ => unreachable!(),
            };

            if p1.x != p2.x {
                if p1.x > p2.x {
                    std::mem::swap(&mut p1, &mut p2);
                }
                for x in p1.x..=p2.x {
                    map[x as usize][p1.y as usize] = CaveElem::Rock;
                }
            } else if p1.y != p2.y {
                if p1.y > p2.y {
                    std::mem::swap(&mut p1, &mut p2);
                }
                for y in p1.y..=p2.y {
                    map[p1.x as usize][y as usize] = CaveElem::Rock;
                }
            }
        }
    }

    CaveMap { map }
}

#[derive(Clone, Debug)]
struct RockPath {
    points: Vec<CavePoint>,
}

#[derive(Clone, Copy, Debug)]
struct CavePoint {
    x: u32,
    y: u32,
}

fn parse_path(path_desc: &str) -> nom::IResult<&str, RockPath> {
    use bytes::complete::tag;
    use nom::*;

    let parse_pair = |input| {
        let (input, (x, y)) =
            sequence::separated_pair(character::complete::u32, tag(","), character::complete::u32)(
                input,
            )?;
        IResult::Ok((input, CavePoint { x, y }))
    };

    let (input, points) =
        nom::multi::separated_list1(nom::bytes::complete::tag(" -> "), parse_pair)(path_desc)?;
    Ok((input, RockPath { points }))
}
