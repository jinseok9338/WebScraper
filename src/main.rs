#![allow(unused)]
use structopt::StructOpt;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()>  {
let args = Cli::from_args();
 let f = File::open(&args.path).expect("Unable to open");
let mut reader = BufReader::new(f);
let mut line = String::new();
  for line in reader.lines() {
        println!("{}", line?);
    }
 Ok(())
}
