//! A list of constants and the Source enum used by the RustNAO library.
//! Constants are pulled from [here](https://saucenao.com/status.html).

pub const API_URL: &str = "https://saucenao.com/search.php";

/// A list of all available sources on SauceNAO.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Source {
    /// H-Magazines (index 0)
    HMagazines = 0,
    /// H-Game CG (index 2)
    HGameCG = 2,
    /// DoujinshiDB (index 3)
    DoujinshiDB = 3,
    /// Pixiv (index 5)
    Pixiv = 5,
    /// Nico Nico Seiga (index 8)
    NicoNicoSeiga = 8,
    /// Danbooru (index 9)
    Danbooru = 9,
    /// drawr Images (index 10)
    Drawr = 10,
    /// Nijie Images (index 11)
    Nijie = 11,
    /// Yand.ere (index 12)
    Yandere = 12,
    /// Shutterstock (index 15)
    Shutterstock = 15,
    /// Fakku (index 16)
    Fakku = 16,
    /// H-Misc (index 18)
    HMisc = 18,
    /// 2D-Market (index 19)
    TwoDMarket = 19,
    /// MediBang (index 20)
    MediBang = 20,
    /// Anime (index 21)
    Anime = 21,
    /// H-Anime (index 22)
    HAnime = 22,
    /// Movies (index 23)
    Movies = 23,
    /// Shows (index 24)
    Shows = 24,
    /// Gelbooru (index 25)
    Gelbooru = 25,
    /// Konachan (index 26)
    Konachan = 26,
    /// Sankaku Channel (index 27)
    SankakuChannel = 27,
    /// Anime-Pictures.net (index 28)
    AnimePicturesNet = 28,
    /// e621.net (index 29)
    E621Net = 29,
    /// Idol Complex (index 30)
    IdolComplex = 30,
    /// bcy.net Illust (index 31)
    BcyNetIllust = 31,
    /// bcy.net Cosplay (index 32)
    BcyNetCosplay = 32,
    /// PortalGraphics.net (index 33)
    PortalGraphicsNet = 33,
    /// deviantArt (index 34)
    DeviantArt = 34,
    /// Pawoo.net (index 35)
    PawooNet = 35,
    /// Madokami (index 36)
    Madokami = 36,
    /// MangaDex (index 37)
    MangaDex = 37,
    /// E-Hentai / H-Misc (index 38)
    EHentai = 38,
    /// ArtStation (index 39)
    ArtStation = 39,
    /// FurAffinity (index 40)
    FurAffinity = 40,
    /// Twitter (index 41)
    Twitter = 41,
    /// Furry Network (index 42)
    FurryNetwork = 42,
    /// Kemono (index 43)
    Kemono = 43,
    /// Skeb (index 44)
    Skeb = 44,
}

impl Source {
    /// Returns the name of the source as a string.
    pub fn name(&self) -> &'static str {
        match self {
            Source::HMagazines => "H-Magazines",
            Source::HGameCG => "H-Game CG",
            Source::DoujinshiDB => "DoujinshiDB",
            Source::Pixiv => "pixiv Images",
            Source::NicoNicoSeiga => "Nico Nico Seiga",
            Source::Danbooru => "Danbooru",
            Source::Drawr => "drawr Images",
            Source::Nijie => "Nijie Images",
            Source::Yandere => "Yande.re",
            Source::Shutterstock => "Shutterstock",
            Source::Fakku => "FAKKU",
            Source::HMisc => "H-Misc",
            Source::TwoDMarket => "2D-Market",
            Source::MediBang => "MediBang",
            Source::Anime => "Anime",
            Source::HAnime => "H-Anime",
            Source::Movies => "Movies",
            Source::Shows => "Shows",
            Source::Gelbooru => "Gelbooru",
            Source::Konachan => "Konachan",
            Source::SankakuChannel => "Sankaku Channel",
            Source::AnimePicturesNet => "Anime-Pictures.net",
            Source::E621Net => "e621.net",
            Source::IdolComplex => "Idol Complex",
            Source::BcyNetIllust => "bcy.net Illust",
            Source::BcyNetCosplay => "bcy.net Cosplay",
            Source::PortalGraphicsNet => "PortalGraphics.net",
            Source::DeviantArt => "deviantArt",
            Source::PawooNet => "Pawoo.net",
            Source::Madokami => "Madokami",
            Source::MangaDex => "MangaDex",
            Source::EHentai => "E-Hentai",
            Source::ArtStation => "ArtStation",
            Source::FurAffinity => "FurAffinity",
            Source::Twitter => "Twitter",
            Source::FurryNetwork => "Furry Network",
            Source::Kemono => "Kemono",
            Source::Skeb => "Skeb",
        }
    }

    /// Converts a u32 index to a Source enum.
    pub fn from_u32(index: u32) -> Option<Self> {
        match index {
            0 => Some(Source::HMagazines),
            2 => Some(Source::HGameCG),
            3 => Some(Source::DoujinshiDB),
            5 => Some(Source::Pixiv),
            8 => Some(Source::NicoNicoSeiga),
            9 => Some(Source::Danbooru),
            10 => Some(Source::Drawr),
            11 => Some(Source::Nijie),
            12 => Some(Source::Yandere),
            15 => Some(Source::Shutterstock),
            16 => Some(Source::Fakku),
            18 => Some(Source::HMisc),
            19 => Some(Source::TwoDMarket),
            20 => Some(Source::MediBang),
            21 => Some(Source::Anime),
            22 => Some(Source::HAnime),
            23 => Some(Source::Movies),
            24 => Some(Source::Shows),
            25 => Some(Source::Gelbooru),
            26 => Some(Source::Konachan),
            27 => Some(Source::SankakuChannel),
            28 => Some(Source::AnimePicturesNet),
            29 => Some(Source::E621Net),
            30 => Some(Source::IdolComplex),
            31 => Some(Source::BcyNetIllust),
            32 => Some(Source::BcyNetCosplay),
            33 => Some(Source::PortalGraphicsNet),
            34 => Some(Source::DeviantArt),
            35 => Some(Source::PawooNet),
            36 => Some(Source::Madokami),
            37 => Some(Source::MangaDex),
            38 => Some(Source::EHentai),
            39 => Some(Source::ArtStation),
            40 => Some(Source::FurAffinity),
            41 => Some(Source::Twitter),
            42 => Some(Source::FurryNetwork),
            43 => Some(Source::Kemono),
            44 => Some(Source::Skeb),
            _ => None,
        }
    }
}