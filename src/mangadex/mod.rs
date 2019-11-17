// Public modules
pub mod manga;
pub mod chapter;
pub mod search;
pub mod user;
pub mod error;

// Private modules
mod enums;

// Constants for server URL's
const HTTPS_URI: &str = "https://";
const BASE_URL: &str = "mangadex.org/";

const API_MANGA_URL: &str = "api/manga/";
const API_CHAPTER_URL: &str = "api/chapter/";

// Trims the first and last character of a string
pub fn trim_first_last (text: & str) -> & str {
	&text[1..text.len()]
}