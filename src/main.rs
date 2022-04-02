use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    billard_balls::simulate()?;
    Ok(())
}
