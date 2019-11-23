#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]
// Public export
pub use manga::Manga;
pub use chapter::Chapter;
pub use session::Session;
pub use self::error::Error;

// Modules
mod manga;
mod chapter;
mod search;
mod session;
mod enums;
mod error;

// Constants for server URL's
const HTTPS_URI: &str = "https://";
const BASE_URL: &str = "mangadex.org/";

const API_MANGA_URL: &str = "api/manga/";
const API_CHAPTER_URL: &str = "api/chapter/";

// Rexports

// Trims the first and last character of a string
pub fn trim_first_last (text: & str) -> & str {
	&text[1..text.len()]
}