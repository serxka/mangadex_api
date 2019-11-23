use crate::error::Error;

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
	Ecchi = 9,
	Fantasy = 10,
	Gyaru = 11,
	Harem = 12,
	Historical = 13,
	Horror = 14,
	MartialArts = 16,
	Mecha = 17,
	Medical = 18,
	Music = 19,
	Mystery = 20,
	Oneshot = 21,
	Psychological = 22,
	Romance = 23,
	SchoolLife = 24,
	SciFi = 25,
	ShoujoAi = 28,
	ShounenAi = 30,
	SliceOfLife = 31,
	Smut = 32,
	Sports = 33,
	Supernatural = 34,
	Tragedy = 35,
	LongStrip = 36,
	Yaoi = 37,
	Yuri = 38,
	VideoGames = 40,
	Isekai = 41,
	Adaptation = 42,
	Anthology = 43,
	WebComic = 44,
	FullColor = 45,
	UserCreated = 46,
	OfficialCreated = 47,
	FanColored = 48,
	Gore = 49,
	SexualViolence = 50,
	Crime = 51,
	MagicalGirls = 52,
	Philosophical = 53,
	Superhero = 54,
	Thriller = 55,
	Wuxia = 56,
	Aliens = 57,
	Animals = 58,
	Crossdressing = 59,
	Demons = 60,
	Delinquents = 61,
	Genderswap = 62,
	Ghosts = 63,
	MonsterGirls = 64,
	Loli = 65,
	Magic = 66,
	Military = 67,
	Monsters = 68,
	Ninja = 69,
	OfficeWorkers = 70,
	Police = 71,
	PostApocalyptic = 72,
	Reincarnation = 73,
	ReverseHarem = 74,
	Samurai = 75,
	Shota = 76,
	Survival = 77,
	TimeTravel = 78,
	Vampires = 79,
	TraditionalGames = 80,
	VirtualReality = 81,
	Zombies = 82,
	Incest = 83,
}

#[derive(Debug)]
pub enum Lang {
	Jp,
	Gb,
	Kr,
	It,
	Fr,
	Br,
	Undefined,
}

// Done
#[derive(Debug)]
pub enum Status {
	Ongoing = 1,
	Completed = 2,
	Cancelled = 3,
	Haitus = 4,
	Undefined,
}

// Done
#[derive(Debug)]
pub enum Server {
	Server(u8),
	None
}

