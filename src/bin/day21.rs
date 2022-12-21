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

use std::collections::{hash_map, HashMap};

fn main() {
    let mut monkey_ids = HashMap::new();
    let mut cur_monkey_id = 0;
    let jobs: HashMap<MonkeyId, Job> = std::io::stdin()
        .lines()
        .map(|l| parse_line(&l.unwrap(), &mut monkey_ids, &mut cur_monkey_id))
        .collect();

    let root = *monkey_ids.get("root").unwrap();

    println!("{}", eval(root, &jobs));

    let humn = *monkey_ids.get("humn").unwrap();

    let mut cur_expr_id = 0;
    let mut sub_exprs = HashMap::new();
    let expr = make_expr(root, humn, &jobs, &mut sub_exprs, &mut cur_expr_id);

    println!("{}", count_var(&expr, &sub_exprs));
}

fn eval(monkey: MonkeyId, jobs: &HashMap<MonkeyId, Job>) -> Num {
    let job = jobs.get(&monkey).unwrap();

    match job {
        Job::Const(n) => *n,
        Job::Op(op, lhs, rhs) => {
            let left = eval(*lhs, jobs);
            let right = eval(*rhs, jobs);
            use Operation::*;
            match op {
                Add => left + right,
                Sub => left - right,
                Mul => left * right,
                Div => left / right,
            }
        }
    }
}

fn make_expr(
    monkey: MonkeyId,
    humn: MonkeyId,
    jobs: &HashMap<MonkeyId, Job>,
    exprs: &mut HashMap<ExprId, Expr>,
    cur_expr_id: &mut u64,
) -> Expr {
    if monkey == humn {
        return Expr::Var;
    }

    let job = jobs.get(&monkey).unwrap();

    match job {
        Job::Const(n) => Expr::Const(*n),
        Job::Op(op, lhs, rhs) => {
            *cur_expr_id += 1;
            let left_id = ExprId(*cur_expr_id);
            let left_expr = make_expr(*lhs, humn, jobs, exprs, cur_expr_id);
            exprs.insert(left_id, left_expr);
            *cur_expr_id += 1;
            let right_id = ExprId(*cur_expr_id);
            let right_expr = make_expr(*rhs, humn, jobs, exprs, cur_expr_id);
            exprs.insert(right_id, right_expr);
            Expr::Op(*op, left_id, right_id)
        }
    }
}

fn count_var(expr: &Expr, sub_exprs: &HashMap<ExprId, Expr>) -> u64 {
    match expr {
        Expr::Const(_) => 0,
        Expr::Var => 1,
        Expr::Op(_, lhs, rhs) => {
            count_var(sub_exprs.get(lhs).unwrap(), sub_exprs) + count_var(sub_exprs.get(rhs).unwrap(), sub_exprs)
        }
    }
}

fn reduce(expr: &mut Expr, sub_exprs: &mut HashMap<ExprId, Expr>) {
    match expr {
        Expr::Const(_) => (),
        Expr::Var => (),
        Expr::Op(op, lhs, rhs) => {
            let lhs = *lhs;
            let rhs = *rhs;
            let mut left = sub_exprs.remove(&lhs).unwrap();
            let mut right = sub_exprs.remove(&rhs).unwrap();
            reduce(&mut left, sub_exprs);
            reduce(&mut right, sub_exprs);
            match (&left, &right) {
                (Expr::Const(l), Expr::Const(r)) => *expr = Expr::Const(op.apply(*l, *r)),
                _ => (),
            }
            sub_exprs.insert(lhs, left);
            sub_exprs.insert(rhs, right);
        }
    }
}

fn get_first_subexpr_with_var<'e>(
    expr: &'e Expr,
    sub_exprs: &'e HashMap<ExprId, Expr>,
) -> Option<&'e Expr> {
    match expr {
        Expr::Const(_) => None,
        Expr::Var => Some(expr),
        Expr::Op(_, lhs, rhs) => get_first_subexpr_with_var(sub_exprs.get(lhs).unwrap(), sub_exprs)
            .or_else(|| get_first_subexpr_with_var(sub_exprs.get(rhs).unwrap(), sub_exprs)),
    }
}

fn distribute(expr: &mut Expr, factor: Option<Expr>, sub_exprs: &mut HashMap<ExprId, Expr>) {
    match expr {
        Expr::Const(_) => (),
        Expr::Var => (),
        Expr::Op(op, lhs, rhs) => {
            let lhs = *lhs;
            let rhs = *rhs;
            let mut left = sub_exprs.remove(&lhs).unwrap();
            let mut right = sub_exprs.remove(&rhs).unwrap();
            reduce(&mut left, sub_exprs);
            reduce(&mut right, sub_exprs);
            match (&left, &right) {
                (Expr::Const(l), Expr::Const(r)) => *expr = Expr::Const(op.apply(*l, *r)),
                _ => (),
            }
            sub_exprs.insert(lhs, left);
            sub_exprs.insert(rhs, right);
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Solution {
    Const(Num),
    Expr(Operation, MonkeyId, MonkeyId),
}

fn parse_line(
    line: &str,
    monkey_ids: &mut HashMap<String, MonkeyId>,
    cur_monkey_id: &mut u32,
) -> (MonkeyId, Job) {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        static ref LINE_REGEX: Regex = {
            Regex::new(r#"(?P<name>[a-z]{4}): (?:(?P<const>[0-9]+)|(?:(?P<lhs>[a-z]{4}) (?P<op>[-+*/]) (?P<rhs>[a-z]{4})))$"#).unwrap()
        };
    }

    let caps = LINE_REGEX.captures(line).unwrap();
    assert_eq!(caps.get(0).unwrap().as_str(), line);

    let mut get_monkey_id = |name: &str| {
        use hash_map::Entry;
        match monkey_ids.entry(name.to_string()) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => {
                *cur_monkey_id += 1;
                *e.insert(MonkeyId(*cur_monkey_id))
            }
        }
    };

    let monkey_id = get_monkey_id(caps.name("name").unwrap().as_str());

    if let Some(m) = caps.name("const") {
        let num: Num = m.as_str().parse().unwrap();
        return (monkey_id, Job::Const(num));
    }

    let lhs = get_monkey_id(caps.name("lhs").unwrap().as_str());
    let rhs = get_monkey_id(caps.name("rhs").unwrap().as_str());

    let op = match caps.name("op").unwrap().as_str() {
        "+" => Operation::Add,
        "-" => Operation::Sub,
        "*" => Operation::Mul,
        "/" => Operation::Div,
        op => unreachable!("invalid op {op}"),
    };

    (monkey_id, Job::Op(op, lhs, rhs))
}

#[derive(Clone, Debug)]
enum Expr {
    Const(Num),
    Var,
    Op(Operation, ExprId, ExprId),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct ExprId(u64);

#[derive(Clone, Copy, Debug)]
enum Job {
    Const(Num),
    Op(Operation, MonkeyId, MonkeyId),
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    fn apply(&self, lhs: Num, rhs: Num) -> Num {
        use Operation::*;
        match *self {
            Add => lhs + rhs,
            Sub => lhs - rhs,
            Mul => lhs * rhs,
            Div => lhs / rhs,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct MonkeyId(u32);

type Num = i64;
