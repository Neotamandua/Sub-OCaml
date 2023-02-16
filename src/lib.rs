// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod lex;
mod parse;
//mod typechecker;
//mod eval;
mod error;
mod utils;

pub use lex::lexer;
pub use parse::parse;
//pub use typechecker::type_check;

#[cfg(test)]
mod tests {
    #[test]
    fn test_all() {
        use crate::lex::Token;
        //use crate::parse::ty;
        //use std::collections::HashMap;

        let src = "let x = 5 in x";

        println!("Code: {} ", src);
        println!("-------------------------------");

        let mut x: Vec<Token> = vec![];

        if let Ok(e) = super::lexer(&src) {
            x = e;
            println!("After Lex: {:?}", x);
        };

        println!("-------------------------------");

        let y = super::parse(x);
        println!("After Parse: {:?}", y);

        /*println!("-------------------------------");
        let mut map: HashMap<String, ty> = HashMap::new();
        let z = super::type_check(&mut map, y.0);
        println!("After Typecheck: {:?}", z);*/
    }
}
