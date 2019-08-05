/*!
# logtea
This is a generic log file Fill Ingredient crate for use with `rettle` ETL. This crate uses [nom](https://docs.rs/nom/) as the parser library to allow any project to define how it wants to parse logs by supplying a custom built parser.

## Data Structures
- FillLogArg: Ingredient params for FillLogTea
- FillLogTea: Wrapper to simplifiy the creation of the Fill Ingredient to be used in the rettle Pot.

## Example
```ignore
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
    let mut new_pot = Pot::new();
    let fill_logtea = FillLogTea::new::<LogTea>("log_tea_source", "log_fixture", test_fill_logarg);

    new_pot.add_source(fill_logtea);

    // Steep/Pour operations of choice

    new_pot.brew(&brewery);
}
```
*/

pub mod fill;

// Re-exports
pub use self::fill::{FillLogArg, FillLogTea};
