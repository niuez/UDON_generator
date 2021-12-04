use nom::bytes::complete::*;
use nom::combinator::*;
use nom::sequence::*;
use nom::multi::*;
use nom::IResult;

use crate::parser::{
    statement::*,
    space::*,
    expression::*,
};

#[derive(Debug)]
pub struct Suite {
    stmts: Vec<Statement>,
}

impl Suite {
    pub fn parse<'a>(indent: usize) -> impl FnMut(&'a str) -> IResult<&str, Self> {
        move |s| {
            let (s, stmts) = many0(map(tuple((pyindent(indent), Statement::parse(indent))), |(_, s)| s))(s)?;
            Ok((s, Suite { stmts }))
        }
    }
    pub fn transpile(self) -> String {
        let stmts = self.stmts.into_iter().map(|s| s.transpile()).collect::<Vec<_>>().join(", ");
        format!("[{}]", stmts)
    }
}


#[derive(Debug)]
pub struct ReturnSuite {
    stmts: Vec<Statement>,
    return_expr: Option<Expression>,
}

impl ReturnSuite {
    pub fn parse<'a>(indent: usize) -> impl FnMut(&'a str) -> IResult<&str, Self> {
        move |s| {
            let (s, (stmts, return_expr)) = tuple((
                many0(map(tuple((pyindent(indent), Statement::parse(indent))), |(_, s)| s)),
                opt(map(tuple((pyindent(indent), tag("return"), pyspace1, Expression::parse, pyspace0, pynewline)), |(_, _, _, e, _, _)| e)),
            ))(s)?;
            Ok((s, ReturnSuite { stmts, return_expr }))
        }
    }
    pub fn transpile(self) -> String {
        let stmts_cnt = self.stmts.len();
        let stmts = self.stmts.into_iter().map(|s| s.transpile())
            .chain(std::iter::once(self.return_expr.map(|e| e.transpile()).unwrap_or(format!("None"))))
            .collect::<Vec<_>>().join(", ");
        format!("[{}][{}]", stmts, stmts_cnt)
    }
}

#[test]
fn parse_suite_test() {
    println!("{:?}", Suite::parse(1)("  a = 3\n  b = 4\n").unwrap());
}
