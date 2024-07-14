use config::Config;
use directories::ProjectDirs;
use std::error::Error;

pub struct Settings {
	pub token: String,
	pub api_url: String,
}

pub fn load_settings() -> Result<Settings, Box<dyn Error>> {

	let config_file = match ProjectDirs::from("com", "Matt Tew", "OneThing") {
		Some(project_dirs) => Some(project_dirs.config_dir().join("onething.toml")),
		None => None,
	};

	let mut config_builder = Config::builder();

	if config_file.is_some() {
		config_builder = config_builder.add_source(config::File::with_name(config_file.unwrap().to_str().unwrap()));
	}

	let config_values = config_builder
		.add_source(config::Environment::with_prefix("ONETHING"))
		.build();

	match config_values {
		Ok(config_values) => {
			let token = config_values.get_string("token")?;
			let api_url = match config_values.get_string("api_url") {
				Ok(url) => url,
				Err(_) => "https://api.matt.tew.io/".to_string(),
			};

			Ok(Settings { token, api_url })
		}
		Err(e) => Err(Box::new(e)),
	}
}
