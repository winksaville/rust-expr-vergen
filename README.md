# Experiment with vergen

As it turns out my initial attempt to use SERGEN_GIT_SEMVER
doesn't work by design. See: https://docs.rs/vergen/5.1.8/vergen/struct.Git.html
where it says:

> NOTE - The SemVer is only useful if you have tags on your repository. If your repository has no tags, this will default to CARGO_PKG_VERSION.

Instead, this does what I want:

build.rs:
```
use vergen::{vergen, Config, ShaKind};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::default();

    *config.git_mut().sha_kind_mut() = ShaKind::Short;

    // Do `cargo build -vv` to see the output
    // println!("build.rs: config: {:#?}", config);

    Ok(vergen(config)?)
```

main.rs:
```
use vergen::{vergen, Config, ShaKind};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::default();

    *config.git_mut().sha_kind_mut() = ShaKind::Short;

    // Do `cargo build -vv` to see the output
    // println!("build.rs: config: {:#?}", config);

    Ok(vergen(config)?)
```

Now running produces the expected results:
```
wink@3900x:~/prgs/rust/projects/expr-vergen (main)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/expr-vergen`
version: 0.1.0-36e7958
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
