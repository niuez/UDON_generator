use nom::branch::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::sequence::*;
use nom::IResult;

use crate::parser::{
    literal::*,
    identifier::*,
    subseq::*,
    expression::*,
    space::*,
};

#[derive(Debug)]
pub enum UnaryExpr {
    Literal(Literal),
    Identifier(Identifier),
    Paren(Box<Expression>),
    Subseq(Box<UnaryExpr>, Subseq),
}

impl UnaryExpr {
    pub fn parse(s: &str) -> IResult<&str, UnaryExpr> {
        let (mut s, mut unary) = alt((
                map(Literal::parse, |l| UnaryExpr::Literal(l)),
                map(Identifier::parse, |i| UnaryExpr::Identifier(i)),
                map(tuple((char('('), pyspace0, Expression::parse, pyspace0, char(')'))), |(_, _, e, _, _)| UnaryExpr::Paren(Box::new(e))),
        ))(s)?;
        while let Ok((ss, (_, subseq))) = tuple((pyspace0, Subseq::parse))(s) {
            s = ss;
            unary = Self::Subseq(Box::new(unary), subseq);
        }
        Ok((s, unary))
    }
    pub fn transpile(self) -> String {
        match self {
            Self::Literal(l) => l.transpile(),
            Self::Identifier(i) => i.transpile(),
            Self::Paren(e) => format!("({})", (*e).transpile()),
            Self::Subseq(u, s) => format!("{}{}", (*u).transpile(), s.transpile()),
        }
    }
}

#[test]
pub fn unary_expr_test() {
    println!("{:?}", UnaryExpr::parse("func(1, 2, 3)").unwrap());
    println!("{:?}", UnaryExpr::parse("input().split()").unwrap());
    println!("{:?}", UnaryExpr::parse("plt.imshow(cmap=\"gray\")").unwrap());
    println!("{:?}", UnaryExpr::parse("plt.imshow(img, cmap=\"gray\")").unwrap());
    println!("{:?}", UnaryExpr::parse("plt.imshow(img, cmap=\"gray\", **settings)").unwrap());

    println!("{:?}", UnaryExpr::parse("v[1][2]").unwrap());
    println!("{:?}", UnaryExpr::parse("self.v[1][2]").unwrap());
}
