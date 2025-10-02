# weirdboi_posthog

Bevy integration with Posthog analytics.

## Install
Add to Cargo.toml:

```toml
[dependencies]
weirdboi_posthog = "0.1"
```

## Quick start
- Create a PosthogConfig and insert it as a Bevy resource.
- Add the PosthogPlugin.
- Send events using the track! macro and either `Commands` or `Posthog` system params, via SendAnalyticsExt trait.

## Usage

### Simple Config

```rust
use bevy::prelude::*;
use weirdboi_posthog::{track, PosthogConfig, PosthogPlugin, SendAnalyticsExt};

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(PosthogPlugin)
        .add_systems(Update, example_system)
        // configure PostHog
        .insert_resource(
            PosthogConfig::new("https://app.posthog.com", "PH_PROJECT_API_KEY").unwrap()
        )
        // or: PosthogConfig::from_env().unwrap() with POSTHOG_HOST and POSTHOG_KEY
        .run();
}

fn example_system(mut commands: Commands) {
    commands
        .send_analytics(track!("game_started"));

    commands
        .send_analytics(track!("level_completed", {
            "level" => [1],
            "time_ms" => [12345],
        }));
}
```

### Using the Posthog system param

```rust
use bevy::prelude::*;
use weirdboi_posthog::Posthog;

fn screen_changes(mut posthog: Posthog) {
    posthog.send_analytics(track!("game_over"));
    posthog.screen("MainMenu");
}
```

### Real world example

This example conditionally initialises the posthog system and sets custom global params to include

```rust
use bevy::prelude::*;
use weirdboi_posthog::{Posthog, PosthogPlugin};

/// Perform some logic to generate an anonymous per-install identifier. In a real project:
/// - Fetch and return if it exists
/// - Create and store if it does not
///
/// Using a random per-install identifier allows correlation of game sessions and events
/// without making a user identifiable. There are a variety of events such as reinstalling
/// the game that should regenerate such an identifier.
/// 
/// weirdboi_posthog will generate a new ID each time it is initialised if no user_id is provided.
/// 
/// Fun fact: a UUID is a 128 bit integer, consider using the `uuid` library where you can call
/// `Uuid::as_u128` to get a valid user id
fn fetch_cached_user_id() -> u128 {
    fastrand::u128(u128::MIN..u128::MAX)
}

fn main() {
    let user_id = fetch_cached_user_id();
    
    let mut app = App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(PosthogPlugin);
    
    if let Ok(config) = weirdboi_posthog::PosthogConfig::from_env() {
        app.insert_resource(
            config
                .with_property("game_version", json!(env!("CARGO_PKG_VERSION")))
                .with_user(fetch_cached_user_id()),
        );
    } else {
        bevy::log::warn_once!(
			"Failed to load PosthogConfig from environment variables. Please set the POSTHOG_API_KEY and POSTHOG_HOST environment variables."
		);
    }
}
```

## Configuration
Environment variables supported by PosthogConfig::from_env():
- POSTHOG_HOST: e.g. https://app.posthog.com or your self-hosted URL
- POSTHOG_KEY: Project API key

Common configuration methods:
- identify(user_id: u128) — set the distinct_id
- set_property(key, value) — set a default property for all events
- set_enabled(bool), enable(), disable() — control sending
- user_id() — current distinct_id string

## Event batching
- Events are stored in an Analytics resource and flushed roughly once per second by the plugin.
- If sending fails or serialization errors occur, events are re-queued.

## Notes
- Requires Bevy (uses bevy_app, bevy_ecs, bevy_time, and tasks).
- Uses ehttp for HTTP requests.
