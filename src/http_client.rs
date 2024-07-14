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
		//let body = response.text()?;
		//println!("Response body: {}", body);

		//let result = serde_json::from_str::<TOut>(&body)?;
		let result = response.json::<TOut>()?;
		Ok(result)
	} else {
		Err(Box::new(HttpError { status: response.status() }))
	}
}
