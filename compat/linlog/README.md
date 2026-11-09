# linlog is now deduxit

> **dēdūxit** [deːˈduːk.sɪt] *Latin, verb*: "it has derived", the perfect of *dēdūcō*, to lead down, derive.

The crate `linlog` was renamed
[`deduxit`](https://crates.io/crates/deduxit) at 0.3.0, the library of
[deduxit](https://github.com/deduxit/deduxit), a collection of logic
suites with linear logic first. This last release under the old name
re-exports `deduxit`, so a project that names `linlog` still builds; no
later release of `linlog` will be made.

Depend on the new name:

```toml
[dependencies]
deduxit = "0.3"
```

Up to 0.2.0 the linear suite's items were at the crate's root; they are
under `deduxit::linear` now. The
[changelog](https://github.com/deduxit/deduxit/blob/main/CHANGELOG.md)
lists what changed.
