use nom::branch::*;
use nom::combinator::*;
use nom::character::complete::*;
use nom::sequence::*;
use nom::IResult;

use crate::parser::{
    expression::*,
    identifier::*,
    subseq::*,
    index::*,
    member::*,
    space::*,
};

/* https://docs.python.org/ja/3/reference/simple_stmts.html#grammar-token-python-grammar-expression_stmt */
#[derive(Debug)]
pub enum SimpleStatement {
    Expression(Expression),
    Assignment(AssignmentStatement),
}

impl SimpleStatement {
    pub fn parse(s: &str) -> IResult<&str, Self> {
        alt((
            map(AssignmentStatement::parse, |a| Self::Assignment(a)),
            map(Expression::parse, |e| Self::Expression(e)),
        ))(s)
    }
}

#[derive(Debug)]
pub struct AssignmentStatement {
    target: TargetList,
    expr: Expression,
}

impl AssignmentStatement {
    pub fn parse(s: &str) -> IResult<&str, Self> {
        let (s, (target, _, _, _, expr)) = tuple((TargetList::parse, pyspace0, char('='), pyspace0, Expression::parse))(s)?;
        Ok((s, Self { target, expr }))
    }
}

#[derive(Debug)]
pub enum TargetList {
    Identifier(Identifier),
    Attributeref(Box<TargetList>, Member),
    Slice(Box<TargetList>, Index),
}

impl TargetList {
    pub fn parse(s: &str) -> IResult<&str, Self> {
        let (mut s, id) = Identifier::parse(s)?;
        let mut target = Self::Identifier(id);
        while let Ok((ss, (_, subseq))) = alt((
                tuple((pyspace0, map(Member::parse, |m| Subseq::Member(m)))),
                tuple((pyspace0, map(Index::parse, |m| Subseq::Index(m)))),
            ))(s) {
            let new_target = match subseq {
                Subseq::Member(m) => Self::Attributeref(Box::new(target), m),
                Subseq::Index(i) => Self::Slice(Box::new(target), i),
                _ => unreachable!(),
            };
            s = ss;
            target = new_target;
        }
        Ok((s, target))
    }
}

#[test]
fn parse_statement_test() {
    println!("{:?}", SimpleStatement::parse("hoge = 5").unwrap());
    println!("{:?}", SimpleStatement::parse("hoge.a = 5").unwrap());
    println!("{:?}", SimpleStatement::parse("hoge[1] = 5").unwrap());
    println!("{:?}", SimpleStatement::parse("hoge[1:2] = 5").unwrap());
}
