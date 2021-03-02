mod common;
use common::wapc;

#[test]
fn integration_help_check() {
    let help_output = wapc()
        .args(&["--help"])
        .output()
        .expect("failed to display help text");
    assert!(help_output.status.success());
}
