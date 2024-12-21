// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_camel_case_types)]

use crate::error::{EvaluatorError, Result, TypeCheckError};
use crate::parse::{con, exp, operator, var};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum value {
    BoolValue(bool),
    IntValue(isize),
    Closure(var, exp, BTreeMap<var, Box<value>>),
    Rclosure(var, var, exp, BTreeMap<var, Box<value>>),
}

fn eval_operator(o: operator, v1: value, v2: value) -> Result<value> {
    match (o, v1, v2) {
        (operator::Add, value::IntValue(x), value::IntValue(y)) => Ok(value::IntValue(x + y)),
        (operator::Sub, value::IntValue(x), value::IntValue(y)) => Ok(value::IntValue(x - y)),
        (operator::Mul, value::IntValue(x), value::IntValue(y)) => Ok(value::IntValue(x * y)),
        (operator::Leq, value::IntValue(x), value::IntValue(y)) => Ok(value::BoolValue(x <= y)),
        (_, _, _) => Err(EvaluatorError::ArgumentError)?,
    }
}

fn eval_fun(v1: value, v2: value) -> Result<value> {
    match v1.clone() {
        value::Closure(x, e, mut env) => {
            env.insert(x, Box::new(v2));
            evaluate(&mut env, e)
        }
        value::Rclosure(f, x, e, mut env) => {
            env.insert(f, Box::new(v1));
            env.insert(x, Box::new(v2));
            evaluate(&mut env, e)
        }
        _ => Err(EvaluatorError::MissingFunction)?,
    }
}

pub fn evaluate(env: &mut BTreeMap<String, Box<value>>, e: exp) -> Result<value> {
    match e {
        exp::Var(x) => match env.get(&x) {
            Some(v) => Ok(*v.clone()),
            None => Err(TypeCheckError::UnboundVariable(x))?,
        },
        exp::Con(con::Bcon(b)) => Ok(value::BoolValue(b)),
        exp::Con(con::Icon(n)) => Ok(value::IntValue(n)),
        exp::Oapp(o, e1, e2) => eval_operator(o, evaluate(env, *e1)?, evaluate(env, *e2)?),
        exp::Fapp(e1, e2) => eval_fun(evaluate(env, *e1)?, evaluate(env, *e2)?),
        exp::If(e1, e2, e3) => match evaluate(env, *e1)? {
            value::BoolValue(b) => evaluate(env, b.then(|| *e2).unwrap_or(*e3)),
            x => {
                let x: String = format!("{:?}", x);
                Err(EvaluatorError::WrongIfType(x))?
            }
        },
        exp::Lam(x, e) | exp::Lamty(x, _, e) => Ok(value::Closure(x, *e, env.clone())),
        exp::Let(x, e1, e2) => {
            let v = evaluate(env, *e1)?;
            env.insert(x, Box::new(v));
            evaluate(env, *e2)
        }
        exp::Letrec(f, x, e1, e2) | exp::Letrecty(f, x, _, _, e1, e2) => {
            env.insert(f.clone(), Box::new(value::Rclosure(f, x, *e1, env.clone())));
            evaluate(env, *e2)
        }
    }
}
