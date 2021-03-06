# Sub-OCaml

> Interpreter for a subset of the OCaml language.

## Usage

### Dependencies:

```toml
[dependencies]
anyhow = "1.0.57"
```

### Features:

| Features |   Status      |
| -------- | --------------- |
| Lexer  | ✅|
| Parser    | ✅ |
| Typechecker   | not yet published            |
| Evaluator   | not yet published            |


### Examples:
**1**
```ocaml
let x = 5 in x
```
Lex result:
> [LET, VAR("x"), EQ, CON(ICON(5)), IN, VAR("x")]

Parse result:
> Let("x", Con(Icon(5)), Var("x"))

___

**2**
```ocaml
let rec fib (a:int) : int -> int -> int = fun (b:int) -> fun (n:int) -> if n <= 0 then a else fib (b) (a+b) (n-1) in fib 0 1 8
```
Lex result:
> [LET, REC, VAR("fib"), LP, VAR("a"), COL, VAR("int"), RP, COL, VAR("int"), ARR, VAR("int"), ARR, VAR("int"), EQ, LAM, LP, VAR("b"), COL, VAR("int"), RP, ARR, LAM, LP, VAR("n"), COL, VAR("int"), RP, ARR, IF, VAR("n"), LEQ, CON(ICON(0)), THEN, VAR("a"), ELSE, VAR("fib"), LP, VAR("b"), RP, LP, VAR("a"), ADD, VAR("b"), RP, LP, VAR("n"), SUB, CON(ICON(1)), RP, IN, VAR("fib"), CON(ICON(0)), CON(ICON(1)), CON(ICON(8))]

Parse result:
> Letrecty("a", "fib", Int, Arrow(Int, Arrow(Int, Int)), Lamty("b", Int, Lamty("n", Int, If(Oapp(Leq, Var("n"), Con(Icon(0))), Var("a"), Fapp(Fapp(Fapp(Var("fib"), Var("b")), Oapp(Add, Var("a"), Var("b"))), Oapp(Sub, Var("n"), Con(Icon(1))))))), Fapp(Fapp(Fapp(Var("fib"), Con(Icon(0))), Con(Icon(1))), Con(Icon(8))))

Example usage code:
```rust
use Sub_OCaml::{lexer, parse};

fn main() {
    let src = "let x = 5 in x";
    let tokenlist = lexer(&src).unwrap();
    println!("After Lex: {:?}", tokenlist);
    let ast = parse(tokenlist).0;
    println!("After Parse: {:?}", ast);
}
```
