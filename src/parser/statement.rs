use nom::branch::*;
use nom::combinator::*;
use nom::sequence::*;
use nom::IResult;

use crate::parser::{
    simplestatement::*,
    if_stmt::*,
    for_stmt::*,
    space::*, };

#[derive(Debug)]
pub enum Statement {
    Simple(SimpleStatement),
    IfStmt(IfStatement),
    ForStmt(ForStatement),
}

impl Statement {
    pub fn parse<'a>(indent: usize) -> impl FnMut(&'a str) -> IResult<&'a str, Self> {
        move |s| {
            alt((
                map(tuple((SimpleStatement::parse, pynewline)), |(s, _)| Self::Simple(s)),
                map(IfStatement::parse(indent), |s| Self::IfStmt(s)),
                map(ForStatement::parse(indent), |s| Self::ForStmt(s)),
            ))(s)
        }
    }
    pub fn transpile(self) -> String {
        match self {
            Self::Simple(s) => s.transpile(),
            Self::IfStmt(i) => i.transpile(),
            Self::ForStmt(f) => f.transpile(),
        }
    }
}

#[test] 
fn parse_stmt_test() {
    println!("{:?}", Statement::parse(0)("for i in range(5):\n  print(i)\n").unwrap())
}
