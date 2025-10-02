use std::fmt::Display;

mod client;
mod config;
mod event;
mod interface;
mod plugin;

pub use config::{ConfigError, PosthogConfig};
pub use event::{Analytics, HogEvent, SendAnalyticsExt};
pub use interface::Posthog;
pub use plugin::PosthogPlugin;
