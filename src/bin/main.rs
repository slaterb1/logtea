use rettle::pot::Pot;
use rettle::ingredient::{Fill, Pour};
use rettle::brewer::Brewery;

use nom::{
    IResult,
    sequence::{delimited, preceded},
    character::complete::{char, space1, not_line_ending},
    bytes::complete::{tag, is_not, take},
    combinator::opt,
};

#[derive(Debug)]
struct LogTea {
    log_type: &'static str,
    datetime: &'static str,
    msg: &'static str,
    data: Option<&'static str>,
}

fn log_type(input: &'static str) -> IResult<&'static str, &'static str> {
    delimited(char('['), is_not("]"), char(']'))(input)
}

fn datetime(input: &str) -> IResult<&str, &str> {
    take(19u8)(input)
}

fn msg(input: &str) -> IResult<&str, &str> {
    not_line_ending(input)
}

fn data(input: &str) -> IResult<&str, Option<&str>> {
    opt(delimited(char('-'), is_not(" "), char(' ')))(input)
}

fn parse_log(input: &'static str) -> IResult<&str, LogTea> {
    let (input, log_type) = log_type(input)?;
    let (input, _) = tag(" - ")(input)?;
    let (input, datetime) = datetime(input)?;
    let (input, _) = space1(input)?;
    let (input, msg) = msg(input)?;
    let (input, data) = data(input)?;
    Ok((input, LogTea { log_type, datetime, msg, data }))
}

fn main() {
    let test = "[INFO] - 2019-07-26T00:00:00 Server listening on port 8000";
    let test2 = "[ERROR] - 2019-07-26T00:00:00 Error on request! {status: 400, msg: Bad Request, data: []}";
    let res = parse_log(test);
    //let res2 = datetime(test);
    println!("{:?}", res);
}
