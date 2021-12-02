use nom::IResult;

#[derive(Debug)]
pub enum Expression {}

impl Expression {
    pub fn parse(s: &str) -> IResult<&str, Expression> {
        unimplemented!()
    }
}
