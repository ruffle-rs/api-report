# API Report

This is a tool used to generate a progress report from two given API states, a 'specification' and an 'implementation'.

[Ruffle](https://github.com/ruffle-rs/ruffle/) uses this to create their automatic Flash AVM2 progress report,
[as seen on their website](https://ruffle.rs/compatibility/avm2).

## Building from source

[Follow the official guide](https://www.rust-lang.org/tools/install) to install Rust for your platform.

Then simply `cargo build --release` to build the binary,
or in the instructions below replace `avm_report` with `cargo run --` to run the tool directly.

## Usage

To see the full help info, run `avm_report --help`.

You'll need two files, a `specification` that describes the API and an `implementation` that describes the current state
of the API implemented. We provide an AVM2 specification in the repo for Ruffle's own use.

Run `avm_report -s specification.json -i implementation.json -o report.json`.

## Specification / Implementation format

If you read Rust, there's a general view of the structure over at [src/specification.rs](src/specification.rs).

## Report format

This tool quanitifies progress with a points system:
- Any "item" in `specification` is worth 1 point. If it's a function or a class or a field, doesn't matter. This is totalled in `max_points`.
- If an item from `specification` exists in `implementation`, you gain a point in `impl_points`.
- If that same item is marked as a "stub" in `implementation`, you gain a point in `stub_penalty`.

To calculate a percentage, it's `impl_points / max_points` to see what exists, or `(impl_points - stub_penalty) / max_points` to see what's actually **done**.

This allows you to show the stubs differently on a progress bar, for example.
