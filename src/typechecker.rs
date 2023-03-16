// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use crate::error::{Result, TypeCheckError};
use crate::parse::{con, exp, operator, ty};
use std::collections::HashMap;

fn check_operator(o: operator, t1: ty, t2: ty) -> Result<ty> {
    match (o, t1, t2) {
        (operator::Add, ty::Int, ty::Int)
        | (operator::Sub, ty::Int, ty::Int)
        | (operator::Mul, ty::Int, ty::Int) => Ok(ty::Int),
        (operator::Leq, ty::Int, ty::Int) => Ok(ty::Bool),
        (_, _, _) => Err(TypeCheckError::ArgumentError)?,
    }
}

fn check_fun(t1: ty, t2: ty) -> Result<ty> {
    match t1 {
        ty::Arrow(t1_, t2_) => {
            if *t1_ == t2 {
                return Ok(*t2_);
            } else {
                Err(TypeCheckError::WrongArgument)?
            }
        }
        _ => Err(TypeCheckError::MissingFunction)?,
    }
}

pub fn type_check(env: &mut HashMap<String, ty>, e: exp) -> Result<ty> {
    match e {
        exp::Var(x) => match env.get(&x) {
            Some(t) => Ok(t.clone()),
            None => Err(TypeCheckError::UnboundVariable(x))?,
        },
        exp::Con(con::Bcon(b)) => Ok(ty::Bool),
        exp::Con(con::Icon(n)) => Ok(ty::Int),
        exp::Oapp(o, e1, e2) => Ok(check_operator(
            o,
            type_check(env, *e1)?,
            type_check(env, *e2)?,
        )?),
        exp::Fapp(e1, e2) => Ok(check_fun(type_check(env, *e1)?, type_check(env, *e2)?)?),
        exp::If(e1, e2, e3) => match (
            type_check(env, *e1)?,
            type_check(env, *e2)?,
            type_check(env, *e3)?,
        ) {
            (ty::Bool, t2, t3) => {
                if t2 == t3 {
                    return Ok(t2);
                } else {
                    Err(TypeCheckError::UnequalIfTypes)?
                }
            }
            (x, _, _) => {
                let x: String = format!("{:?}", x);
                Err(TypeCheckError::WrongIfType(x))?
            }
        },
        exp::Lam(_, _) => Err(TypeCheckError::MissingFunctionType)?,
        exp::Lamty(x, t, e) => {
            env.insert(x, t.clone());
            Ok(ty::Arrow(Box::new(t), Box::new(type_check(env, *e)?)))
        }
        exp::Let(x, e1, e2) => {
            let t = type_check(env, *e1)?;
            env.insert(x, t);
            type_check(env, *e2)
        }
        exp::Letrec(f, x, e1, e2) => Err(TypeCheckError::MissingType)?,
        exp::Letrecty(f, x, t1, t2, e1, e2) => {
            env.insert(f, ty::Arrow(Box::new(t1.clone()), Box::new(t2.clone())));
            let mut new_env = env.clone();
            new_env.insert(x, t1);
            if type_check(&mut new_env, *e1)? == t2 {
                return type_check(env, *e2);
            } else {
                Err(TypeCheckError::NoTypeMatch)?
            }
        }
    }
}
