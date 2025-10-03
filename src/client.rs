use crate::{Analytics, PosthogConfig};
use bevy_ecs::prelude::World;
use bevy_ecs::world::CommandQueue;
use ehttp::Request;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fmt::Display;
use url::Url;

pub enum Region {
	US,
	EU,
}

impl TryFrom<Region> for Url {
	type Error = url::ParseError;
	fn try_from(region: Region) -> Result<Self, Self::Error> {
		Url::parse(&region.to_string())
	}
}

impl Display for Region {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Region::US => write!(f, "https://us.posthog.com"),
			Region::EU => write!(f, "https://eu.posthog.com"),
		}
	}
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
	#[error("Failed to serialise event data: {0}")]
	SerdeError(#[from] serde_json::Error),
    #[error("Received HTTP Error, Status Code {0}")]
    HttpError(u16),
    #[error("Failed to fetch: {0}")]
    FetchError(ehttp::Error),
}

pub fn merge_event_data(
	config: &PosthogConfig,
	event_name: &str,
	properties: HashMap<String, Value>,
) -> Value {
	let mut event_properties = config.default_properties.clone();
	event_properties.extend(properties.iter().map(|(a, b)| (a.clone(), b.clone())));
	json!({
		"event": event_name,
		"properties": event_properties,
	})
}

pub async fn send_event_batch(config: &PosthogConfig, events: &[Value]) -> Result<(), ApiError> {
	let mut posthog_url = config.api_host.clone();
	posthog_url.set_path("/batch");

	let body = match serde_json::to_vec(&json!({
		"api_key": config.token(),
		"batch": events,
	})) {
		Ok(body) => body,
		Err(err) => {
			log::error!("Failed to serialize event data: {}", err);
			return Err(ApiError::SerdeError(err));
		}
	};

	match ehttp::fetch_async(Request::post(&posthog_url, body)).await {
        Ok(result) if result.ok => {
            Ok(())
        },
        Ok(result) => {
            Err(ApiError::HttpError(result.status))
        },
        Err(error) => {
            Err(ApiError::FetchError(error))
        }
    }
}
