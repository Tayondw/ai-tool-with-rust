use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};

use csv::Reader;
use llm_chain::{executor, parameters, prompt, step::Step};

// This allows us to use the Tokio asynchronous runtime for async functionality and automatic polling of futures (values that have may or may not finished work).
#[tokio::main]
// The return type is set to Result<(), Box<dyn Error>> to enable error propagation with ? and overall error handling across different error types.
async fn main() -> Result<(), Box<dyn Error>> {
    // ...
}