impl Genres {
	pub fn _to_str(&self) -> &str {
		use Genres::*;
		match *self { // test to see if a hash table is faster
			FourKoma => "4-Koma",
			Action => "Action",
			Adventure => "Adventure",
			AwardWinning => "Award Winning",
			Comedy => "Comedy",
			Cooking => "Cooking",
			Doujinshi => "Doujinshi",
			Drama => "Drama",
			SliceOfLife => "Slice Of Life",
			Sports => "Sports",
			_ => "INVALID"
		}
	}
	pub fn from_number(index: u8) -> Result<Genres, Error> {
		use Genres::*;
		match index {
			1 => Ok(FourKoma),
			2 => Ok(Action),
			3 => Ok(Adventure),
			4 => Ok(AwardWinning),
			5 => Ok(Comedy),
			6 => Ok(Cooking),
			7 => Ok(Doujinshi),
			8 => Ok(Drama),
			9 => Ok(Ecchi),
			10 => Ok(Fantasy),
			11 => Ok(Gyaru),
			12 => Ok(Harem),
			13 => Ok(Historical),
			14 => Ok(Horror),
			16 => Ok(MartialArts),
			17 => Ok(Mecha),
			18 => Ok(Medical),
			19 => Ok(Music),
			20 => Ok(Mystery),
			21 => Ok(Oneshot),
			22 => Ok(Psychological),
			23 => Ok(Romance),
			24 => Ok(SchoolLife),
			25 => Ok(SciFi),
			28 => Ok(ShoujoAi),
			30 => Ok(ShounenAi),
			31 => Ok(SliceOfLife),
			32 => Ok(Smut),
			33 => Ok(Sports),
			34 => Ok(Supernatural),
			35 => Ok(Sports),
			36 => Ok(LongStrip),
			37 => Ok(Yaoi),
			38 => Ok(Yuri),
			40 => Ok(VideoGames),
			41 => Ok(Isekai),
			42 => Ok(Adaptation),
			43 => Ok(Anthology),
			44 => Ok(WebComic),
			45 => Ok(FullColor),
			46 => Ok(UserCreated),
			47 => Ok(OfficialCreated),
			48 => Ok(FanColored),
			49 => Ok(Gore),
			50 => Ok(SexualViolence),
			51 => Ok(Crime),
			52 => Ok(MagicalGirls),
			53 => Ok(Philosophical),
			54 => Ok(Superhero),
			55 => Ok(Thriller),
			56 => Ok(Wuxia),
			57 => Ok(Aliens),
			58 => Ok(Animals),
			59 => Ok(Crossdressing),
			60 => Ok(Demons),
			61 => Ok(Delinquents),
			62 => Ok(Genderswap),
			63 => Ok(Ghosts),
			64 => Ok(MonsterGirls),
			65 => Ok(Loli),
			66 => Ok(Magic),
			67 => Ok(Military),
			68 => Ok(Monsters),
			69 => Ok(Ninja),
			70 => Ok(OfficeWorkers),
			71 => Ok(Police),
			72 => Ok(PostApocalyptic),
			73 => Ok(Reincarnation),
			74 => Ok(ReverseHarem),
			75 => Ok(Samurai),
			76 => Ok(Shota),
			77 => Ok(Survival),
			78 => Ok(TimeTravel),
			79 => Ok(Vampires),
			80 => Ok(TraditionalGames),
			81 => Ok(VirtualReality),
			82 => Ok(Zombies),
			83 => Ok(Incest),
			//_ => Err(Error::Parse)
			_ => panic!("missed one dipshit")
		}
	}
}

impl Lang {
	pub fn to_str (&self) -> &str {
		match *self {
			Lang::Jp => "JP",
			Lang::Gb => "EN",
			Lang::Kr => "KR",
			Lang::It => "IT",
			Lang::Fr => "FR",
			Lang::Br => "BR",
			Lang::Undefined => "Undefined"
		}
	}
	pub fn to_str_full (&self) -> &str {
		match *self {
			Lang::Jp => "Japanese",
			Lang::Gb => "English",
			Lang::Kr => "Korean",
			Lang::It => "Italian",
			Lang::Fr => "French",
			Lang::Br => "Portuguese",
			Lang::Undefined => "Undefined"
		}
	}
	pub fn from_str (lang: &str) -> Result<Lang, Error> {
		match lang {
			"jp" => Ok(Lang::Jp),
			"gb" => Ok(Lang::Gb),
			"kr" => Ok(Lang::Kr),
			"it" => Ok(Lang::It),
			"fr" => Ok(Lang::Fr),
			"br" => Ok(Lang::Br),
			_ => Err(Error::Parse)
		}
	}
}

// Done
impl Status {
	pub fn to_str(&self) -> &str {
		match *self {
			Status::Ongoing => "Ongoing",
			Status::Completed => "Completed",
			Status::Cancelled => "Cancelled",
			Status::Haitus => "Haitus",
			Status::Undefined => "Undefined"
		}
	}
	pub fn from_number (status: u8) -> Result<Status, Error> {
		match status {
			1 => Ok(Status::Ongoing),
			2 => Ok(Status::Completed),
			3 => Ok(Status::Cancelled),
			4 => Ok(Status::Haitus),
			_ => Err(Error::Parse)
		}
	}
}

// Done
impl Server {
	pub fn to_str (&self) -> Result<String, Error> {
		match *self {
			Server::Server(s) => Ok(format!("s{}.", s)), // There has to be a better way
			Server::None => Err(Error::UnknownValue)
		}
	}
	// This is finicky, but I think its the fastest way?
	pub fn from_str (server: &str) -> Result<Server, Error> {
		let num: u8 = server[9..10].parse().expect("oh shit wtf, @from_str in enum Server");
		Ok(Server::Server(num))
	}
}