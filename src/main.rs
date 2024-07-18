use std::collections::hash_map::HashMap;
use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::Verbosity;
use human_panic::setup_panic;

mod error;

use error::Result;

fn main() -> Result<()> {
    setup_panic!();
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    let template = mustache::compile_path(args.template)?;
    let mut reader = csv::Reader::from_path(args.data_file)?;

    for record in reader.deserialize() {
        let record: HashMap<String, String> = record?;
        let output = template.render_to_string(&record)?;
        println!("{}", output);
        println!();
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The location of the multashe template to render.
    #[arg(short = 'm', long = "mustache")]
    template: PathBuf,

    /// The data file containt the values to add to the file. Currently, this should be a CSV file.
    #[arg(short = 'd', long = "data")]
    data_file: PathBuf,

    #[command(flatten)]
    verbose: Verbosity,
}
