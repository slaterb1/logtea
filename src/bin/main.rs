use rettle::pot::Pot;
use rettle::ingredient::{Fill, Pour};
use rettle::brewer::Brewery;

use nom::{
    named,
    IResult,
    sequence::delimited,
    character::complete::char,
    bytes::complete::is_not,
};

fn log_type(input: &str) -> IResult<&str, &str> {
    delimited(char('['), is_not("]"), char(']'))(input)
}

fn main() {
    let test = "[INFO] - 2019-07-26T00:00:00 Server listening on port 8000";
    let res = log_type(test);
    println!("{:?}", res);
}
