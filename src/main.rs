fn main() {
    const VER: &str = env!("CARGO_PKG_VERSION");
    const SHORT_SHA: &str = env!("VERGEN_GIT_SHA_SHORT");
    let version = format!("{}-{}", VER, SHORT_SHA);
    println!("version: {}", version);
}
