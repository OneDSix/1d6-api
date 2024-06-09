//! This file was removed from the mod.rs because i couldn't get the matrix sdk to work properly.
//! If someone wants to take a shot at this, go right ahead.
//! 
//! https://github.com/matrix-org/matrix-rust-sdk/blob/main/examples/getting_started/src/main.rs
//! https://crates.io/crates/matrix-sdk

use matrix_sdk::{
    ruma::{events::room::message::RoomMessageEventContent, RoomId},
    Client,
};

use crate::routes::errors::ApiErrors;

use super::MatrixInfo;

struct MatrixWebhook<'a> {
    pub client: Client,
    pub room_id: &'a RoomId,
    pub content: RoomMessageEventContent,
}

impl<'a> MatrixWebhook<'a> {

	pub async fn login<'b>(&mut self, info: &MatrixInfo) -> Result<(), ApiErrors<'b>> {
		self.client = Client::builder()
			.homeserver_url(&info.homeserver)
			.build()
			.await
			.map_err(|e| ApiErrors::WebhookError(e.to_string()))?;

		let _ = self.client
			.matrix_auth()
			.login_username(&info.username, &info.password)
			.initial_device_display_name("1D6 Webhook Bot")
			.await
			.map_err(|e| ApiErrors::WebhookError(e.to_string()))?;

		self.room_id = self.client.get_room(&RoomId::new(&info.room_id)).await.unwrap();
		let room = self.client.join_room_by_id().await;

		Ok(())
	}

    pub async fn send<'b>(&self) -> Result<(), ApiErrors<'b>> {
		match self.room_id.send(self.content.clone()).await {
			Ok(_) => Ok(()),
			Err(e) => Err(ApiErrors::WebhookError(e.to_string())),
		}
	}
}
