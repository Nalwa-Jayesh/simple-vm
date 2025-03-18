use std::convert::Infallible;

use crate::{combinator::{repeat0, Any}, Parser};

struct Token {
    content: String,
    line_number: usize,
    position_in_line: usize,
}

type PResult<T, E> = Result<(&str, T), E>;

enum TokenizeError{
    Generic(String)
}

fn skip_whitespace<'a, T, E, F>(f: F) -> impl Fn(&str) -> PResult<T, String>
where
    F: Parser<&str, T, E> 
{
    move |input| {
        let (s0, _) = input
            .run(discard(repeat0(whitespace)));
        f.run(s0)
    }
}
fn lit_token<'a>(t: &'a str) -> impl Fn(&str) -> PResult<Token, ()> {
    move |state| state.run(crate::character::token(t))
}

fn next_token(input: &str) -> Result<(&str, Token),  Infallible> {
    let p = Any::new(vec![
        repeat1(not_char("{}[]()+-*%<>=:")),
        lit_token(":="),
        lit_token("=="),
        lit_token(">="),
        lit_token("<="),
        ]);
    skip_whitespace()
}
