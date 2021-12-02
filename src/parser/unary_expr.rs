use nom::branch::*;
use nom::combinator::*;
use nom::IResult;

use crate::parser::{
    literal::*,
    identifier::*,
    subseq::*,
};

#[derive(Debug)]
pub enum UnaryExpr {
    Literal(Literal),
    Identifier(Identifier),
    Subseq(Box<UnaryExpr>, Subseq),
}

impl UnaryExpr {
    pub fn parse(s: &str) -> IResult<&str, UnaryExpr> {
        let (mut s, mut unary) = alt((
                map(Literal::parse, |l| UnaryExpr::Literal(l)),
                map(Identifier::parse, |i| UnaryExpr::Identifier(i))
        ))(s)?;
        while let Ok((ss, subseq)) = Subseq::parse(s) {
            s = ss;
            unary = Self::Subseq(Box::new(unary), subseq);
        }
        Ok((s, unary))
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
