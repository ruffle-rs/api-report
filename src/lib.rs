pub mod report;
pub mod specification;

use anyhow::Result;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub use report::Report;
pub use specification::Specification;

pub const DEFAULT_SPEC: &[u8] = include_bytes!("../avm2_specification.json");

pub fn read_spec(path: &Path) -> Result<Specification> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}

pub fn default_spec() -> Result<Specification> {
    Ok(serde_json::from_slice(DEFAULT_SPEC)?)
}

pub fn generate_report(specification: &Specification, implementation: &Specification) -> Report {
    let mut report = Report::new();
    for (name, spec) in specification.iter() {
        report.compare_class(name, spec, implementation.get(name));
    }
    report
}
