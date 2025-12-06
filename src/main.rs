use reprack::bundle;

use anyhow::Result;

fn main() -> Result<()> {
    print!("{}", bundle()?);
    Ok(())
}
