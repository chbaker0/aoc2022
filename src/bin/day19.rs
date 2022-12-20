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

use lazy_static::lazy_static;

fn main() {
    let blueprints: Vec<Blueprint> = std::io::stdin()
        .lines()
        .map(|l| parse_blueprint(&l.unwrap()))
        .collect();

    // for (i, blueprint) in blueprints.iter().enumerate() {
    //     println!("{blueprint:#?}");
    //     println!("{i}: {}", blueprint.solve());
    // }

    let p1 = blueprints
        .iter()
        .enumerate()
        .map(|(i, b)| (i + 1) as u32 * b.solve(24))
        .sum::<u32>();
    let p2 = blueprints
        .iter()
        .take(3)
        .map(|b| b.solve(32))
        .product::<u32>();
    println!("{p1} {p2}");
}

fn parse_blueprint(desc: &str) -> Blueprint {
    use regex::Regex;

    lazy_static! {
        static ref BLUEPRINT_REGEX: Regex = {
            Regex::new(r#"Blueprint [0-9]+: Each ore robot costs (?P<ore>[ a-zA-Z0-9]+)\. Each clay robot costs (?P<clay>[ a-zA-Z0-9]+)\. Each obsidian robot costs (?P<obsidian>[ a-zA-Z0-9]+)\. Each geode robot costs (?P<geode>[ a-zA-Z0-9]+)\.$"#).unwrap()
        };
        static ref COST_REGEX: Regex = {
            Regex::new(r#"(?P<ore>[0-9]+) ore(?: and (?P<clay>[0-9]+) clay)?(?: and (?P<obsidian>[0-9]+) obsidian)?$"#).unwrap()
        };
    }

    let cost_matches = BLUEPRINT_REGEX.captures(desc).unwrap();
    assert_eq!(cost_matches.get(0).unwrap().as_str(), desc);

    fn parse_cost(desc: &str) -> Resources {
        let fields = COST_REGEX.captures(desc).unwrap();
        assert_eq!(fields.get(0).unwrap().as_str(), desc);
        let get_field = |name| (|| fields.name(name)?.as_str().parse::<u32>().ok())().unwrap_or(0);
        Resources {
            ore: get_field("ore"),
            clay: get_field("clay"),
            obsidian: get_field("obsidian"),
            geode: 0,
        }
    }

    Blueprint {
        ore_cost: parse_cost(cost_matches.name("ore").unwrap().as_str()),
        clay_cost: parse_cost(cost_matches.name("clay").unwrap().as_str()),
        obsidian_cost: parse_cost(cost_matches.name("obsidian").unwrap().as_str()),
        geode_cost: parse_cost(cost_matches.name("geode").unwrap().as_str()),
    }
}

#[derive(Clone, Copy, Debug)]
struct Blueprint {
    ore_cost: Resources,
    clay_cost: Resources,
    obsidian_cost: Resources,
    geode_cost: Resources,
}

impl Blueprint {
    fn solve(&self, num_minutes: usize) -> u32 {
        use good_lp::*;

        #[derive(Clone, Copy, Debug)]
        struct RoundVariables {
            // Action to take this round.
            // build_nothing: Variable,
            build_ore: Variable,
            build_clay: Variable,
            build_obsidian: Variable,
            build_geode: Variable,

            // Number of operational bots at beginning of round.
            numbots_ore: Variable,
            numbots_clay: Variable,
            numbots_obsidian: Variable,
            numbots_geode: Variable,

            // Amount of resources available at end of round.
            total_ore: Variable,
            total_clay: Variable,
            total_obsidian: Variable,
            total_geode: Variable,
        }

        let mut round_variables = Vec::<RoundVariables>::new();

        let vars = std::cell::RefCell::new(ProblemVariables::new());
        for minute in 0..num_minutes {
            let make_choice = |choice| {
                vars.borrow_mut()
                    .add(variable().binary().name(format!("{minute}_build_{choice}")))
            };
            let make_bots = |resource| {
                vars.borrow_mut().add(
                    variable()
                        .integer()
                        .name(format!("{minute}_numbots_{resource}")),
                )
            };
            let make_total = |resource| {
                vars.borrow_mut().add(
                    variable()
                        .integer()
                        .name(format!("{minute}_total_{resource}")),
                )
            };
            round_variables.push(RoundVariables {
                // build_nothing: make_choice("nothing"),
                build_ore: make_choice("ore"),
                build_clay: make_choice("clay"),
                build_obsidian: make_choice("obsidian"),
                build_geode: make_choice("geode"),
                numbots_ore: make_bots("ore"),
                numbots_clay: make_bots("clay"),
                numbots_obsidian: make_bots("obsidian"),
                numbots_geode: make_bots("geode"),
                total_ore: make_total("ore"),
                total_clay: make_total("clay"),
                total_obsidian: make_total("obsidian"),
                total_geode: make_total("geode"),
            });
        }
        let vars = vars.into_inner();

        let rfinal = round_variables.last().unwrap();
        let max_geodes = rfinal.total_geode;
        let mut prob = vars.maximise(max_geodes.clone()).using(default_solver);

        let r1 = round_variables[0];
        prob.add_constraint(constraint::eq(r1.build_ore, 0));
        prob.add_constraint(constraint::eq(r1.build_clay, 0));
        prob.add_constraint(constraint::eq(r1.build_obsidian, 0));
        prob.add_constraint(constraint::eq(r1.build_geode, 0));
        prob.add_constraint(constraint::eq(r1.numbots_ore, 1));
        prob.add_constraint(constraint::eq(r1.numbots_clay, 0));
        prob.add_constraint(constraint::eq(r1.numbots_obsidian, 0));
        prob.add_constraint(constraint::eq(r1.numbots_geode, 0));
        prob.add_constraint(constraint::eq(r1.total_ore, 1));
        prob.add_constraint(constraint::eq(r1.total_clay, 0));
        prob.add_constraint(constraint::eq(r1.total_obsidian, 0));
        prob.add_constraint(constraint::eq(r1.total_geode, 0));

        for minute in 1..num_minutes {
            let rv_prev = round_variables[minute - 1];
            let rv = round_variables[minute];

            // Can only do one action per minute.
            prob.add_constraint(constraint::leq(
                /* rv.build_nothing
                + */
                rv.build_ore + rv.build_clay + rv.build_obsidian + rv.build_geode,
                1,
            ));

            // Number of operational bots this minute is number from last round plus the bot that was constructed.
            prob.add_constraint(constraint::eq(
                rv.numbots_ore,
                rv_prev.numbots_ore + rv_prev.build_ore,
            ));
            prob.add_constraint(constraint::eq(
                rv.numbots_clay,
                rv_prev.numbots_clay + rv_prev.build_clay,
            ));
            prob.add_constraint(constraint::eq(
                rv.numbots_obsidian,
                rv_prev.numbots_obsidian + rv_prev.build_obsidian,
            ));
            prob.add_constraint(constraint::eq(
                rv.numbots_geode,
                rv_prev.numbots_geode + rv_prev.build_geode,
            ));

            let ore_cost = rv.build_ore * self.ore_cost.ore
                + rv.build_clay * self.clay_cost.ore
                + rv.build_obsidian * self.obsidian_cost.ore
                + rv.build_geode * self.geode_cost.ore;
            let clay_cost = rv.build_obsidian * self.obsidian_cost.clay;
            let obsidian_cost = rv.build_geode * self.geode_cost.obsidian;

            // Cannot dip below 0 for any resource when constructing a bot.
            prob.add_constraint(constraint::geq(rv_prev.total_ore, ore_cost.clone()));
            prob.add_constraint(constraint::geq(rv_prev.total_clay, clay_cost.clone()));
            prob.add_constraint(constraint::geq(
                rv_prev.total_obsidian,
                obsidian_cost.clone(),
            ));

            // Total resources at end of this round is total from end of prev
            // round, plus number of bots operational this round, minus cost of
            // building bot this round.
            prob.add_constraint(constraint::eq(
                rv.total_ore,
                rv_prev.total_ore + rv.numbots_ore - ore_cost,
            ));
            prob.add_constraint(constraint::eq(
                rv.total_clay,
                rv_prev.total_clay + rv.numbots_clay - clay_cost,
            ));
            prob.add_constraint(constraint::eq(
                rv.total_obsidian,
                rv_prev.total_obsidian + rv.numbots_obsidian - obsidian_cost,
            ));
            prob.add_constraint(constraint::eq(
                rv.total_geode,
                rv_prev.total_geode + rv.numbots_geode,
            ));
        }

        let sol = prob.solve().unwrap();

        // for minute in 0..24 {
        //     println!("== Minute {minute} ==");
        //     let rv = round_variables[minute].as_ref().unwrap();
        //     for v in [
        //         // rv.build_nothing,
        //         rv.build_ore,
        //         rv.build_clay,
        //         rv.build_obsidian,
        //         rv.build_geode,
        //     ] {
        //         print!("{} ", sol.eval(v));
        //     }
        //     println!();
        //     for v in [
        //         rv.numbots_ore,
        //         rv.numbots_clay,
        //         rv.numbots_obsidian,
        //         rv.numbots_geode,
        //     ] {
        //         print!("{} ", sol.eval(v));
        //     }
        //     println!();
        //     for v in [
        //         rv.total_ore,
        //         rv.total_clay,
        //         rv.total_obsidian,
        //         rv.total_geode,
        //     ] {
        //         print!("{} ", sol.eval(v));
        //     }
        //     println!();
        // }

        sol.eval(max_geodes) as u32
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}
