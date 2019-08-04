use logtea::fill::{FillLogArg, FillLogTea};
use rettle::pot::Pot;
use rettle::ingredient::Pour;
use rettle::tea::Tea;
use rettle::brewer::Brewery;

use nom::{
    IResult,
    sequence::{delimited},
    character::complete::{char, space1, not_line_ending},
    bytes::complete::{tag, is_not, take},
};

use std::time::Instant;
use std::any::Any;

#[derive(Default, Clone, Debug)]
struct LogTea {
    log_type: &'static str,
    datetime: &'static str,
    msg: &'static str,
}

impl Tea for LogTea {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn new(self: Box<Self>) -> Box<dyn Tea + Send> {
        self
    }
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

fn parse_log(input: &'static str) -> IResult<&str, LogTea> {
    let (input, log_type) = log_type(input)?;
    let (input, _) = tag(" - ")(input)?;
    let (input, datetime) = datetime(input)?;
    let (input, _) = space1(input)?;
    let (input, msg) = msg(input)?;
    Ok((input, LogTea { log_type, datetime, msg }))
}

fn main() {
    let test_fill_logarg = FillLogArg::new("fixtures/log.LOG", 50, parse_log);

    let brewery = Brewery::new(4, Instant::now());
    let mut new_pot = Pot::new();
    let fill_cstea = FillLogTea::new::<LogTea>("log_tea_source", "log_fixture", test_fill_logarg);

    new_pot.add_source(fill_cstea);
}
