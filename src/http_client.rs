use reqwest::blocking::Client;
use std::error::Error;
use serde::Deserialize;
use crate::settings;
use std::fmt::Display;


#[derive(Debug)]
struct HttpError {
	status: reqwest::StatusCode,
}

impl Display for HttpError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "HTTP error: {}", self.status)
	}
}

impl Error for HttpError {}

const fn fix_empty_json_str(json: &str) -> &str {
	if json.is_empty() {
		"null"
	} else {
		json
	}
}

pub fn get<TOut>(url: &str) -> Result<TOut, Box<dyn Error>>
where
	TOut: for<'de> Deserialize<'de>
{
	let client = Client::new();
	let settings = settings::get_settings()?;

	let url = format!("{}{}", settings.api_url, url);

	//println!("GET {}", url);

	let response = client
		.get(&url)
		.header("Authorization", format!("Bearer {}", settings.token))
		.send()?;

	//println!("Response: {:?}", response);

	if response.status().is_success() {
		let body = response.text()?;
		//println!("Response body: {}", body);

		match serde_json::from_str(fix_empty_json_str(&body)) {
			Ok(result) => Ok(result),
			Err(e) => {
				println!("Error parsing JSON: {}", e);
				Err(Box::new(e))
			}
		}
		//let result: TOut = serde_json::from_str(&body)?;
		//Ok(result)
	} else {
		Err(Box::new(HttpError { status: response.status() }))
	}
}
