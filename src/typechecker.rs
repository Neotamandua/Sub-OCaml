// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use crate::parse::{con, exp, operator, ty};
use std::collections::HashMap;

fn check_operator(o: operator, t1: ty, t2: ty) -> ty {
    match (o, t1, t2) {
        (operator::Add, ty::Int, ty::Int)
        | (operator::Sub, ty::Int, ty::Int)
        | (operator::Mul, ty::Int, ty::Int) => ty::Int,
        (operator::Leq, ty::Int, ty::Int) => ty::Bool,
        (_, _, _) => panic!("operator application failed: ill-typed arguments"),
    }
}

fn check_fun(t1: ty, t2: ty) -> ty {
    match t1 {
        ty::Arrow(t1_, t2_) => {
            if *t1_ == t2 {
                return *t2_;
            } else {
                panic!("function application: wrong argument type")
            }
        }
        _ => panic!("function application: function expected but none given"),
    }
}

pub fn type_check(env: &mut HashMap<String, ty>, e: exp) -> ty {
    match e {
        exp::Var(x) => match env.get(&x) {
            Some(t) => t.clone(),
            None => panic!("variable {} is unbound", x),
        },
        exp::Con(con::Bcon(b)) => ty::Bool,
        exp::Con(con::Icon(n)) => ty::Int,
        exp::Oapp(o, e1, e2) => check_operator(o, type_check(env, *e1), type_check(env, *e2)),
        exp::Fapp(e1, e2) => check_fun(type_check(env, *e1), type_check(env, *e2)),
        exp::If(e1, e2, e3) => match (
            type_check(env, *e1),
            type_check(env, *e2),
            type_check(env, *e3),
        ) {
            (ty::Bool, t2, t3) => {
                if t2 == t3 {
                    return t2;
                } else {
                    panic!("If: types for branch cases (if-case, else-case) are not equal")
                }
            }
            (x, _, _) => panic!("If: bool expected, got {:?}", x),
        },
        exp::Lam(_, _) => panic!("fun: missing type"),
        exp::Lamty(x, t, e) => {
            env.insert(x, t.clone());
            ty::Arrow(Box::new(t), Box::new(type_check(env, *e)))
        }
        exp::Let(x, e1, e2) => {
            let t = type_check(env, *e1);
            env.insert(x, t);
            type_check(env, *e2)
        }
        exp::Letrec(f, x, e1, e2) => panic!("let rec: missing types"),
        exp::Letrecty(f, x, t1, t2, e1, e2) => {
            env.insert(f, ty::Arrow(Box::new(t1.clone()), Box::new(t2.clone())));
            let mut new_env = env.clone();
            new_env.insert(x, t1);
            if type_check(&mut new_env, *e1) == t2 {
                return type_check(env, *e2);
            } else {
                panic!("let rec: declared type not matched")
            }
        }
    }
}
