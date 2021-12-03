use nom::branch::*;
use nom::combinator::*;
use nom::sequence::*;
use nom::IResult;

use crate::parser::{
    simplestatement::*,
    if_stmt::*,
    space::*,
};

#[derive(Debug)]
pub enum Statement {
    Simple(SimpleStatement),
    IfStmt(IfStatement),
}

impl Statement {
    pub fn parse<'a>(indent: usize) -> impl FnMut(&'a str) -> IResult<&'a str, Self> {
        move |s| {
            alt((
                map(tuple((SimpleStatement::parse, pynewline)), |(s, _)| Self::Simple(s)),
                map(tuple((IfStatement::parse(indent), pynewline)), |(s, _)| Self::IfStmt(s)),
            ))(s)
        }
    }
    pub fn transpile(self) -> String {
        match self {
            Self::Simple(s) => s.transpile(),
            Self::IfStmt(i) => i.transpile(),
        }
    }
}
