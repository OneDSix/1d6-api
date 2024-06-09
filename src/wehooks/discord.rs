//! Based off https://github.com/modrinth/labrinth/blob/master/src/util/webhook.rs

use std::u32::MAX;

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::routes::errors::ApiErrors;

#[derive(Serialize)]
struct DiscordEmbed {
	pub author: Option<DiscordEmbedAuthor>,
	pub title: String,
	pub description: String,
	pub url: String,
	pub timestamp: DateTime<Utc>,
	pub color: u32,
	pub fields: Vec<DiscordEmbedField>,
	pub thumbnail: DiscordEmbedThumbnail,
	pub image: Option<DiscordEmbedImage>,
	pub footer: Option<DiscordEmbedFooter>,
}

#[derive(Serialize)]
struct DiscordEmbedAuthor {
	pub name: String,
	pub url: Option<String>,
	pub icon_url: Option<String>,
}

#[derive(Serialize)]
struct DiscordEmbedField {
	pub name: &'static str,
	pub value: String,
	pub inline: bool,
}

#[derive(Serialize)]
struct DiscordEmbedImage {
	pub url: Option<String>,
}

#[derive(Serialize)]
struct DiscordEmbedThumbnail {
	pub url: Option<String>,
}

#[derive(Serialize)]
struct DiscordEmbedFooter {
	pub text: String,
	pub icon_url: Option<String>,
}

#[derive(Serialize)]
struct DiscordWebhook {
	pub avatar_url: Option<String>,
	pub username: Option<String>,
	pub embeds: Vec<DiscordEmbed>,
	pub content: Option<String>,
}

impl DiscordWebhook {
	#[allow(dead_code)]
	pub async fn discord_hook(&self) -> Result<(), ApiErrors> {
		let client = reqwest::Client::new();
		client
			.post("None".to_string())
			.json(
				&DiscordWebhook {
					avatar_url: None,
					username: None,
					embeds: vec![
						DiscordEmbed {
							author: None,
							url: "None".to_string(),
							title: "None".to_string(),
							description: "None".to_string(),
							timestamp: Utc::now(),
							color: MAX,
							fields: vec![
								DiscordEmbedField {
									name: "None",
									value: "None".to_string(),
									inline: false,
								}
							],
							thumbnail: DiscordEmbedThumbnail {
								url: None,
							},
							image: None,
							footer: None,
						}
					],
					content: None,
				}
			)
			.send()
			.await
			.map_err(|e| ApiErrors::WebhookError(e.to_string()).into())?;
		Ok(())
	}
}
