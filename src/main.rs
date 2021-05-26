fn main() {
    const VERSION: &'static str = env!("VERGEN_GIT_SEMVER");
    println!("VERSION: {}", VERSION);
}
