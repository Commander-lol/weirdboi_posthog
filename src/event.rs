use bevy_ecs::message::Message;
use bevy_ecs::resource::Resource;
use bevy_ecs::system::Commands;
use bevy_ecs::world::World;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Message, Clone)]
pub struct HogEvent {
	pub event_name: String,
	pub properties: HashMap<String, Value>,
	pub timestamp: String,
}

#[macro_export]
macro_rules! track {
    ($name:expr, {$($key:expr => [$($t:tt)+],)+}) => {
        $crate::HogEvent {
            event_name: $name.to_string(),
            properties: {
                let mut map = std::collections::HashMap::new();
                $(
                    map.insert($key.to_string(), serde_json::json!($($t)*));
                )*
                map
            },
            timestamp: std::string::String::new(),
        }
    };

    ($name: expr) => {
        $crate::HogEvent {
            event_name: $name.to_string(),
            properties: std::collections::HashMap::new(),
            timestamp: std::string::String::new(),
        }
    };
}

#[derive(Default, Resource)]
pub struct Analytics {
	events: Vec<HogEvent>,
}

impl Analytics {
	pub fn take(&mut self) -> Vec<HogEvent> {
		std::mem::take(&mut self.events)
	}
	pub fn send(&mut self, event: HogEvent) -> &mut Self {
		self.events.push(event);
		self
	}

	pub fn send_batch(&mut self, events: Vec<HogEvent>) -> &mut Self {
		self.events.extend(events);
		self
	}
}

pub trait SendAnalyticsExt {
	fn send_analytics(&mut self, event: HogEvent) -> &mut Self;
	fn send_analytics_batch(&mut self, events: Vec<HogEvent>) -> &mut Self;
}

impl SendAnalyticsExt for Commands<'_, '_> {
	fn send_analytics(&mut self, event: HogEvent) -> &mut Self {
		self.queue(move |world: &mut World| {
			world.get_resource_or_init::<Analytics>().send(event);
		});
		self
	}

	fn send_analytics_batch(&mut self, events: Vec<HogEvent>) -> &mut Self {
		self.queue(move |world: &mut World| {
			world.get_resource_or_init::<Analytics>().send_batch(events);
		});
		self
	}
}
