mod report;
mod specification;

use crate::report::Report;
use crate::specification::Specification;
use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

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

const DEFAULT_SPEC: &[u8] = include_bytes!("../avm2_specification.json");

fn read_file(path: &Path) -> Result<Specification> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}

fn main() -> Result<()> {
    let args = Args::parse();
    let specification = match &args.specification {
        Some(path) => read_file(path)?,
        None => serde_json::from_slice(DEFAULT_SPEC)?,
    };
    let implementation = read_file(&args.implementation)?;

    let mut report = Report::new();

    for (name, spec) in specification.iter() {
        report.compare_class(name, spec, implementation.get(name));
    }

    if let Some(out) = &args.output {
        report.write(out)?;
    }

    // println!("Total points: {spec_total}. Implementation points: {imp_total}. Stub penalty: {stub_penalty}. Percentage: {:.2}", ((imp_total - stub_penalty) as f32) / (spec_total as f32) * 100.0);

    Ok(())
}
