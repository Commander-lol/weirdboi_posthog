use crate::{Analytics, HogEvent, PosthogConfig, SendAnalyticsExt, track};
use bevy_ecs::system::{ResMut, SystemParam};
use serde_json::json;
use std::fmt::Display;

#[derive(SystemParam)]
pub struct Posthog<'w> {
	config: Option<ResMut<'w, PosthogConfig>>,
	events: ResMut<'w, Analytics>,
}

impl SendAnalyticsExt for Posthog<'_> {
	fn send_analytics(&mut self, event: HogEvent) -> &mut Self {
		self.events.send(event);
		self
	}

	fn send_analytics_batch(&mut self, events: Vec<HogEvent>) -> &mut Self {
		self.events.send_batch(events);
		self
	}
}

impl Posthog<'_> {
	pub fn screen(&mut self, screen_name: impl Display) {
		if let Some(ref mut config) = self.config {
			config.set_property("$screen_name", json!(screen_name.to_string()));
		}

		self.send_analytics(track!("$screen", {
			"$screen_name" => [screen_name.to_string()],
		}));
	}
}
