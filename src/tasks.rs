use std::collections::HashMap;
use serde::Deserialize;
use chrono::{DateTime, Utc};

use crate::http_client::get;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FocusEventModel {
    pub task_id: Option<String>,
    pub ends: Option<DateTime<Utc>>,
    pub event_id: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Step {
    pub title: String,
    pub completed: bool,
    pub optional: bool,
    pub alternate: bool,
    pub action: Option<String>,
    //pub action_data: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub file_name: String,
    pub content_type: String,
    pub content_url: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub enum PreviewContentTypes {
    Text,
    Html,
    Markdown,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub title: String,
    pub preview: Option<String>,
    pub html_preview: Option<String>,
    pub steps: Vec<Step>,
    pub preview_content_type: PreviewContentTypes,
    pub connector: String,
    pub connector_id: Option<String>,
    pub connector_account: Option<String>,
    pub connector_account_name: Option<String>,
    pub data: Option<String>,
    pub metadata: HashMap<String, String>,
    pub private: bool,
    pub locator: Option<String>,
    pub boyancy: f32,
    pub mass: f32,
    pub time_added: DateTime<Utc>,
    pub time_expected: Option<DateTime<Utc>>,
    pub time_submerged: Option<DateTime<Utc>>,
    pub time_launched: Option<DateTime<Utc>>,
    pub time_starts: Option<DateTime<Utc>>,
    pub time_ends: Option<DateTime<Utc>>,
    pub resolved: bool,
    pub time_resolved: Option<DateTime<Utc>>,
    pub calendar_event_id: Option<String>,
    pub html_colour: Option<String>,
    pub attachments: Vec<Attachment>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskWithFocus {
    pub task: Task,
    pub focus_event: Option<FocusEventModel>,
}

pub fn get_one() -> Result<TaskWithFocus, Box<dyn std::error::Error>> {
	get::<TaskWithFocus>("tasks/one")
}
