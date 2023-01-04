/*!
 * CLI Runner
 * 
 * is a library which makes it convenient to execute a command line String.
 * It is using shell_words crate to parse the command line, allowing it to accept a command
 * as it would be issued in a shell.
 * This is a multi-platform library.
 * 
 * Example:
 * ```
 * use cli_runner::{run, get_stdout, get_stderr};
 * 
 * let cmd = "ls -alF";
 * let output = run(cmd);
 * 
 * assert!(output.status.success());
 * 
 * let so = get_stdout(&output);
 * assert!(!so.is_empty());
 * 
 * let se = get_stderr(&output);
 * assert!(se.is_empty());
 * ```
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
 /// let cmd = r#"ls -alF"#;
 /// let output = cli_runner::run(cmd);
 /// assert!(output.status.success());
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
 