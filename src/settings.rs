use config::Config;
use directories::ProjectDirs;
use std::error::Error;

pub struct Settings {
	pub token: String,
	pub api_url: String,
}

pub fn loadSettings() -> Result<Settings, Box<dyn Error>> {
	let project_dirs = ProjectDirs::from("com", "Matt Tew", "OneThing");
	let config_dir =  project_dirs.config_dir();
	let config_file = config_dir.join("onething.toml");

	let config_file = match ProjectDirs::from("com", "Matt Tew", "OneThing") {
		Some(project_dirs) => project_dirs.config_dir().join("onething.toml"),
		None => None,
	};

	let config_builder = Config::builder();

	if Some(config_file) {
		config_builder.add_source(config::File::with_name(config_file.to_str()?));
	}

	let config_values = match config_builder
		.add_source(config::Environment::with_prefix("ONETHING"))
		.build();

	match config_values {
		Ok(config) => {
			config.try_into()
		}
		Err(e) => Err(Box::new(e)),
	}
}
