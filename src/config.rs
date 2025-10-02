use bevy_ecs::resource::Resource;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fmt::{Display, Formatter};
use url::Url;

#[derive(Resource)]
pub struct PosthogConfig {
	enabled: bool,
	api_key: String,
	current_user: u128,
	pub api_host: Url,
	pub default_properties: HashMap<String, Value>,
}

#[derive(Debug)]
pub enum ConfigError {
	InvalidApiHost,
	MissingEnvironment(String),
}

impl Display for ConfigError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			ConfigError::InvalidApiHost => write!(f, "Invalid API host"),
			ConfigError::MissingEnvironment(env_var) => {
				write!(f, "Missing environment variable: {}", env_var)
			}
		}
	}
}

impl std::error::Error for ConfigError {}

impl PosthogConfig {
	pub fn from_env() -> Result<Self, ConfigError> {
		let api_host = env::var("POSTHOG_HOST")
			.map_err(|_| ConfigError::MissingEnvironment("POSTHOG_HOST".to_string()))?;
		let api_key = env::var("POSTHOG_KEY")
			.map_err(|_| ConfigError::MissingEnvironment("POSTHOG_KEY".to_string()))?;

		Self::new(api_host.as_str(), api_key)
	}

	pub fn new(api_host: impl TryInto<Url>, api_key: impl Display) -> Result<Self, ConfigError> {
		let random_uid = fastrand::u128(u128::MIN..u128::MAX);
		let user_id = format!("user:{:X}", random_uid);

		Ok(Self {
			enabled: true,
			api_host: api_host
				.try_into()
				.map_err(|_| ConfigError::InvalidApiHost)?,
			api_key: api_key.to_string(),
			current_user: random_uid,
			default_properties: {
				let mut map = HashMap::new();
				map.insert("distinct_id".to_string(), Value::String(user_id));
				map.insert("$process_person_profile".to_string(), Value::Bool(false));
				map.insert(
					"$lib".to_string(),
					Value::String(format!("weirdboi_posthog@{}", env!("CARGO_PKG_VERSION"))),
				);
				map
			},
		})
	}

	pub fn with_user(mut self, user_id: u128) -> Self {
		let mut default_properties = std::mem::take(&mut self.default_properties);
		default_properties.insert(
			"distinct_id".to_string(),
			Value::String(make_user_id(user_id)),
		);
		Self {
			current_user: user_id,
			default_properties,
			..self
		}
	}

	pub fn with_property(mut self, key: impl ToString, value: Value) -> Self {
		let mut default_properties = std::mem::take(&mut self.default_properties);
		default_properties.insert(key.to_string(), value);
		Self {
			default_properties,
			..self
		}
	}

	pub fn set_property(&mut self, key: impl ToString, value: Value) -> &mut Self {
		self.default_properties.insert(key.to_string(), value);
		self
	}

	pub fn identify(&mut self, user_id: u128) {
		self.current_user = user_id;
		self.default_properties.insert(
			"distinct_id".to_string(),
			Value::String(make_user_id(user_id)),
		);
	}

	pub fn token(&self) -> String {
		self.api_key.clone()
	}
	pub fn disable(&mut self) {
		self.enabled = false;
	}
	pub fn enable(&mut self) {
		self.enabled = true;
	}
	pub fn is_enabled(&self) -> bool {
		self.enabled
	}
	pub fn set_enabled(&mut self, enabled: bool) {
		self.enabled = enabled;
	}
	pub fn user_id(&self) -> String {
		make_user_id(self.current_user)
	}
}

fn make_user_id(user_id: u128) -> String {
	format!("user:{:X}", user_id)
}
