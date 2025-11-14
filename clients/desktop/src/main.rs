use desktop::Application;

use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rom_path = env::args().nth(1);
    let mut application = Application::new(rom_path)?;
    application.run()?;
    Ok(())
}
