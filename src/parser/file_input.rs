use nom::branch::*;
use nom::combinator::*;
use nom::multi::*;
use nom::IResult;

use crate::parser::{
    statement::*,
    space::*,
};

#[derive(Debug)]
pub struct FileInput {
    stmts: Vec<Statement>,
}

impl FileInput {
    pub fn parse(s: &str) -> IResult<&str, Self> {
        let (s, stmts) = 
            many0(alt((
                map(Statement::parse(0), |s| Some(s)),
                map(pynewline, |_| None),
            )))
        (s)?;
        let stmts = stmts.into_iter().filter_map(|s| s).collect::<Vec<_>>();
        Ok((s, Self { stmts }))
    }
    pub fn transpile(self) -> String {
        let stmts = self.stmts.into_iter().map(|s| s.transpile()).collect::<Vec<_>>().join(", ");
        format!("[{}]", stmts)
    }
}
