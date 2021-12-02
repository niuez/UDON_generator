use nom::branch::*;
use nom::combinator::*;
use nom::IResult;

use crate::parser::{
    call::*,
    index::*,
};

#[derive(Debug)]
pub enum Subseq {
    Call(Call),
    Index(Index),
}

impl Subseq {
    pub fn parse(s: &str) -> IResult<&str, Self> {
        alt((
            map(Call::parse, |c| Self::Call(c)),
            map(Index::parse, |c| Self::Index(c)),
        ))(s)
            
    }
}
