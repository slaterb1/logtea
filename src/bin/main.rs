use rettle::pot::Pot;
use rettle::ingredient::{Fill, Pour};
use rettle::brewer::Brewery;

use nom::{
    IResult,
    sequence::{delimited, preceded},
    character::complete::char,
    bytes::complete::{tag, is_not},
    combinator::opt,
};

struct LogTea {
    log_type: &'static str,
    datetime: &'static str,
    msg: &'static str,
    data: Option<&'static str>,
}

fn log_type(input: &str) -> IResult<&str, &str> {
    delimited(char('['), is_not("]"), char(']'))(input)
}

fn datetime(input: &str) -> IResult<&str, &str> {
    delimited(char('-'), is_not(" "), char(' '))(input)
}

fn msg(input: &str) -> IResult<&str, &str> {
    delimited(char('-'), is_not(" "), char(' '))(input)
}

fn data(input: &str) -> IResult<&str, &str> {
    delimited(char('-'), is_not(" "), char(' '))(input)
}

fn parse_log(input: &str) -> IResult<&str, LogTea> {
    let (input, log_type) = log_type(input)?;
    let (input, _) = tag("- ")(input)?;
    let (input, datetime) = datetime(input)?;
    let (input, msg) = msg(input)?;
    let (input, data) = opt(data(input))(input)?;
    (input, LogTea { log_type, datetime, msg, data })
}

fn main() {
    let test = "[INFO] - 2019-07-26T00:00:00 Server listening on port 8000";
    let test2 = "[ERROR] - 2019-07-26T00:00:00 Error on request! {status: 400, msg: Bad Request, data: []}";
    let res = log_type(test);
    let res2 = datetime(test);
    println!("{:?}", res2);
}
