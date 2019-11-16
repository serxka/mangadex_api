#[derive(Debug)]
pub enum Genres {
	FourKoma = 1,
	Action = 2,
	Adventure = 3,
	AwardWinning = 4,
	Comedy = 5,
	Cooking = 6,
	Doujinshi = 7,
	Drama = 8,
	SliceOfLife = 31,
	Sports = 33,
}
#[derive(Debug)]
pub enum Lang {
	JP,
	GB,
}
#[derive(Debug)]
pub enum Status {
	Ongoing = 1,
	Completed = 2,
	Cancelled = 3,
	Haitus = 4,
}

impl Genres {
	pub fn as_str(&self) -> &str {
		match self {
			&Genres::FourKoma => "4-Koma",
			&Genres::SliceOfLife => "Slice Of Life",
			&Genres::Sports => "Sports",
			_ => "INVALID"
		}
	}
	pub fn from_number(index: u8) -> Genres {
		match index {
			1 => Genres::FourKoma,
			31 => Genres::SliceOfLife,
			35 => Genres::Sports,
			_ => Genres::FourKoma
		}
	}
}

impl Lang {
	pub fn as_str (&self) -> &str {
		match self {
			&Lang::JP => "JP",
			&Lang::GB => "EN"
		}
	}
	pub fn as_full_str (&self) -> &str {
		match self {
			&Lang::JP => "Japanese",
			&Lang::GB => "English"
		}
	}
	pub fn from_str (lang: &str) -> Lang {
		match lang {
			"jp" => Lang::JP,
			"gb" => Lang::GB,
			_ => panic!("Unknown lang code")
		}
	}
}

impl Status {
	pub fn as_str(&self) -> &str {
		match self {
			&Status::Ongoing => "Ongoing",
			&Status::Completed => "Completed",
			&Status::Cancelled => "Cancelled",
			&Status::Haitus => "Haitus",
		}
	}
	pub fn from_number (status: u8) -> Status{
		match status {
			1 => Status::Ongoing,
			2 => Status::Completed,
			3 => Status::Cancelled,
			4 => Status::Haitus,
			_ => panic!("Unknown status number")
		}
	}
}