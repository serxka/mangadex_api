use crate::error::Error;

pub struct Session {
	pub session_id: String,
	pub session_remember_me_token: String,
	pub client: reqwest::Client,
}

impl Session {
	pub fn login (session_id: String, remember_me_token: String) -> Result<Session, Error> {
		unimplemented!()
	}
	pub fn generate_token (username: String, password: String, remember_me: bool) -> Result<(String, String), Error> {
		unimplemented!()
	}
}