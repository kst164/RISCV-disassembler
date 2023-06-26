The disassembler is written in [Rust](https://rust-lang.org/).

To run, `cd` into the `disassembler` folder and run one of the following:

```bash
cargo r -- -u # for unlabelled assembly code
cargo r -- -l # for labelled assembly code
cargo r       # defaults to labelled
```

(cargo is the Rust package manager, similar to npm/pip)

Input should have one instruction per line, each line having 8 hex characters.

The input text is read as lines from stdin. If the machine code is in a file, then it can be passed by piping:

```bash
cargo r < input.txt
```
