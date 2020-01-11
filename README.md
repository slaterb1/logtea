# logtea

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Build Status](https://travis-ci.com/slaterb1/logtea.svg?branch=master)](https://travis-ci.com/slaterb1/logtea)
[![Crates.io Version](https://img.shields.io/crates/v/logtea.svg)](https://crates.io/crates/logtea)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.35.0+-lightgray.svg)](#rust-version-requirements)

This is a generic log file Fill Ingredient crate for use with `rettle` ETL. This crate uses [nom](https://docs.rs/nom/) as the parser library to allow any project to define how it wants to parse logs by supplying a custom built parser.

## Data Structures
- FillLogArg: Ingredient params for FillLogTea
- FillLogTea: Wrapper to simplifiy the creation of the Fill Ingredient to be used in the rettle Pot.

## Example
```rust
#[derive(Default, Clone, Debug)]
struct LogTea {
    log_type: String,
    datetime: String,
    msg: String,
}

impl Tea for LogTea {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Custom parser setup.
fn log_type(input: &str) -> IResult<&str, &str> {
    delimited(char('['), is_not("]"), char(']'))(input)
}

fn datetime(input: &str) -> IResult<&str, &str> {
    take(19u8)(input)
}

fn msg(input: &str) -> IResult<&str, &str> {
    not_line_ending(input)
}

fn parse_log(input: &str) -> IResult<&str, LogTea> {
    // Parse log attributes.
    let (input, log_type) = log_type(input)?;
    let (input, _) = tag(" - ")(input)?;
    let (input, datetime) = datetime(input)?;
    let (input, _) = space1(input)?;
    let (input, msg) = msg(input)?;

    // Convert &str to String
    let log_type = String::from(log_type);
    let datetime = String::from(datetime);
    let msg = String::from(msg);
    Ok((input, LogTea { log_type, datetime, msg }))
}

fn main() {
    let test_fill_logarg = FillLogArg::new("fixtures/log.LOG", 50, parse_log);

    let brewery = Brewery::new(4, Instant::now());
    let fill_logtea = FillLogTea::new::<LogTea>("log_tea_source", "log_fixture", test_fill_logarg);

    let new_pot = Pot::new()
        .add_source(fill_logtea);

    // Steep/Pour operations of choice

    new_pot.brew(&brewery);
}
```
