use nom::branch::*;
use nom::combinator::*;
use nom::sequence::*;
use nom::IResult;

use crate::parser::{
    simplestatement::*,
    if_stmt::*,
    for_stmt::*,
    space::*,
    func_def::*,
};

#[derive(Debug)]
pub enum Statement {
    Simple(SimpleStatement),
    IfStmt(IfStatement),
    ForStmt(ForStatement),
    FuncDef(FuncDefinition),
}

impl Statement {
    pub fn parse<'a>(indent: usize) -> impl FnMut(&'a str) -> IResult<&'a str, Self> {
        move |s| {
            alt((
                map(tuple((SimpleStatement::parse, pynewline)), |(s, _)| Self::Simple(s)),
                map(IfStatement::parse(indent), |s| Self::IfStmt(s)),
                map(ForStatement::parse(indent), |s| Self::ForStmt(s)),
                map(FuncDefinition::parse(indent), |s| Self::FuncDef(s)),
            ))(s)
        }
    }
    pub fn transpile(self) -> String {
        match self {
            Self::Simple(s) => s.transpile(),
            Self::IfStmt(i) => i.transpile(),
            Self::ForStmt(f) => f.transpile(),
            Self::FuncDef(f) => f.transpile(),
        }
    }
}

#[test] 
fn parse_stmt_test() {
    println!("{:?}", Statement::parse(0)("for i in range(5):\n  print(i)\n").unwrap())
}
