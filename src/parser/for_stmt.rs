use nom::character::complete::*;
use nom::combinator::*;
use nom::sequence::*;
use nom::bytes::complete::*;
use nom::multi::*;
use nom::IResult;

use crate::parser::{
    expression::*,
    simplestatement::TargetList,
    suite::*,
    space::*,
};

#[derive(Debug)]
pub struct ForStatement {
    target: TargetList,
    iter: Vec<Expression>,
    suite: Suite,
}

impl ForStatement {
    pub fn parse<'a>(indent: usize) -> impl FnMut(&'a str) -> IResult<&'a str, Self> {
        move |s| {
            let (s, (_, _, target, _, _, _, iter, _, _, _, _, suite)) = tuple((
                tag("for"), pyspace1, TargetList::parse, pyspace1, tag("in"), pyspace1,
                separated_list1(
                    tuple((pyspace0, char(','))),
                    map(tuple((pyspace0, Expression::parse)), |(_, e)| e)
                ),
                pyspace0, char(':'), pyspace0, pynewline,
                Suite::parse(indent + 1),
            ))(s)?;
            Ok((s, Self { target, iter, suite }))
        }
    }

    pub fn transpile(self) -> String {
        let iter = self.iter.into_iter().map(|e| e.transpile()).collect::<Vec<_>>().join(", ");
        format!("[{} for {} in {}]", self.suite.transpile(), self.target.transpile_as_var(), iter)
    }
}

#[test]
fn parse_forstmt_test() {
    println!("{:?}", ForStatement::parse(0)("for i in range(5):\n  print(i)\n").unwrap())
}
