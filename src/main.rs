#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

extern crate reqwest;

mod mangadex;

fn main() {
	mangadex::manga::Manga::new(27785).unwrap();
}