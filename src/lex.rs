// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::error::{LexerError, Result};
use crate::utils::getsubchar;
use std::fmt::Debug;
use std::iter::{Iterator, Peekable};
use std::str;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Constant {
    BCON(bool),
    ICON(isize),
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    LP,
    RP,
    EQ,
    COL,
    ARR,
    ADD,
    SUB,
    MUL,
    LEQ,
    IF,
    THEN,
    ELSE,
    LAM,
    LET,
    IN,
    REC,
    CON(Constant),
    VAR(String),
}

pub fn lex(code: &str) -> Result<Vec<Token>> {
    let mut tokenlist: Vec<Token> = Vec::new();
    let mut iterator = code.chars().peekable();

    loop {
        if let Some(_) = iterator.peek() {
            lexer(&mut tokenlist, &mut iterator)?;
            continue;
        } else {
            break;
        }
    }

    Ok(tokenlist)
}

fn lexer<I>(tokenlist: &mut Vec<Token>, iterator: &mut Peekable<I>) -> Result<()>
where
    I: Iterator<Item = char> + Debug + Clone,
{
    while iterator.peek().unwrap().is_whitespace() {
        drop(iterator.next())
    }

    if let Ok(e) = getsubchar(iterator, 2) {
        match e.as_str() {
            "(*" => {
                drop(iterator.nth(2));

                while getsubchar(iterator, 2)
                    .map_err(|_| LexerError::CommentError)?
                    .as_str()
                    != "*)"
                {
                    iterator.next();
                }
                if iterator.nth(2).is_none() {
                    return Ok(());
                }
            }
            _ => (),
        }
    }

    let token = match iterator.peek().unwrap() {
        '+' => {
            iterator.next();
            Token::ADD
        }
        '*' => {
            iterator.next();
            Token::MUL
        }
        '=' => {
            iterator.next();
            Token::EQ
        }
        '(' => {
            iterator.next();
            Token::LP
        }
        ')' => {
            iterator.next();
            Token::RP
        }
        '<' => {
            iterator.next();
            if iterator.peek() == Some(&'=') {
                iterator.next();
                Token::LEQ
            } else {
                return Err(LexerError::ForbiddenCharLEQ)?;
            }
        }
        '-' => {
            iterator.next();
            if iterator.peek() == Some(&'>') {
                iterator.next();
                Token::ARR
            } else {
                Token::SUB
            }
        }
        ':' => {
            iterator.next();
            Token::COL
        }
        '0'..='9' => tokenize_number(iterator)?,
        c if c.is_lowercase() => tokenize_identifiers(iterator)?,
        _ => Err(LexerError::ForbiddenChar)?,
    };

    tokenlist.push(token);
    Ok(())
}

fn tokenize_number<I>(iterator: &mut Peekable<I>) -> Result<Token>
where
    I: Iterator<Item = char> + Debug,
{
    /* this should never happen
    if let Some(n) = iterator.peek() {
        if !n.is_numeric() {
            unreachable!("tokenize_number: Iterator does not begin with a Number");
        }
    }*/

    let mut integer: isize = 0;

    while if let Some(n) = iterator.peek() {
        n.is_numeric()
    } else {
        false
    } {
        integer = integer * 10 + iterator.next().unwrap().to_digit(10).unwrap() as isize;
    }
    Ok(Token::CON(Constant::ICON(integer)))
}

fn tokenize_identifiers<I>(iterator: &mut Peekable<I>) -> Result<Token>
where
    I: Iterator<Item = char> + Debug + Clone,
{
    match iterator.peek() {
        Some(ch) if ch.is_digit(10) => Err(LexerError::IdentifierError)?,
        None => Err(LexerError::EOFError)?,
        _ => {}
    }

    let code: String = iterator.clone().collect::<String>();

    let (got, index) = take_while(&code, |ch| ch == '_' || ch.is_alphanumeric())?;

    drop(iterator.nth(index - 1));

    let token: Token = match got {
        "if" => Token::IF,
        "then" => Token::THEN,
        "else" => Token::ELSE,
        "fun" => Token::LAM,
        "let" => Token::LET,
        "in" => Token::IN,
        "rec" => Token::REC,
        "false" => Token::CON(Constant::BCON(false)),
        "true" => Token::CON(Constant::BCON(true)),
        _ => Token::VAR(got.to_string()),
    };

    Ok(token)
}

