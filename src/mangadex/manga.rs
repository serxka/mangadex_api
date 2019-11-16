use crate::mangadex::{BASE_URI,API_MANGA_URI};
use crate::mangadex::enums::{Genres,Lang,Status};
use crate::mangadex::chapter::Chapter;

use reqwest;
use serde_json::{Value};

#[derive(Debug)]
pub struct Manga {
	pub id: u32,
	pub cover_url: String,
	pub description: String,
	pub title: String,
	pub artist: String,
	pub author: String,
	pub status: Status,
	pub genres: Vec<Genres>,
	pub lang: Lang,
	pub hentai: bool,
	pub chapters: Vec<Chapter>,
}

impl Manga {
	// Create a manga and fill it with data from id
	pub fn new (id: u32) -> Result<Manga, reqwest::Error> {
		let mut genres: Vec<Genres> = Vec::new();
		let mut chapters: Vec<Chapter> = Vec::new();
		
		let resp = reqwest::get(format!("{}{}{}", BASE_URI, API_MANGA_URI, id).as_str())?.text()?;
		let json: Value = serde_json::from_str(resp.as_str()).expect("Failed to read api respone");

		Ok(Manga {
			id: id,

			cover_url: json["manga"]["cover_url"].to_string(),
			description: json["manga"]["description"].to_string(),
			title: json["manga"]["title"].to_string(),
			artist: json["manga"]["artist"].to_string(),
			author: json["manga"]["author"].to_string(),

			status: Status::from_number(json["manga"]["status"].as_u64().expect("unable to unwrap `status` as u64") as u8),
			lang: Lang::from_str(json["manga"]["lang_flag"].as_str().expect("unable to unwrap `lang_flag` as &str")),
			hentai: json["manga"]["hentai"].as_u64().expect("unable to unwrap `hentai` as u64") != 0,
			
			genres: Self::extract_genres(&json["manga"]["genres"]),
			chapters: Self::extract_chapters_shallow(&json["chapter"])
		})
	}

	// Extract the genres from a api response
	fn extract_genres (json_array: &Value) -> Vec<Genres> {
		let mut genres: Vec<Genres> = Vec::new();
		for genre in json_array.as_array().unwrap().iter() { // Loop through the array and push to a vector after parsing it to an int
			genres.push(Genres::from_number( genre.as_u64().expect("unable to unwrap 'genre' as u64") as u8 ));
		}
		genres
	}

	// Extract the chapter info from the manga response not the chapter response
	fn extract_chapters_shallow (json_chapters: &Value) -> Vec<Chapter>{
		let map: Vec<(&String, &Value)> = json_chapters.as_object().unwrap().iter().collect();
		let mut chapters: Vec<Chapter> = Vec::new();

		for i in map.iter() {
			chapters.push(Chapter::from_shallow(i.0.parse::<u32>().expect("could not parse chapter id"), i.1));
		}

		chapters
	}

	// Fill in chapters with page data
	pub fn fill_chapters (&mut self) {
		for chap in self.chapters.iter_mut() {
			Chapter::fill_chapter(chap).expect("error filling chapters");
		}
	}

}