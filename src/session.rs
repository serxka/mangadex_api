use crate::error::Error;

use std::path::PathBuf;
use reqwest::header::{HeaderMap, HeaderValue, HeaderName, REFERER, USER_AGENT};

pub struct Session {
	pub session_id: String,
	pub session_remember_me_token: String,
	pub client: reqwest::Client,
}

impl Session {
	pub fn anon_connect() -> Session {
		Session {
			session_id: "NULL".to_string(),
			session_remember_me_token: "NULL".to_string(),
			client: reqwest::Client::new(),
		}
	}

	pub fn login (username: String, password: String) -> Result<Session, Error> {
		// Create a header so we dont get yeet'd out by the anti bot
		let headers: HeaderMap = {
			let mut headers = HeaderMap::new();
				headers.append(USER_AGENT, HeaderValue::from_static("Mangadex-rs"));
				headers.append(REFERER, HeaderValue::from_static("https://mangadex.org/login"));
				headers.append(HeaderName::from_static("x-requested-with"), HeaderValue::from_static("XMLHttpRequest"));
		headers };

		// Make a post form with the username and password
		let form = reqwest::multipart::Form::new()
			.text("login_username", username)
			.text("login_password", password)
			.text("remember_me", "1");

		// Create a client and log in
		let client = reqwest::Client::new();
		let mut response = client.post("https://mangadex.org/ajax/actions.ajax.php?function=login")
			.headers(headers)
			.multipart(form)
			.send()?;

		// Get the cookies as an iterator and filter them into a vector
		let mut cookies = response.headers().get_all("set-cookie").iter()
			.filter_map (|c| {
				if c.to_str().expect("Could not unwrap cookie").contains("__cfduid") {None}
				else {Some(c.to_str().expect("Could not unwrap cookie"))}
			})
			.collect::<Vec<_>>();

		unimplemented!()
	}
	pub fn token_login (path: PathBuf) -> Result<Session, Error> {
		unimplemented!()
	}
	pub fn cache_token (path: PathBuf) -> Result<(), Error> {
		unimplemented!()	
	}
}