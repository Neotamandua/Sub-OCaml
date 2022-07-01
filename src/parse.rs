// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
#![allow(non_camel_case_types)]

use crate::lex::{Constant, Token};

pub fn parse(l: Vec<Token>) -> (exp, Vec<Token>) {
    return exp(l);
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ty {
    Bool,
    Int,
    Arrow(Box<ty>, Box<ty>),
}

#[derive(Debug)]
pub enum con {
    Bcon(bool),
    Icon(isize),
}
#[derive(Debug)]
pub enum operator {
    Add,
    Sub,
    Mul,
    Leq,
    Geq,
    Eq,
    Lt,
    Gt,
}

type var = String;

#[derive(Debug)]
pub enum exp {
    Var(var),
    Con(con),
    Oapp(operator, Box<exp>, Box<exp>),
    Fapp(Box<exp>, Box<exp>),
    If(Box<exp>, Box<exp>, Box<exp>),
    Lam(var, Box<exp>),
    Lamty(var, ty, Box<exp>),
    Let(var, Box<exp>, Box<exp>),
    Letrec(var, var, Box<exp>, Box<exp>),
    Letrecty(var, var, ty, ty, Box<exp>, Box<exp>),
}

fn verify(token: Token, tokenlist: Vec<Token>) -> Vec<Token> {
    match tokenlist.as_slice() {
        [] => panic!("Verify failed: No token"),
        [t, l @ ..] => {
            if *t == token {
                l.to_vec()
            } else {
                panic!("Verify failed: wrong token")
            }
        }
    }
}

fn ty(l: Vec<Token>) -> (ty, Vec<Token>) {
    let (t, l) = pty(l);
    return ty2(t, l);

    fn ty2(t1: ty, l: Vec<Token>) -> (ty, Vec<Token>) {
        match l.as_slice() {
            [Token::ARR, l @ ..] => {
                let (t2, l) = pty(l.to_vec());
                let (t, l) = ty2(t2, l);
                return (ty::Arrow(Box::new(t1), Box::new(t)), l);
            }
            l => (t1, l.to_vec()),
        }
    }

    fn pty(l: Vec<Token>) -> (ty, Vec<Token>) {
        match l.as_slice() {
            [Token::VAR(x), l @ ..] => {
                if *x == "bool".as_ref() {
                    (ty::Bool, l.to_vec())
                } else if *x == "int".as_ref() {
                    (ty::Int, l.to_vec())
                } else {
                    panic!("Parser: type error")
                }
            }
            [Token::LP, l @ ..] => {
                let (t, l) = ty(l.to_vec());
                return (t, verify(Token::RP, l));
            }
            _ => panic!("Parser: type error"),
        }
    }
}

fn exp(l: Vec<Token>) -> (exp, Vec<Token>) {
    match l.as_slice() {
        [Token::IF, l @ ..] => {
            let (e1, l) = exp(l.to_vec());
            let (e2, l) = exp(verify(Token::THEN, l));
            let (e3, l) = exp(verify(Token::ELSE, l));

            return (
                exp::If(Box::new(e1), Box::new(e2), Box::new(e3)),
                l.to_vec(),
            );
        }
        [Token::LAM, Token::VAR(x), Token::ARR, l @ ..] => {
            let (e, l) = exp(l.to_vec());

            return (exp::Lam(x.to_string(), Box::new(e)), l);
        }
        [Token::LAM, Token::LP, Token::VAR(x), Token::COL, l @ ..] => {
            let (t, l) = ty(l.to_vec());
            let (e, l) = exp(verify(Token::ARR, verify(Token::RP, l)));
            return (exp::Lamty(x.to_string(), t, Box::new(e)), l);
        }
        [Token::LET, Token::VAR(x), Token::EQ, l @ ..] => {
            let (e1, l) = exp(l.to_vec());
            let (e2, l) = exp(verify(Token::IN, l));

            return (exp::Let(x.to_string(), Box::new(e1), Box::new(e2)), l);
        }
        [Token::LET, Token::REC, Token::VAR(f), Token::VAR(x), Token::EQ, l @ ..] => {
            let (e1, l) = exp(l.to_vec());
            let (e2, l) = exp(verify(Token::IN, l));

            return (
                exp::Letrec(f.to_string(), x.to_string(), Box::new(e1), Box::new(e2)),
                l,
            );
        }
        [Token::LET, Token::REC, Token::VAR(f), Token::LP, Token::VAR(x), Token::COL, l @ ..] => {
            let (t1, l) = ty(l.to_vec());
            let (t2, l) = ty(verify(Token::COL, verify(Token::RP, l)));
            let (e1, l) = exp(verify(Token::EQ, l));
            let (e2, l) = exp(verify(Token::IN, l));

            return (
                exp::Letrecty(
                    x.to_string(),
                    f.to_string(),
                    t1,
                    t2,
                    Box::new(e1),
                    Box::new(e2),
                ),
                l,
            );
        }
        l => return cexp(l.to_vec()),
    }

    fn cexp(l: Vec<Token>) -> (exp, Vec<Token>) {
        let (e, l) = sexp(l.to_vec());
        return cexp_la(e, l);
    }

    fn cexp_la(e1: exp, l: Vec<Token>) -> (exp, Vec<Token>) {
        match l.as_slice() {
            [Token::LEQ, l @ ..] => {
                let (e2, l) = sexp(l.to_vec());
                return (exp::Oapp(operator::Leq, Box::new(e1), Box::new(e2)), l);
            }
            l => (e1, l.to_vec()),
        }
    }

    fn sexp(l: Vec<Token>) -> (exp, Vec<Token>) {
        let (e, l) = mexp(l);
        return sexp_la(e, l);
    }

    fn sexp_la(e1: exp, l: Vec<Token>) -> (exp, Vec<Token>) {
        match l.as_slice() {
            [Token::ADD, l @ ..] => {
                let (e2, l) = mexp(l.to_vec());
                return sexp_la(exp::Oapp(operator::Add, Box::new(e1), Box::new(e2)), l);
            }
            [Token::SUB, l @ ..] => {
                let (e2, l) = mexp(l.to_vec());
                return sexp_la(exp::Oapp(operator::Sub, Box::new(e1), Box::new(e2)), l);
            }
            l => (e1, l.to_vec()),
        }
    }

    fn mexp(l: Vec<Token>) -> (exp, Vec<Token>) {
        let (e, l) = aexp(l);
        return mexp_la(e, l);
    }

    fn mexp_la(e1: exp, l: Vec<Token>) -> (exp, Vec<Token>) {
        match l.as_slice() {
            [Token::MUL, l @ ..] => {
                let (e2, l) = aexp(l.to_vec());
                return mexp_la(exp::Oapp(operator::Mul, Box::new(e1), Box::new(e2)), l);
            }
            l => (e1, l.to_vec()),
        }
    }

    fn aexp(l: Vec<Token>) -> (exp, Vec<Token>) {
        let (e, l) = pexp(l);
        return aexp_la(e, l);
    }

    fn aexp_la(e1: exp, l: Vec<Token>) -> (exp, Vec<Token>) {
        match l.as_slice() {
            [Token::CON(_), n @ ..] | [Token::VAR(_), n @ ..] | [Token::LP, n @ ..] => {
                let (e2, l) = pexp(l.to_vec());
                return aexp_la(exp::Fapp(Box::new(e1), Box::new(e2)), l.to_vec());
            }
            _ => return (e1, l),
        }
    }
    fn pexp(l: Vec<Token>) -> (exp, Vec<Token>) {
        match l.as_slice() {
            [Token::CON(Constant::BCON(b)), l @ ..] => (exp::Con(con::Bcon(*b)), l.to_vec()),
            [Token::CON(Constant::ICON(n)), l @ ..] => (exp::Con(con::Icon(*n)), l.to_vec()),
            [Token::VAR(x), l @ ..] => (exp::Var(x.to_string()), l.to_vec()),
            [Token::LP, l @ ..] => {
                let (e, l) = exp(l.to_vec());
                return (e, verify(Token::RP, l));
            }
            e => {
                println!("MATCH {:?}", e);
                panic!("pexp parse error")
            }
        }
    }
}
