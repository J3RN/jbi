# J3RN's Brainf*ck Interpreter (JBI)

```
++++++++++[>++++++++++>+++++++++++>+++<<<-]>++++.---.>--..+++.>+++.
```

This is my first ever Rust project, so the code is probably really bad. Don't `@` me.

## Building

1. Clone the repository
2. Run `cargo build`

## Running

Run `cargo run`

You will be presented with a `jbi>` prompt. The state is retained across inputs to the REPL, so try to remember what the state of your array looks like.

## TODO

- [ ] Don't immediately give up if a loop is unclosed. Give the user the opportunity to close it.
- [ ] Allow the user to pass a file to interpret instead of using the REPL.
- [ ] Tests, probably.
- [ ] J3RN's Brainf*ck Compiler (JBC) :wink:
