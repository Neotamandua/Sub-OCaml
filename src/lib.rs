// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod error;
mod evaluator;
mod lex;
mod parse;
mod typechecker;
mod utils;
pub use error::Result;
pub use evaluator::{evaluate, value};
pub use lex::{lex, Token};
pub use parse::{parse, ty};
pub use typechecker::type_check;
use std::collections::HashMap;

pub fn run_code(
    code: &str,
) -> Result<(HashMap<String, ty>, HashMap<String, Box<value>>, ty, value)> {
    let tokenlist: Vec<Token> = lex(&code)?;
    let ast = parse(tokenlist)?.0;
    let mut type_env: HashMap<String, ty> = HashMap::new();
    let typed = type_check(&mut type_env, ast.clone())?;
    let mut value_env: HashMap<String, Box<value>> = HashMap::new();
    let evaluated = evaluate(&mut value_env, ast)?;
    Ok((type_env, value_env, typed, evaluated))
}

pub fn run_code_with_persistent_environment<'a>(
    type_env: &'a mut HashMap<String, ty>,
    value_env: &'a mut HashMap<String, Box<value>>,
    code: &'a str,
) -> Result<(
    &'a mut HashMap<String, ty>,
    &'a mut HashMap<String, Box<value>>,
    ty,
    value,
)> {
    let tokenlist: Vec<Token> = lex(&code)?;
    let ast = parse(tokenlist)?.0;
    let typed = type_check(type_env, ast.clone())?;
    let evaluated = evaluate(value_env, ast)?;
    Ok((type_env, value_env, typed, evaluated))
}

#[cfg(test)]
mod tests {
    use crate::evaluator::value;
    use super::run_code;
    use crate::lex::Token;
    use crate::parse::ty;
    use std::collections::HashMap;

    #[test]
    fn test_all_int_1() {
        let src = "let x = 5 in x";
        println!("Code: {} \n", src);
        let tokenlist: Vec<Token> = super::lex(&src).unwrap();
        println!("After Lex: {:?} \n", tokenlist);
        let ast = super::parse(tokenlist).unwrap().0;
        println!("After Parse: {:?} \n", ast);
        let mut map: HashMap<String, ty> = HashMap::new();
        let typed = super::type_check(&mut map, ast.clone()).unwrap();
        println!("After Typecheck: {:?}", typed);
        let mut map: HashMap<String, Box<value>> = HashMap::new();
        let evaluated = super::evaluate(&mut map, ast);
        println!("After Evaluation: {:?}", evaluated);
        assert_eq!(typed, ty::Int);
    }

    #[test]
    fn test_all_int_2() {
        let src = "let rec fib (a:int) : int -> int -> int = fun (b:int) -> fun (n:int) -> if n <= 0 then a else fib (b) (a+b) (n-1) in fib 0 1 8";
        println!("Code: {} \n", src);
        let tokenlist: Vec<Token> = super::lex(&src).unwrap();
        println!("After Lex: {:?} \n", tokenlist);
        let ast = super::parse(tokenlist).unwrap().0;
        println!("After Parse: {:?} \n", ast);
        let mut map: HashMap<String, ty> = HashMap::new();
        let typed = super::type_check(&mut map, ast.clone()).unwrap();
        println!("After Typecheck: {:?}", typed);
        let mut map: HashMap<String, Box<value>> = HashMap::new();
        let evaluated = super::evaluate(&mut map, ast);
        println!("After Evaluation: {:?}", evaluated);

        assert_eq!(typed, ty::Int);
    }

    #[test]
    fn test_all_bool_1() {
        let src = "let x = false in if x then true else false";
        println!("Code: {} \n", src);
        let tokenlist: Vec<Token> = super::lex(&src).unwrap();
        println!("After Lex: {:?} \n", tokenlist);
        let ast = super::parse(tokenlist).unwrap().0;
        println!("After Parse: {:?} \n", ast);
        let mut map: HashMap<String, ty> = HashMap::new();
        let typed = super::type_check(&mut map, ast.clone()).unwrap();
        println!("After Typecheck: {:?}", typed);
        let mut map: HashMap<String, Box<value>> = HashMap::new();
        let evaluated = super::evaluate(&mut map, ast);
        println!("After Evaluation: {:?}", evaluated);

        assert_eq!(typed, ty::Bool);
    }

    #[test]
    fn test_stack() {
        let src = "let rec loop (x:int) : bool = if x<=1 then true else loop (x-1) in loop 9999999";
        let _ = run_code(src).unwrap();
    }
}
