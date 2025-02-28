# Sub-OCaml

[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?style=flat-square)](https://www.rust-lang.org/)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg?style=flat-square)](https://github.com/Neotamandua/Sub-OCaml/blob/master/LICENSE)
![License: MPL 2.0](https://img.shields.io/github/languages/code-size/Neotamandua/Sub-OCaml?style=flat-square)
![Github CI](https://img.shields.io/github/actions/workflow/status/Neotamandua/Sub-OCaml/build.yml?style=flat-square)
> Interpreter for a subset of the OCaml language. \
> This project did not intend to use idiomatic rust code. I mainly explored the syntactical possibilites of matches, recursion etc. for rust.

## Usage
You can use the [REPL](https://github.com/Neotamandua/Sub-OCaml-REPL/) to directly execute code and try it out

### Dependencies:

```toml
[dependencies]
thiserror = "2.0"
```

### Features:

| Features |   Status      |
| -------- | --------------- |
| Lexer  | ✅|
| Parser    | ✅ |
| Typechecker   | ✅            |
| Evaluator   | ✅         |


### Examples:
**1**
```ocaml
let x = 5 in x
```
Lex result:
> [LET, VAR("x"), EQ, CON(ICON(5)), IN, VAR("x")]

Parse result:
> Let("x", Con(Icon(5)), Var("x"))

Typechecker result:
> Int

Evaluator result:
> IntValue(5)
___

**2**
```ocaml
let rec fib (a:int) : int -> int -> int = fun (b:int) -> fun (n:int) -> if n <= 0 then a else fib (b) (a+b) (n-1) in fib 0 1 8
```
Lex result:
> [LET, REC, VAR("fib"), LP, VAR("a"), COL, VAR("int"), RP, COL, VAR("int"), ARR, VAR("int"), ARR, VAR("int"), EQ, LAM, LP, VAR("b"), COL, VAR("int"), RP, ARR, LAM, LP, VAR("n"), COL, VAR("int"), RP, ARR, IF, VAR("n"), LEQ, CON(ICON(0)), THEN, VAR("a"), ELSE, VAR("fib"), LP, VAR("b"), RP, LP, VAR("a"), ADD, VAR("b"), RP, LP, VAR("n"), SUB, CON(ICON(1)), RP, IN, VAR("fib"), CON(ICON(0)), CON(ICON(1)), CON(ICON(8))]

Parse result:
> Letrecty("fib", "a", Int, Arrow(Int, Arrow(Int, Int)), Lamty("b", Int, Lamty("n", Int, If(Oapp(Leq, Var("n"), Con(Icon(0))), Var("a"), Fapp(Fapp(Fapp(Var("fib"), Var("b")), Oapp(Add, Var("a"), Var("b"))), Oapp(Sub, Var("n"), Con(Icon(1))))))), Fapp(Fapp(Fapp(Var("fib"), Con(Icon(0))), Con(Icon(1))), Con(Icon(8))))

Typechecker result:
> Int

Evaluator result:
> IntValue(21)

Example usage code:
```rust
use Sub_OCaml::{lexer, parse, ty, type_check};

fn main() {
    let src = "let x = 5 in x";
    println!("Code: {} \n", src);
    let tokenlist: Vec<Token> = lexer(&src).unwrap();
    println!("After Lex: {:?} \n", tokenlist);
    let ast = parse(tokenlist).unwrap().0;
    println!("After Parse: {:?} \n", ast);
    let mut map: BTreeMap<String, ty> = BTreeMap::new();
    let typed = super::type_check(&mut map, ast).unwrap();
    println!("After Typecheck: {:?}", typed);
}
```
