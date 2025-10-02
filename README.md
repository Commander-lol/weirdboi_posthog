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
- Send events using the track! macro and SendAnalyticsExt.

```rust
use bevy::prelude::*;
use weirdboi_posthog::{track, PosthogConfig, PosthogPlugin, SendAnalyticsExt};

fn main() {
    App::new()
        // configure PostHog
        .insert_resource(
            PosthogConfig::new("https://app.posthog.com", "PH_PROJECT_API_KEY").unwrap()
        )
        // or: PosthogConfig::from_env().unwrap() with POSTHOG_HOST and POSTHOG_KEY
        .add_plugins(MinimalPlugins)
        .add_plugins(PosthogPlugin)
        .add_systems(Update, example_system)
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

## Using the Posthog system param
```rust
use bevy::prelude::*;
use weirdboi_posthog::Posthog;

fn screen_changes(mut posthog: Posthog) {
    posthog.screen("MainMenu");
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
