use std::{fs::File, process::Command};

#[test]
fn ruffle_implementation() {
    let dir = env!("CARGO_MANIFEST_DIR");
    let output_path = std::env::temp_dir().join("ruffle_api_report_test_output.json");

    let status = Command::new(env!("CARGO_BIN_EXE_ruffle_api_report"))
        .args([
            "--implementation",
            &format!("{dir}/tests/assets/implementation.json"),
            "--output",
            output_path.to_str().unwrap(),
        ])
        .status()
        .expect("failed to run binary");

    assert!(status.success(), "binary exited with non-zero status");

    let actual: serde_json::Value = serde_json::from_reader(File::open(&output_path).unwrap())
        .expect("output is not valid JSON");

    let expected: serde_json::Value =
        serde_json::from_reader(File::open(format!("{dir}/tests/assets/expected.json")).unwrap())
            .expect("expected.json is not valid JSON");

    assert_eq!(actual, expected);
}
