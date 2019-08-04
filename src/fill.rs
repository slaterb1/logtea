use rettle::ingredient::{Ingredient, Argument, Fill};
use rettle::brewer::{Brewery, make_tea};
use rettle::tea::Tea;

use std::sync::{Arc, RwLock};
use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::any::Any;
use std::fmt::Debug;

use nom::IResult;

///
/// Ingredient params for FillLogTea.
pub struct FillLogArg<T> where
    T: Tea + Send + Debug + Sized + 'static,
{
    /// The filepath to the csv that will be processed.
    filepath: String,
    buffer_length: usize,
    parser: fn(&'static str) -> IResult<&str, T>,
}

impl<T> FillLogArg<T> where
    T: Tea + Send + Debug + Sized + 'static,
{
    ///
    /// Returns a FillLogArg to be used as params in FillLogTea.
    ///
    /// # Arguments
    ///
    /// * `filepath` - filepath for log file to load.
    /// * `buffer_length` - number of lines to process at a time.
    /// * `parser` - nom parser to parse data from lines
    pub fn new(filepath: &str, buffer_length: usize, parser: fn(&str) -> IResult<&str, T>) -> FillLogArg<T> {
        let filepath = String::from(filepath);
        FillLogArg { filepath, buffer_length, parser }
    }
}

impl<T> Argument for FillLogArg<T> where
    T: Tea + Send + Debug + Sized + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

///
/// Wrapper to simplifiy the creation of the Fill Ingredient to be used in the rettle Pot.
pub struct FillLogTea {}

impl FillLogTea {
    ///
    /// Returns the Fill Ingredient to be added to the `rettle` Pot.
    ///
    /// # Arguments
    ///
    /// * `name` - Ingredient name
    /// * `source` - Ingredient source
    /// * `params` - Params data structure holding the `filepath`, `buffer_length`, and `parser`
    pub fn new<T: Tea + Send + Debug + Sized + 'static>(name: &str, source: &str, params: FillLogArg<T>) -> Box<Fill> {
        Box::new(Fill {
            name: String::from(name),
            source: String::from(source),
            computation: Box::new(|args, brewery, recipe| {
                fill_from_log::<T>(args, brewery, recipe);
            }),
            params: Some(Box::new(params))
        })
    }
}

/// Helper function that sends to batch request to Brewers for processing.
///
/// # Arguments
///
/// * `brewery` - Brewery that processes the data.
/// * `recipe` - Recipe for the ETL used by the Brewery.
/// * `tea_batch` - Current batch to be sent and processed
fn call_brewery(brewery: &Brewery, recipe: Arc<RwLock<Vec<Box<dyn Ingredient + Send + Sync>>>>, tea_batch: Vec<Box<dyn Tea + Send>>) {
    brewery.take_order(|| {
        make_tea(tea_batch, recipe);
    });
}

///
/// Implements the log file read, deserialization to specified data struct, and passes the data to the
/// brewery for processing.
///
/// # Arguments
///
/// * `args` - Params specifying the filepath of the csv.
/// * `brewery` - Brewery that processes the data.
/// * `recipe` - Recipe for the ETL used by the Brewery.
fn fill_from_log<T: Tea + Send + Debug + Sized + 'static>(args: &Option<Box<dyn Argument + Send>>, brewery: &Brewery, recipe: Arc<RwLock<Vec<Box<dyn Ingredient + Send + Sync>>>>) {
    match args {
        None => (),
        Some(box_args) => {
            // Unwrap params.
            let box_args = box_args.as_any().downcast_ref::<FillLogArg<T>>().unwrap();
            
            // Initialize reader with specified file from path.
            let file = match File::open(&box_args.filepath) {
                Ok(file) => file,
                Err(e) => {
                    println!("Failed opening file! Error: {:}", e);
                    return
                },
            };

            let reader = BufReader::new(file);
            let parser = &box_args.parser;
            
            // Iterate over csv lines and push data into processer
            let mut tea_batch: Vec<Box<dyn Tea + Send>> = Vec::with_capacity(box_args.buffer_length);
            for line in reader.lines() {
                // Check if batch size has been reached and send to brewers if so.
                if tea_batch.len() == box_args.buffer_length {
                    let recipe = Arc::clone(&recipe);
                    call_brewery(brewery, recipe, tea_batch);
                    tea_batch = Vec::with_capacity(box_args.buffer_length);
                }
                let (_input, tea) = parser(&line.unwrap()[..]).unwrap();
                tea_batch.push(Box::new(tea));
            }
            let recipe = Arc::clone(&recipe);
            call_brewery(brewery, recipe, tea_batch);
        }
    }
}

