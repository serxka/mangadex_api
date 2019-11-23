use crate::manga::ChapterObject;
use crate::enums::{Lang, Server};
use crate::{HTTPS_URI, BASE_URL, API_CHAPTER_URL};
use crate::error::Error;
use crate::session::Session;

use std::fs::{File, DirBuilder};
use std::fmt;

use serde::Deserialize;

#[derive(Debug)]
pub struct Chapter {
	pub id: u32,
	pub timestamp: u32,
	pub title: String,
	pub volume: u8,
	pub chapter: u8,
	pub lang: Lang,
	pub long_strip: bool,

	filled: bool,
	hash: String,
	server: Server,
	pages: Vec<String>,
}

impl Chapter {
	pub fn new (id: u32, json: ChapterObject) -> Result<Chapter, Error> {
		//println!("{}", json.lang_code);
		Ok(Chapter {
			id,
			timestamp: json.timestamp,
			title: json.title,
			volume: json.volume.parse::<u8>().unwrap_or(0),
			chapter: json.chapter.parse::<u8>().unwrap_or(0),
			lang: Lang::from_str(&json.lang_code)?,
			//lang: Lang::Gb,
			filled: false,

			long_strip: false,
			hash: String::new(),
			server: Server::None,
			pages: Vec::new(),
		})
	}
	pub fn fill (session: &Session, id: u32) -> Result<Chapter, Error>{
		let json = self::ChapterJson::get(session, id)?;

		Ok(Chapter {
			id,
			timestamp: json.timestamp,
			title: json.title,
			volume: json.volume.parse::<u8>().unwrap_or(0),
			chapter: json.chapter.parse::<u8>().unwrap(),
			lang: Lang::from_str(&json.lang_code).unwrap(),

			filled: true,
			long_strip: json.long_strip != 0,
			hash: json.hash,
			server: Server::from_str(&json.server).expect("failed to unwrap server"),
			pages: json.page_array,
		})
	}
	pub fn download (&self, format: &str) -> Result<(), Error> {
		if !std::path::Path::new(format).exists() {
			DirBuilder::new().recursive(true).create(format)?;
		}
		for page in self.pages.iter() {
			let mut file = File::create(format!("{}/{}", format, page))?;
			let mut image = reqwest::get(format!("{}{}{}data/{}/{}",
					HTTPS_URI, self.server.to_str()?, BASE_URL, self.hash, page).as_str())?;
			std::io::copy(&mut image, &mut file)?;
		}
		Ok(())
	}
	pub fn len (&self) -> usize {
		self.pages.len()
	}
}

impl fmt::Display for Chapter {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "")
	}
}

// This section of code just gets response in a nice and easy format
#[derive(Debug, Deserialize)]
struct ChapterJson {
	id: u32,
	timestamp: u32,
	hash: String,
	volume: String,
	chapter: String,
	title: String,
	lang_name: String,
	lang_code: String,
	manga_id: u32,
	server: String,
	page_array: Vec<String>,
	long_strip: u8,
	status: String,
}

impl ChapterJson {
	fn get (session: &Session, id: u32) -> Result<ChapterJson, Error> {
		let json: ChapterJson = session.client
		.get(format!("{}{}{}{}", HTTPS_URI, BASE_URL, API_CHAPTER_URL, id).as_str())
		.send()?
		.json()?;
		Ok(json)
	}
}