#[test]
fn external_test() {
    let command = "ls -alF";
    let output = cli_runner::run(command);
    
    assert!(output.status.success());

    let se = cli_runner::get_stderr(&output);
    assert!(se.is_empty());

    let so = cli_runner::get_stdout(&output);
    assert!(!so.is_empty());
}