// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::error::{Result, UtilsError};
use std::iter::{Iterator, Peekable};

fn _getsubchar_generic<I, T>(iterator: &mut I, n: usize) -> Result<Vec<T>>
where
    I: Iterator<Item = T>,
{
    let mut s: Vec<T> = Vec::new();

    for _ in 0..n {
        s.push(if let Some(n) = iterator.next() {
            n
        } else {
            Err(UtilsError::OutOfBounds)?
        });
    }
    return Ok(s);
}

pub fn getsubchar<I>(iterator: &mut Peekable<I>, n: usize) -> Result<String>
where
    I: Iterator<Item = char> + Clone,
{
    let mut s = String::new();
    let mut citerator = iterator.clone();

    for _ in 0..n {
        s.push(if let Some(n) = citerator.next() {
            n.clone()
        } else {
            Err(UtilsError::OutOfBounds)?
        });
    }
    return Ok(s);
}
