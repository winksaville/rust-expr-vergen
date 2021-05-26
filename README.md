# Experiment with vergen

Attempting to use VERGEN_GIT_SEMVER from [vergen](https://crates.io/crates/vergen),
my program is outputing `VERSION: 0.1.0` but I'm expecting something like
`VERSION: 0.1.0-gf49246c`.

```
wink@3900x:~/prgs/rust/projects/expr-vergen (main)
$ cargo run
   Compiling expr-vergen v0.1.0 (/home/wink/prgs/rust/projects/expr-vergen)
    Finished dev [unoptimized + debuginfo] target(s) in 0.94s
     Running `target/debug/expr-vergen`
VERSION: 0.1.0
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
