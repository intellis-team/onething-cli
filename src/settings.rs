use config::Config;
use directories::ProjectDirs;
use std::error::Error;
use std::sync::Mutex;

#[derive(Clone)]
pub struct Settings {
	pub token: String,
	pub api_url: String,
}

fn load_settings() -> Result<Settings, Box<dyn Error>> {

	let config_file = ProjectDirs::from("com", "Matt Tew", "OneThing")
		.map(|project_dirs| project_dirs.config_dir().join("onething.toml"));

	let mut config_builder = Config::builder();

	if let Some(config_file) = config_file {
		config_builder = config_builder.add_source(config::File::with_name(config_file.to_str().unwrap()));
	}

	let config_values = config_builder
		.add_source(config::Environment::with_prefix("ONETHING"))
		.build()?;

	let token = config_values.get_string("token")?;
			let api_url = config_values.get_string("api_url").unwrap_or_else(|_| "https://onething.matt.tew.io/api/".to_string());

	Ok(Settings { token, api_url })

}

static SETTINGS: Mutex<Option<Settings>> = Mutex::new(None);

pub fn get_settings() -> Result<Settings, Box<dyn Error>> {
	let mut settings = SETTINGS.lock()?;

	if settings.is_none() {
		*settings = Some(load_settings()?);
	}

	Ok(settings.as_ref().unwrap().clone())
}
