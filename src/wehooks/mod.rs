pub mod discord;
//pub mod matrix;
// Read matrix.rs for more info

struct DiscordInfo {
	pub webhook: String
}

struct MatrixInfo {
	pub homeserver: &'static str,
	pub username: &'static str,
	pub password: &'static str,
	pub room_id: &'static str,
}

struct Message {
	pub discord: DiscordInfo,
	pub matrix: MatrixInfo
}

impl Message {

}
