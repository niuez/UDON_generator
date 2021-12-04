use nom::character::complete::*;
use nom::combinator::*;
use nom::sequence::*;
use nom::bytes::complete::*;
use nom::multi::*;
use nom::IResult;

use crate::parser::{
    identifier::*,
    space::*,
    expression::*,
    suite::*,
};

#[derive(Debug)]
pub struct FuncDefinition {
    funcname: Identifier,
    args: Vec<FuncArg>,
    suite: ReturnSuite,
}

impl FuncDefinition {
    pub fn parse<'a>(indent: usize) -> impl FnMut(&'a str) -> IResult<&'a str, Self> {
        move |s| {
            let (s, (_, _, funcname, _, _, args, _, _, _, _, _, _, suite)) = tuple((
                tag("def"), pyspace1, Identifier::parse, pyspace0, char('('),
                separated_list0(tuple((pyspace0, char(','))), map(tuple((pyspace0, FuncArg::parse)), |(_, arg)| arg)),
                pyspace0, char(')'), pyspace0, char(':'), pyspace0, pynewline,
                ReturnSuite::parse(indent + 1),
            ))(s)?;
            Ok((s, Self { funcname, args, suite }))
        }
    }
    pub fn transpile(self) -> String {
        let args = self.args.into_iter().map(|a| a.transpile()).collect::<Vec<_>>().join(", ");
        format!("{} := lambda {} : {}", self.funcname.transpile(), args, self.suite.transpile())
    }
}

#[derive(Debug)]
pub struct FuncArg {
    id: Identifier,
    annotation: Option<Expression>,
    default_param: Option<Expression>,
}

impl FuncArg {
    pub fn parse(s: &str) -> IResult<&str, Self> {
        let (s, (id, annotation, default_param)) = tuple((
            Identifier::parse,
            opt(map(tuple((pyspace0, char(':'), pyspace0, Expression::parse)), |(_, _, _, e)| e)),
            opt(map(tuple((pyspace0, char('='), pyspace0, Expression::parse)), |(_, _, _, e)| e)),
        ))(s)?;
        Ok((s, Self { id, annotation, default_param } ))
    }

    pub fn transpile(self) -> String {
        let default_param = self.default_param.map(|a| format!("={}", a.transpile())).unwrap_or(String::new());
        format!("{}{}", self.id.transpile(), default_param)
    }
}

#[test]
fn parse_funcdef_test() {
    println!("{:?}", all_consuming(FuncDefinition::parse(0))("def unite(self, x, y):\n  x = self.find(x)\n  y = self.find(y)\n  self.par[y] = x\n").unwrap().1.transpile());
    println!("{:?}", all_consuming(FuncDefinition::parse(0))("def add(x: int, y: int = 0):\n  return x + y\n").unwrap().1.transpile());
}