fn take_while<F>(data: &str, mut pred: F) -> Result<(&str, usize)>
where
    F: FnMut(char) -> bool,
{
    let mut current_index = 0;

    for ch in data.chars() {
        let should_continue = pred(ch);

        if !should_continue {
            break;
        }

        current_index += ch.len_utf8();
    }

    if current_index == 0 {
        Err(LexerError::NoMatches)?
    } else {
        Ok((&data[..current_index], current_index))
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use super::*;
    use crate::lex::Token::*;
    use Constant::{BCON, ICON};

    fn check_ok<S>(panicmsg: S, code: &str) -> Vec<Token>
    where
        S: AsRef<str> + Display,
    {
        let r = if let Ok(e) = lex(&code) {
            e
        } else {
            panic!("{}", panicmsg)
        };

        r
    }

    #[test]
    fn test_number_only() {
        let mut code = "123456789";
        let mut r = check_ok("lexer returned Err instead of Ok for \"123456789\"", &code);
        assert_eq!(r, vec![CON(ICON(123456789))]);

        code = "00011122223333444555";
        r = check_ok(
            "lexer returned Err instead of Ok for \"00011122223333444555\"",
            &code,
        );
        assert_eq!(r, vec![CON(ICON(11122223333444555))]);

        code = "01010101010";
        r = check_ok(
            "lexer returned Err instead of Ok for \"01010101010\"",
            &code,
        );
        assert_eq!(r, vec![CON(ICON(1010101010))]);

        code = "0";
        r = check_ok(
            "lexer returned Err instead of Ok for \"01010101010\"",
            &code,
        );
        assert_eq!(r, vec![CON(ICON(0))]);
    }

    #[test]
    fn test_identifiers_only() {
        let mut code = "if true then false else true";
        let mut r = check_ok(
            "lexer returned Err instead of Ok for \"let rec test n = test n\"",
            &code,
        );
        assert_eq!(
            r,
            vec![
                IF,
                CON(BCON(true)),
                THEN,
                CON(BCON(false)),
                ELSE,
                CON(BCON(true))
            ]
        );

        code = "x let rec false in true fun else if xyz then abcdefg";
        r = check_ok("lexer returned Err instead of Ok for \"x let rec false in true fun else if xyz then abcdefg\"",&code);
        assert_eq!(
            r,
            vec![
                VAR("x".to_string()),
                LET,
                REC,
                CON(BCON(false)),
                IN,
                CON(BCON(true)),
                LAM,
                ELSE,
                IF,
                VAR("xyz".to_string()),
                THEN,
                VAR("abcdefg".to_string())
            ]
        );
    }

    #[test]
    fn test_keywords_only() {
        let code = "+*=()<=-:";
        let r = check_ok("lexer returned Err instead of Ok for \"+*=()<-:\"", &code);
        assert_eq!(r, vec![ADD, MUL, EQ, LP, RP, LEQ, SUB, COL]);
    }

    #[test]
    fn mixed_test() {
        let code = "let rec test n = test n";
        let r = check_ok(
            "lexer returned Err instead of Ok for Test, id+kw: \"let rec test n = test n\"",
            &code,
        );
        assert_eq!(
            r,
            vec![
                LET,
                REC,
                VAR("test".to_string()),
                VAR("n".to_string()),
                EQ,
                VAR("test".to_string()),
                VAR("n".to_string())
            ]
        );
    }

    #[test]
    #[should_panic]
    fn invalid_keywords() {
        let code = "<";
        let _r = check_ok(
            "lexer returned Err and thats how it should be, kw: \"<let rec test n = test n>\"",
            &code,
        );
    }
}
