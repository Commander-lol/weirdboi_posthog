use crate::event::HogEvent;
use crate::{Analytics, PosthogConfig};
use bevy_app::{App, Last, Plugin};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::resource_exists;
use bevy_ecs::schedule::IntoScheduleConfigs;
use bevy_ecs::system::{Commands, Local, Query, Res, ResMut};
use bevy_ecs::world::{CommandQueue, World};
use bevy_tasks::{AsyncComputeTaskPool, Task, block_on, poll_once};
use bevy_time::{Stopwatch, Time};
use ehttp::Request;
use serde_json::json;

pub struct PosthogPlugin;

#[derive(Component)]
pub struct AnalyticsTask(Task<CommandQueue>);

pub fn flush_queue(
	time: Res<Time>,
	config: Res<PosthogConfig>,
	mut timer: Local<Stopwatch>,
	mut commands: Commands,
	mut events: ResMut<Analytics>,
) {
	timer.tick(time.delta());
	if timer.elapsed_secs() > 1.0 {
		timer.reset();
	} else {
		return;
	}

	let all_events: Vec<HogEvent> = events.take();
	if !config.is_enabled() || all_events.len() == 0 {
		// Do nothing, discard events
		return;
	}

	let mut posthog_url: url::Url = config.api_host.clone();
	let posthog_properties = config.default_properties.clone();
	let posthog_token = config.token();

	let task_pool = AsyncComputeTaskPool::get();
	let flush_task = task_pool.spawn(async move {
		posthog_url.set_path("/batch");
		let mut transformed_events = Vec::with_capacity(all_events.len());

		for event in &all_events {
			let mut event_properties = posthog_properties.clone();
			event_properties.extend(event.properties.iter().map(|(a, b)| (a.clone(), b.clone())));

			transformed_events.push(json!({
				"event": event.event_name,
				"properties": event_properties,
			}));
		}

		let body = match serde_json::to_vec(&json!({
			"api_key": posthog_token.clone(),
			"batch": transformed_events,
		})) {
			Ok(body) => body,
			Err(err) => {
				log::error!("Failed to serialize event data: {}", err);
				let mut command_queue = CommandQueue::default();
				command_queue.push(move |world: &mut World| {
					world
						.get_resource_or_init::<Analytics>()
						.send_batch(all_events);
				});
				return command_queue;
			}
		};

		let mut command_queue = CommandQueue::default();
		match ehttp::fetch_async(Request::post(&posthog_url, body)).await {
			Ok(response) => {
				log::info!("Sent {} events to Posthog", transformed_events.len());
			}
			Err(err) => {
				log::error!("Failed to send events to Posthog: {}", err);
				command_queue.push(move |world: &mut World| {
					world
						.get_resource_or_init::<Analytics>()
						.send_batch(all_events);
				});
			}
		}

		command_queue
	});

	commands.spawn(AnalyticsTask(flush_task));
}

pub fn poll_analytics_task(mut commands: Commands, mut task: Query<(Entity, &mut AnalyticsTask)>) {
	for (entity, mut task) in &mut task.iter_mut() {
		if let Some(mut command_queue) = block_on(poll_once(&mut task.0)) {
			commands.entity(entity).despawn();
			commands.append(&mut command_queue);
		}
	}
}

impl Plugin for PosthogPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<Analytics>().add_systems(
			Last,
			(flush_queue, poll_analytics_task).run_if(resource_exists::<PosthogConfig>),
		);
	}
}
