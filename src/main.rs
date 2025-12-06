use library_bundler::bundle;

use anyhow::Result;

fn main() -> Result<()> {
    print!("{}", bundle()?);
    Ok(())
}
