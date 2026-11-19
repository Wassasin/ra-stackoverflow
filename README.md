# Rust-analyzer stackoverflow
Run `RA_DISABLE_PERF=1 rust-analyzer analysis-stats --run-all-ide-things .` to see [output.txt].

Failure @ nightly-2026-08-06.
No problem @ nightly-2026-08-05.

Possibly due to [rust-analyzer #22948](https://github.com/rust-lang/rust-analyzer/pull/22948/changes).

```
$ rust-analyzer --version
rust-analyzer 1.99.0-nightly (7608eb7 2026-08-05)
```

```
$ rustc -V
rustc 1.99.0-nightly (7608eb7b0 2026-08-05)
```
