use std::fmt::Display;

mod client;
mod config;
mod event;
mod interface;
mod errors;
mod plugin;

pub use config::{ConfigError, PosthogConfig};
pub use event::{Analytics, HogEvent, SendAnalyticsExt};
pub use interface::Posthog;
pub use plugin::PosthogPlugin;
pub use errors::{capture_error, capture_error_resource, ReportableError};
pub use client::{ApiError, send_event_batch, merge_event_data};