use crate::mangadex::{BASE_URI,API_CHAPTER_URI};
use crate::mangadex::enums::Lang;
use serde_json::{Value};

#[derive(Debug)]
pub struct Chapter {
	pub id: u32,
	pub title: String,
	pub timestamp: u64,
	pub volume: u8,
	pub chapter: u16,
	pub lang: Lang,
	pub groups: Vec<u16>,
	pub hash: Option<String>,
	pub comments: Option<u8>,
	pub pages: Option<Vec<String>>,
	pub long_strip: Option<bool>,
}

impl Chapter {
	// Just an empty place holder return constructor
	pub fn debug_empty (id: u32) -> Chapter {
		Chapter {
			id: 0,
			title: String::new(),
			timestamp: 0,
			
			volume: 0,
			chapter: 0,
			lang: Lang::JP,
			groups: vec![],

			hash: Some(String::new()),
			comments: Some(0),
			pages: Some(vec![]),
			long_strip: Some(false)
		}
	}

	// A lightweight contructor that used parameter to get info (does not get pages or similar info)
	pub fn from_shallow (id: u32, json: &Value) -> Chapter {
		
		// TODO make this figure out how many groups
		let mut group: Vec<u16> = Vec::new();
		group.push(json["group_id"].as_u64().expect("unable to unwrap `group_id` as u64") as u16);

		Chapter {
			id: id,
			title: json["title"].to_string(),
			timestamp: json["timestamp"].as_u64().expect("unable to unwrap `timestamp` as u64"),

			volume: json["volume"].as_str().expect("unable to get as str").parse::<u8>().expect("unable to unwrap `volume` as u64"),
			chapter: json["chapter"].as_str().expect("unable to get as str").parse::<u16>().expect("unable to unwrap `volume` as u64"),
			lang: Lang::from_str(json["lang_code"].as_str().expect("unable to unwrap `lang_code` as &str")),
			groups: group,

			pages: None,
			hash: None,
			comments: None,
			long_strip: None
		}
	}

	// Retrive data from the chapter api response
	pub fn from (id: u32) -> Result<Chapter, reqwest::Error> {
		
		let resp = reqwest::get(format!("{}{}{}", BASE_URI, API_CHAPTER_URI, id).as_str())?.text()?;
		let json: Value = serde_json::from_str(resp.as_str()).expect("Failed to read api respone");

		let mut chapter = Self::from_shallow(id, &json);

		Ok(Self::debug_empty(0))
	}


}