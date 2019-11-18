use std::error::Error as StdError;
use std::io::Error as IoError;
use reqwest::Error as ReqwestError;
use std::fmt;

use self::Error::*;

#[derive(Debug)]
pub enum Error {
	Parse,
	UnknownValue,
	NotFound,
	Io(IoError),
	Reqwest(ReqwestError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {	
            Io(ref e) => fmt::Display::fmt(e, f),
            Reqwest(ref e) => fmt::Display::fmt(e, f),
			ref e => f.write_str(e.description()),
        }
    }
}

impl StdError for Error {
	fn description(&self) -> &str {
		match *self {
			Parse => "Error parsing value, (might be Lang)",
			UnknownValue => "Unknown value, could not convert",
			NotFound => "Could not be found",
			Io(ref e) => e.description(),
			Reqwest(ref e) => e.description(),
		}
	}

	fn cause(&self) -> Option<&dyn StdError> {
		match *self {
			Io(ref error) => Some(error),
			Reqwest(ref error) => Some(error),
			_ => None,
		}
	}
}

impl From<ReqwestError> for Error {
    fn from(err: ReqwestError) -> Error {
        Reqwest(err)
    }
}