use serde::Deserialize;
use chrono::{DateTime, Utc};
use colored::{Colorize, ColoredString};

use crate::http_client::get;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Status {
	pub do_not_disturb: bool,
	pub ends: Option<DateTime<Utc>>,
	pub reason: Option<String>,
}

impl Status {
		pub fn colored_time(&self) -> ColoredString {

				match self.ends {
						Some(ends) => {
								let now = Utc::now();
								let duration = ends.signed_duration_since(now);
								let hours = duration.num_hours();
								let minutes = duration.num_minutes() % 60;
								let time = format!("{:02}:{:02}", hours, minutes);

								if duration.num_minutes() < 1 {
										time.truecolor(0xFF, 0x7B, 0x00)
								} else if duration.num_minutes() < 5 {
										time.truecolor(0xFF, 0xC0, 0x86)
								}	else {
										time.truecolor(0xB8, 0x73, 0x33)
								}
						}
						None => "".clear()
				}

		}
}

pub fn get_status() -> Result<Option<Status>, Box<dyn std::error::Error>> {
	get::<Option<Status>>("dnd")
}
