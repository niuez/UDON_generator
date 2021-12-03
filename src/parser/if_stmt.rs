use nom::character::complete::*;
use nom::combinator::*;
use nom::sequence::*;
use nom::bytes::complete::*;
use nom::multi::*;
use nom::IResult;

use crate::parser::{
    expression::*,
    suite::*,
    space::*,
};

#[derive(Debug)]
pub struct IfStatement {
    if_cond: Box<Expression>,
    if_suite: Suite,
    elseifs: Vec<(Expression, Suite)>,
    else_suite: Option<Suite>,
}

impl IfStatement {
    pub fn parse<'a>(indent: usize) -> impl FnMut(&'a str) -> IResult<&'a str, Self> {
        move |s| {
            let (s, (_, _, if_cond, _, _, _, _, if_suite, elseifs, else_suite)) = tuple((
                tag("if"), pyspace1, Expression::parse, pyspace0, char(':'), pyspace0, pynewline, Suite::parse(indent + 1),
                many0(map(
                    tuple((pyindent(indent), tag("elif"), pyspace1, Expression::parse, pyspace0, char(':'), pyspace0, pynewline, Suite::parse(indent + 1))),
                    |(_, _, _, e, _, _, _, _, s)| (e, s)
                )),
                opt(map(
                    tuple((pyindent(indent), tag("else"), pyspace0, char(':'), pyspace0, pynewline, Suite::parse(indent + 1))),
                    |(_, _, _, _, _, _, s)| s
                )),
            ))(s)?;
            Ok((s, Self { if_cond: Box::new(if_cond), if_suite, elseifs, else_suite, }))
        }
    }

    pub fn transpile(self) -> String {
        let ifs = format!("{} if {}", self.if_suite.transpile(), self.if_cond.transpile());
        let elifs = self.elseifs.into_iter().map(|(e, s)| format!("else {} if {}", s.transpile(), e.transpile()));
        let elses = self.else_suite.map(|s| format!("else {}", s.transpile())).unwrap_or(format!("else []"));
        std::iter::once(ifs).chain(elifs).chain(std::iter::once(elses)).collect::<Vec<_>>().join(" ")
    }
}

#[test]
fn parse_ifstmt_test() {
    println!("{:?}", IfStatement::parse(0)("if a == b:\n  print(\"equal\")\nelif a + 1 == b:\n  print(\"near\")\nelse:\n  print(0)\n").unwrap())
}
