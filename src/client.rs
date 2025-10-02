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
