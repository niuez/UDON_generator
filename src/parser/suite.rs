use nom::combinator::*;
use nom::sequence::*;
use nom::multi::*;
use nom::IResult;

use crate::parser::{
    statement::*,
    space::*,
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

#[test]
fn parse_suite_test() {
    println!("{:?}", Suite::parse(1)("  a = 3\n  b = 4\n").unwrap());
}
