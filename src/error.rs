// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use thiserror::Error;

// std result alias
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    LexerError(#[from] LexerError),
    #[error("{0}")]
    ParserError(#[from] ParserError),
    #[error("{0}")]
    TypeCheckError(#[from] TypeCheckError),
    #[error("{0}")]
    EvaluatorError(#[from] EvaluatorError),
    #[error("{0}")]
    UtilsError(#[from] UtilsError),
}

#[derive(Debug, Error)]
pub enum LexerError {
    #[error(
        "Lexer Error: '<' is forbidden in Identifiers, Keywords and Variables (Syntax Error). \n 
    Additional information: No LT (<=) supported yet"
    )]
    ForbiddenCharLEQ,
    #[error("Lexer Error: no valid Character found")]
    ForbiddenChar,
    #[error("Lexer Error: Comment started but does not end")]
    CommentError,
    #[error("Lexer Error: Identifiers are not allowed to start with a number")]
    IdentifierError,
    #[error("Lexer Error: unexpected EOF")]
    EOFError,
    #[error("Lexer Error (take_while): No Matches for Identifier")]
    NoMatches,
}

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Parser Error: Type Error")]
    TypeError,
    #[error("Parser Error: pexp parse error \n {0}")]
    PexpError(String),
    #[error("Verify failed: No token")]
    NoToken,
    #[error("Verify failed: wrong token")]
    WrongToken,
}

#[derive(Debug, Error)]
pub enum TypeCheckError {}

#[derive(Debug, Error)]
pub enum EvaluatorError {}

#[derive(Debug, Error)]
pub enum UtilsError {
    #[error("EOF, out of bounds")]
    OutOfBounds,
}
