# CLI Runner
CLI runner library for Rust - run a full command line String

This is a library which makes it convenient to execute a command line String.

It is using shell_words crate to parse the command line, allowing it to accept a command as it would be issued in a shell.

This is a multi-platform library.

# Example

```rust
let cmd = "ls -alF";
let output = run(cmd);

assert!(output.status.success());

let so = get_stdout(&output);
assert!(!so.is_empty());

let se = get_stderr(&output);
assert!(se.is_empty());
```

# License

See [License](LICENSE)
