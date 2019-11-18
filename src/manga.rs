use crate::enums::{Status, Genres, Lang};
use crate::{HTTPS_URI, BASE_URL, API_MANGA_URL};
use crate::chapter::Chapter;
use crate::error::Error;

use std::collections::HashMap;
use std::path::PathBuf;
use std::fmt;

use serde::Deserialize;

#[derive(Debug)]
pub struct Manga {
	pub id: u32,
	pub title: String,
	pub description: String,
	pub cover_url: String,
	pub artist: String,
	pub author: String,
	pub status: Status,
	pub genres: Vec<Genres>,
	pub lang: Lang,
	pub hentai: bool,
	pub chapters: Vec<Chapter>,
}

impl Manga {
	pub fn new () -> Manga {
		Manga {
			id: 4294967295,
			title: String::new(),
			description: String::new(),
			cover_url: String::new(),
			artist: String::new(),
			author: String::new(),
			status: Status::Ongoing,
			genres: Vec::new(),
			lang: Lang::Jp,
			hentai: false,
			chapters: Vec::new(),
		}
	}
	pub fn from (client: &reqwest::Client, id: u32) -> Result<Manga, Error> {
		let mut manga = self::Manga::new();
		let json = self::MangaJson::get(client, id)?;
		let json_manga = json.manga.unwrap();

		manga.id = id;
		manga.title = json_manga.title;
		manga.description = json_manga.description;
		manga.cover_url = json_manga.cover_url;
		manga.artist = json_manga.artist;
		manga.author = json_manga.author;
		manga.status = Status::from_number(json_manga.status).unwrap();
		manga.lang = Lang::from_str(json_manga.lang_flag.as_str()).unwrap();
		manga.hentai = json_manga.hentai != 0;
		// Expand the genres into enums
		for genre in json_manga.genres.iter() {
			manga.genres.push(Genres::from_number(*genre).unwrap());
		}
		// Loop though the chapter hashmap and generate chapters
		for (key, value) in json.chapter.unwrap().into_iter() { // unwrap is fine here as it will always be Some(T)
			manga.chapters.push(Chapter::new(key, value)?);
		}

		Ok(manga)
	}
	pub fn download_cover () -> Vec<u8> {
		unimplemented!()
	}
	// Risky might get you blacklisted by MangaDex
	pub fn bulk_download (_path: PathBuf, _format: String) -> Result<(), Error> {
		unimplemented!()
	}
}

impl fmt::Display for Manga {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "ID: {}\nTitle: {}\nDescription: {}\nArtist: {}\nAuthor: {}\nStatus: {}\nLanguage: {}\nIs Hentai: {}\nChapters:\n\tCount: {}",
			self.id, self.title, self.description, self.artist, self.author, self.status.to_str(), self.lang.to_str_full(), self.hentai, self.chapters.len())
	}
}

// This section of code just gets response in a nice and easy format
#[derive(Debug, Deserialize)]
struct MangaObject {
	title: String,
	description: String,
	cover_url: String,
	artist: String,
	author: String,
	status: u8,
	genres: Vec<u8>,
	last_chapter: String,
	lang_name: String,
	lang_flag: String,
	hentai: u8,
}

#[derive(Debug, Deserialize)]
/*pub(super)*/pub struct ChapterObject {
	pub volume: String,
	pub chapter: String,
	pub title: String,
	pub lang_code: String,
	pub timestamp: u32, // Someone tell me how to do this ahhhhhh
}

#[derive(Debug, Deserialize)]
struct MangaJson {
	manga: Option<MangaObject>,
	chapter: Option<HashMap<u32, ChapterObject>>,
	status: String,
}

impl MangaJson {
	fn get (client: &reqwest::Client, id: u32) -> Result<MangaJson, Error> {
		let json: MangaJson = client
			.get(format!("{}{}{}{}", HTTPS_URI, BASE_URL, API_MANGA_URL, id).as_str())
			.send()?
			.json()?;
		if json.status == "Manga ID does not exist.".to_string() {
			return Err(Error::NotFound);
		}
		Ok(json)
	}
}

// Debug stuff
#[cfg(debug_assertions)]
pub fn print_manga_json_response (client: &reqwest::Client, id: u32) {
	println!("{:?}",MangaJson::get(client, id));
}