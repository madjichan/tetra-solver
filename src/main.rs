mod error;
mod graph;
mod field;
mod solution;
mod task;

use std::{
    error::Error,
    env,
    io,
    fs::File,
};

use crate::error::TetraError;
use crate::task::Task;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Box::new(TetraError::ARGUMENTS_SYNTAX));
    }

    let file = File::open(&args[1])?;
    let reader = io::BufReader::new(file);

    let task = Task::parse_input(reader)?;

    let solves = task.solve();

    println!("Founded {} solves", solves.len());

    for (i, s) in solves.iter().enumerate() {
        println!("\nTASK #{}:{}", i+1, task.display(s));
    }

    Ok(())
}
