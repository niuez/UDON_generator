use nom::branch::*;
use nom::combinator::*;
use nom::IResult;

use crate::parser::{
    call::*,
};

#[derive(Debug)]
pub enum Subseq {
    Call(Call),
}

impl Subseq {
    pub fn parse(s: &str) -> IResult<&str, Self> {
        alt((map(Call::parse, |c| Self::Call(c)), ))(s)
    }
}
