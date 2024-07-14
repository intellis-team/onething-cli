use settings::Settings;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error> {
	let settings = match settings::loadSettings()?;

	println!("Token: {}", settings.token);
	println!("Token: {}", settings.token);
}

