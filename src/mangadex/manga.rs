use super::enums::{Status, Genres, Lang};
use super::{HTTPS_URI, BASE_URL, API_MANGA_URL};
use super::chapter::Chapter;
use super::error::Error;

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
	pub fn from (client: &reqwest::Client, id: u32) -> Result<Manga, reqwest::Error> { // idk how to get this working with my error
		let mut manga = self::Manga::new();
		let json = self::MangaJson::get(client, id)?;

		manga.id = id;
		manga.title = json.manga.title;
		manga.description = json.manga.description;
		manga.cover_url = json.manga.cover_url;
		manga.artist = json.manga.artist;
		manga.author = json.manga.author;
		manga.status = Status::from_number(json.manga.status).unwrap();
		manga.lang = Lang::from_str(json.manga.lang_flag.as_str()).unwrap();
		manga.hentai = json.manga.hentai != 0;
		// Expand the genres into enums
		for genre in json.manga.genres.iter() {
			manga.genres.push(Genres::from_number(*genre).unwrap());
		}
		// Loop though the chapter hashmap and generate chapters
		for (key, value) in json.chapter.into_iter() {
			manga.chapters.push(Chapter::new(key, value).unwrap());
		}

		Ok(manga)
	}
	pub fn download_cover () -> Vec<u8> {
		unimplemented!()
	}
	// Risky might get you blacklisted by MangaDex
	pub fn bulk_download (path: PathBuf, format: String) -> Result<(), Error> {
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
	manga: MangaObject,
	chapter: HashMap<u32, ChapterObject>,
	status: String,
}

impl MangaJson {
	fn get (client: &reqwest::Client, id: u32) -> Result<MangaJson, reqwest::Error> {
		client
			.get(format!("{}{}{}{}", HTTPS_URI, BASE_URL, API_MANGA_URL, id).as_str())
			.send()?
			.json()
	}
}

// Debug stuff
#[cfg(debug_assertions)]
pub fn print_manga_json_response (client: &reqwest::Client, id: u32) {
	println!("{:?}",MangaJson::get(client, id));
}