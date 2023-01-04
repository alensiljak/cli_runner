/*!
 * CLI-app runner.
 */

 use std::process::{Command, Output};

 /// Runs a CLI application.
 ///
 /// Runs a given command with parameters. The command is given as &str.
 ///
 /// Returns the output of the CLI application as a string.
 ///
 /// Example:
 /// ```
 /// let cmd = r#"ledger r -b 2022-03-01 -d  "(account =~ /income/ and account =~ /ib/) or (account =~ /ib/ and account =~ /withh/)" --init-file tests/init.ledger"#;
 /// let output = cli_runner::run(cmd);
 /// assert!(output.error.is_empty());
 /// ```
 pub fn run(command: &str) -> Output {
     // parse attributes
     let args = shell_words::split(command)
         .expect("command arguments parsed");
 
     // the first argument is the application name
     let program = &args[0];
     let prog_args = &args[1..];
 
     Command::new(program)
         .args(prog_args)
         .output()
         .expect("CLI output")
 }
 
 /// Returns `stdout` as UTF-8 &str.
 pub fn get_stdout<'a>(output: &'a Output) -> &'a str {
     core::str::from_utf8(&output.stdout).expect("stdout as &str")
     // String::from_utf8(output.stdout).expect("stdout as string")
 }
 
 /// Returns `stderr` as UTF-8 &str.
 pub fn get_stderr<'a>(output: &'a Output) -> &'a str {
     core::str::from_utf8(&output.stderr).expect("stdout as &str")
     // String::from_utf8(output.stderr).expect("stderr as string")
 }
 
 //// Tests

 #[cfg(test)]
 mod tests {
    use super::{run, get_stdout, get_stderr};

    #[test]
    fn basic_test() {
        let cmd = "ls -alF";
        let output = run(cmd);

        assert!(output.status.success());

        let so = get_stdout(&output);
        assert!(!so.is_empty());

        let se = get_stderr(&output);
        assert!(se.is_empty());
    }
 }
 