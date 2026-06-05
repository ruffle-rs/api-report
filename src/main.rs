use anyhow::Result;
use clap::Parser;
use ruffle_api_report::{Specification, default_spec, generate_report, read_spec};
use std::path::PathBuf;

/// Creates an implementation report for ActionScript 3 (AVM2).
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Specification file to compare against.
    ///
    /// If not specified, it uses the default bundled one.
    #[arg(short, long, value_name = "SPEC")]
    specification: Option<PathBuf>,

    /// Implementation file to report on.
    #[arg(short, long, value_name = "IMPL")]
    implementation: PathBuf,

    /// Output file to report to.
    #[arg(short, long, value_name = "OUT")]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let specification: Specification = match &args.specification {
        Some(path) => read_spec(path)?,
        None => default_spec()?,
    };
    let implementation = read_spec(&args.implementation)?;
    let report = generate_report(&specification, &implementation);

    if let Some(out) = &args.output {
        report.write(out)?;
    }

    Ok(())
}
