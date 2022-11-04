use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

mod pdf_playground;
mod ros_parser;

use crate::pdf_playground::make_test_pdf;
use crate::ros_parser::Roster;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to roster (.ros/.rosz) file
    in_path: PathBuf,

    /// Make a PDF?
    #[arg(short, long, default_value_t = false)]
    make_pdf: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let ros = Roster::from_file(&args.in_path).context("Could not parse roster")?;
    dbg!(ros);
    if args.make_pdf {
        make_test_pdf();
    }
    Ok(())
}
