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

// NOTE: this doesn't work. I assumed I only need to keep track of the top rock
// in each column, but that's not true. The jets can blow a falling rock around
// an obstacle, so deeper rocks must be accounted for.

fn main() {
    let jets: Vec<JetDir> = std::io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| match c {
            '<' => JetDir::Left,
            '>' => JetDir::Right,
            _ => unreachable!(),
        })
        .collect();

    let mut chamber = Chamber {
        heights: [0; CHAMBER_WIDTH],
    };

    let mut rock_cycle = ROCKS.iter().cycle();
    let mut jet_cycle = jets.iter().cycle().copied();
    for i in 0..2022 {
        drop_rock(&mut chamber, rock_cycle.next().unwrap(), &mut jet_cycle);
        // println!("{i}");
        // println!("{:?}", chamber.heights);
    }

    println!("{}", chamber.max_height());
}

fn drop_rock<I: IntoIterator<Item = JetDir>>(chamber: &mut Chamber, rock: &Rock, jets: I) {
    let mut jets = jets.into_iter();
    let mut rock_x: isize = 2;
    let mut rock_y: isize = chamber.max_height() as isize + 4;
    loop {
        let mut try_x = rock_x;
        let mut try_y = rock_y;
        assert!(!chamber.intersects(rock, try_x, try_y));

        // println!("{rock_x}, {rock_y}");

        try_x += match jets.next().unwrap() {
            JetDir::Left => -1,
            JetDir::Right => 1,
        };

        if chamber.intersects(rock, try_x, try_y) {
            try_x = rock_x;
        }

        try_y -= 1;
        if chamber.intersects(rock, try_x, try_y) {
            try_y = rock_y;
            chamber.update(rock, try_x.try_into().unwrap(), try_y.try_into().unwrap());
            return;
        }

        rock_x = try_x;
        rock_y = try_y;
    }
}

struct Chamber {
    heights: [usize; CHAMBER_WIDTH],
}

impl Chamber {
    fn max_height(&self) -> usize {
        self.heights.iter().copied().max().unwrap()
    }

    fn intersects(&self, rock: &Rock, pos_x: isize, pos_y: isize) -> bool {
        for cur_y in 0..ROCK_DIMENSION {
            for cur_x in 0..ROCK_DIMENSION {
                if !rock.at(cur_x, cur_y) {
                    continue;
                }

                if (pos_y + cur_y as isize) < 0 || (pos_x + cur_x as isize) < 0 {
                    return true;
                }

                let x: usize = (pos_x + cur_x as isize).try_into().unwrap();
                let y: usize = (pos_y + cur_y as isize).try_into().unwrap();

                if x >= CHAMBER_WIDTH {
                    return true;
                }

                if y <= self.heights[x] {
                    return true;
                }
            }
        }

        false
    }

    fn update(&mut self, rock: &Rock, pos_x: usize, pos_y: usize) {
        for cur_y in 0..ROCK_DIMENSION {
            for cur_x in 0..ROCK_DIMENSION {
                let elem = rock.at(cur_x, cur_y);
                if !elem {
                    continue;
                }

                let col_height: &mut _ = &mut self.heights[cur_x + pos_x];
                *col_height = std::cmp::max(*col_height, cur_y + pos_y);
            }
        }
    }
}

const CHAMBER_WIDTH: usize = 7;

#[derive(Clone, Copy, Debug)]
enum JetDir {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
struct Rock {
    bmp: [[bool; ROCK_DIMENSION]; ROCK_DIMENSION],
}

const ROCK_DIMENSION: usize = 4;

impl Rock {
    fn print(&self) {
        for row in self.bmp {
            for c in row {
                print!("{}", if c { '#' } else { '.' });
            }
            println!();
        }
    }

    fn at(&self, x: usize, y: usize) -> bool {
        self.bmp[ROCK_DIMENSION - y - 1][x]
    }
}

const ROCKS: [Rock; 5] = [WIDE_ROCK, PLUS_ROCK, ANGLE_ROCK, TALL_ROCK, SQUARE_ROCK];

const WIDE_ROCK: Rock = Rock {
    bmp: [[false; 4], [false; 4], [false; 4], [true; 4]],
};

const PLUS_ROCK: Rock = Rock {
    bmp: [
        [false; 4],
        [false, true, false, false],
        [true, true, true, false],
        [false, true, false, false],
    ],
};

const ANGLE_ROCK: Rock = Rock {
    bmp: [
        [false; 4],
        [false, false, true, false],
        [false, false, true, false],
        [true, true, true, false],
    ],
};

const TALL_ROCK: Rock = Rock {
    bmp: [[true, false, false, false]; 4],
};

const SQUARE_ROCK: Rock = Rock {
    bmp: [
        [false; 4],
        [false; 4],
        [true, true, false, false],
        [true, true, false, false],
    ],
};
