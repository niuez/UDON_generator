use nom::branch::*;
use nom::combinator::*;
use nom::sequence::*;
use nom::IResult;

use crate::parser::{
    simplestatement::*,
    space::*,
};

#[derive(Debug)]
pub enum Statement {
    Simple(SimpleStatement),
}

impl Statement {
    pub fn parse(s: &str) -> IResult<&str, Self> {
        alt((
            map(tuple((SimpleStatement::parse, pynewline)), |(s, _)| Self::Simple(s)),
        ))(s)
    }
}
