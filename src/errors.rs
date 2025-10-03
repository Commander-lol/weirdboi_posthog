use crate::{track, Analytics};
use bevy_ecs::error::{BevyError, DefaultErrorHandler, ErrorContext, ErrorHandler};
use bevy_ecs::world::World;
use serde_json::json;
use std::collections::HashMap;
use bevy_ecs::resource::Resource;

pub trait ReportableError {}

pub fn capture_error(error: BevyError, context: ErrorContext) {
	send_error_to_posthog(&error, &context);
	bevy_ecs::error::panic(error, context);
}

pub fn capture_error_resource() -> impl Resource {
    DefaultErrorHandler(capture_error)
}

fn send_error_to_posthog(error: &BevyError, context: &ErrorContext) {
	// let mut fingerprint_parts = Vec::new();
	let parts = format!("{}", error)
		.split('\n')
		.map(ToString::to_string)
		.collect::<Vec<_>>();

	eprintln!("ERROR PARTS {:?}", parts);

	// let event = track!("error", {
	//     "error" => [error.to_string()],
	//     "error_type" => [error.type_name()],
	//     "context" => [format!("{:?}", context)],
	// });
	//
	// if let Some(world) = context.world() {
	//     if let Some(analytics) = world.get_resource_mut::<Analytics>() {
	//         analytics.send(event);
	//     }
	// }
	//
	// let mut error_properties = HashMap::new();
}
