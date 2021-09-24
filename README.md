# J3RN's Brainf*ck Interpreter (JBI)

```
++++++++++[>++++++++++>+++++++++++>+++<<<-]>++++.---.>--..+++.>+++.
```

This is my first ever Rust project, so the code is probably really bad. Don't `@` me.

## Building

1. Clone the repository
2. Run `cargo build`

## Running

You have two options:

1. `cargo run`
    You will be presented with a `jbi>` prompt. The state is retained across inputs to the REPL, so try to remember what the state of your array looks like.
2. `cargo run <filename>`
    The file specified will be executed by the JBI interpreter.

## TODO

- [ ] Don't immediately give up if a loop is unclosed. Give the user the opportunity to close it.
- [ ] Tests, probably.
- [ ] J3RN's Brainf*ck Compiler (JBC) :wink:
