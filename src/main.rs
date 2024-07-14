mod settings;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let settings = settings::load_settings()?;

	println!("Token: {}", settings.token);
	println!("Token: {}", settings.token);

	Ok(())
}

