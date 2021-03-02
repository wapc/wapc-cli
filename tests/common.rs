/// Helper function to create the `wapc` binary process
#[allow(unused)]
pub(crate) fn wapc() -> std::process::Command {
    test_bin::get_test_bin("wapc")
}

#[allow(unused)]
pub(crate) fn output_to_string(output: std::process::Output) -> String {
    String::from_utf8(output.stdout).expect("failed to convert output bytes to String")
}
