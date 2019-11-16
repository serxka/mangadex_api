#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

extern crate reqwest;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
const BASE_URI_DATA_S4: &str = "https://s4.mangadex.org/data/";

mod mangadex;

fn main() -> std::io::Result<()> {
	let mut onepunch = mangadex::manga::Manga::new(27785).unwrap();
	onepunch.fill_chapters();
	fs::create_dir("./skater")?;

	for chapter in onepunch.chapters.iter() {
		println!("{}", chapter.title);
		let chapter_title = format!("./skater/{}_{}-{}", chapter.title, chapter.volume, chapter.chapter);
		fs::create_dir(&chapter_title)?;
		for page in chapter.pages.as_ref().unwrap().iter() {
			println!("\t{}", page);
			let mut image_buf = reqwest::get(format!("{}{}/{}", BASE_URI_DATA_S4, chapter.hash.as_ref().unwrap(), page).as_str()).unwrap();
			let mut file = File::create(format!("{}/{}", chapter_title, page))?;
			let mut buf: Vec<u8> = vec![];
			image_buf.copy_to(&mut buf).unwrap();
			file.write_all(&buf)?;
		}
	}
	Ok(())
}