// Based on https://matthias-research.github.io/pages/tenMinutePhysics/
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    billard_balls::simulate()?;
    Ok(())
}
