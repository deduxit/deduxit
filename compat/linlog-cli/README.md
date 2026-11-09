# linlog-cli is now deduxit-cli

> **dēdūxit** [deːˈduːk.sɪt] *Latin, verb*: "it has derived", the perfect of *dēdūcō*, to lead down, derive.

The command `linlog` was renamed `deduxit` at 0.3.0, the package
[`deduxit-cli`](https://crates.io/crates/deduxit-cli) of
[deduxit](https://github.com/deduxit/deduxit), a collection of logic
suites with linear logic first. This last release of `linlog-cli`
installs a `linlog` that runs `deduxit`'s command, with the same verbs
and flags, after a line on standard error that says so; no later
release of `linlog-cli` will be made.

Install the command under its new name:

```sh
cargo install --locked deduxit-cli
```

The [manual](https://deduxit.github.io/deduxit/manual/) describes it.
