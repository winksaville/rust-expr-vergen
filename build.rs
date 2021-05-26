use vergen::{Config, vergen};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::default();
    vergen(config)?;

    Ok(())
}
