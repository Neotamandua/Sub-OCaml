// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod lex;
mod parse;
mod typechecker;
//mod eval;
mod error;
mod utils;

pub use lex::lexer;
pub use parse::parse;
pub use typechecker::type_check;

#[cfg(test)]
mod tests {
    use crate::lex::Token;
    use crate::parse::ty;
    use std::collections::HashMap;

    #[test]
    fn test_all_int_1() {
        let src = "let x = 5 in x";
        println!("Code: {} \n", src);
        let tokenlist: Vec<Token> = super::lexer(&src).unwrap();
        println!("After Lex: {:?} \n", tokenlist);
        let ast = super::parse(tokenlist).unwrap().0;
        println!("After Parse: {:?} \n", ast);
        let mut map: HashMap<String, ty> = HashMap::new();
        let typed = super::type_check(&mut map, ast);
        println!("After Typecheck: {:?}", typed);

        assert_eq!(typed, ty::Int);
    }

    #[test]
    fn test_all_int_2() {
        let src = "let rec fib (a:int) : int -> int -> int = fun (b:int) -> fun (n:int) -> if n <= 0 then a else fib (b) (a+b) (n-1) in fib 0 1 8";
        println!("Code: {} \n", src);
        let tokenlist: Vec<Token> = super::lexer(&src).unwrap();
        println!("After Lex: {:?} \n", tokenlist);
        let ast = super::parse(tokenlist).unwrap().0;
        println!("After Parse: {:?} \n", ast);
        let mut map: HashMap<String, ty> = HashMap::new();
        let typed = super::type_check(&mut map, ast);
        println!("After Typecheck: {:?}", typed);

        assert_eq!(typed, ty::Int);
    }

    #[test]
    fn test_all_bool_1() {
        let src = "let x = false in if x then true else false";
        println!("Code: {} \n", src);
        let tokenlist: Vec<Token> = super::lexer(&src).unwrap();
        println!("After Lex: {:?} \n", tokenlist);
        let ast = super::parse(tokenlist).unwrap().0;
        println!("After Parse: {:?} \n", ast);
        let mut map: HashMap<String, ty> = HashMap::new();
        let typed = super::type_check(&mut map, ast);
        println!("After Typecheck: {:?}", typed);

        assert_eq!(typed, ty::Bool);
    }
}
