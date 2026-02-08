//! A list of constants used by the RustNAO library.
//! Constants are pulled from [here](https://saucenao.com/status.html).

pub const API_URL: &str = "https://saucenao.com/search.php";

/// A struct to represent a source.
#[derive(Clone)]
pub struct Source<'a> {
    /// The index of the source.
    pub index: u32,
    /// The name of the source.
    pub name: &'a str,
}

/// Constant for H-Magazines.
pub const H_MAGAZINES: Source<'static> = Source {
    index: 0,
    name: "H-Magazines",
};

/// Constant for H-Game CG.
pub const H_GAME_CG: Source<'static> = Source {
    index: 2,
    name: "H-Game CG",
};

/// Constant for DoujinshiDB.
pub const DOUJINSHI_DB: Source<'static> = Source {
    index: 3,
    name: "DoujinshiDB",
};

/// Constant for Pixiv.
pub const PIXIV: Source<'static> = Source {
    index: 5,
    name: "Pixiv",
};

/// Constant for Nico Nico Seiga.
pub const NICO_NICO_SEIGA: Source<'static> = Source {
    index: 8,
    name: "Nico Nico Seiga",
};

/// Constant for Danbooru.
pub const DANBOORU: Source<'static> = Source {
    index: 9,
    name: "Danbooru",
};

/// Constant for drawr Images.
pub const DRAWR: Source<'static> = Source {
    index: 10,
    name: "drawr Images",
};

/// Constant for Nijie Images.
pub const NIJIE: Source<'static> = Source {
    index: 11,
    name: "Nijie Images",
};

/// Constant for Yande.re.
pub const YANDE_RE: Source<'static> = Source {
    index: 12,
    name: "Yande.re",
};

/// Constant for Shutterstock.
pub const SHUTTERSTOCK: Source<'static> = Source {
    index: 15,
    name: "Shutterstock",
};

/// Constant for Fakku.
pub const FAKKU: Source<'static> = Source {
    index: 16,
    name: "FAKKU",
};

/// Constant for H-Misc.
pub const H_MISC: Source<'static> = Source {
    index: 18,
    name: "H-Misc",
};

/// Constant for 2D-Market.
pub const TWO_D_MARKET: Source<'static> = Source {
    index: 19,
    name: "2D-Market",
};

/// Constant for MediBang.
pub const MEDIBANG: Source<'static> = Source {
    index: 20,
    name: "MediBang",
};

/// Constant for Anime.
pub const ANIME: Source<'static> = Source {
    index: 21,
    name: "Anime",
};

/// Constant for H-Anime.
pub const H_ANIME: Source<'static> = Source {
    index: 22,
    name: "H-Anime ",
};

/// Constant for Movies.
pub const MOVIES: Source<'static> = Source {
    index: 23,
    name: "Movies",
};

/// Constant for Shows.
pub const SHOWS: Source<'static> = Source {
    index: 24,
    name: "Shows",
};

/// Constant for Gelbooru.
pub const GELBOORU: Source<'static> = Source {
    index: 25,
    name: "Gelbooru",
};

/// Constant for Konachan.
pub const KONACHAN: Source<'static> = Source {
    index: 26,
    name: "Konachan",
};

/// Constant for Sankaku Channel.
pub const SANKAKU_CHANNEL: Source<'static> = Source {
    index: 27,
    name: "Sankaku Channel",
};

/// Constant for Anime-Pictures.net.
pub const ANIME_PICTURES_NET: Source<'static> = Source {
    index: 28,
    name: "Anime-Pictures.net",
};

/// Constant for e621.net.
pub const E621_NET: Source<'static> = Source {
    index: 29,
    name: "e621.net",
};

/// Constant for Idol Complex.
pub const IDOL_COMPLEX: Source<'static> = Source {
    index: 30,
    name: "Idol Complex",
};

/// Constant for bcy.net Illust.
pub const BCY_NET_ILLUST: Source<'static> = Source {
    index: 31,
    name: "bcy.net Illust",
};

/// Constant for bcy.net Cosplay.
pub const BCY_NET_COSPLAY: Source<'static> = Source {
    index: 32,
    name: "bcy.net Cosplay",
};

/// Constant for PortalGraphics.net.
pub const PORTALGRAPHICS_NET: Source<'static> = Source {
    index: 33,
    name: "PortalGraphics.net",
};

/// Constant for deviantArt.
pub const DEVIANTART: Source<'static> = Source {
    index: 34,
    name: "deviantArt",
};

/// Constant for Pawoo.net.
pub const PAWOO_NET: Source<'static> = Source {
    index: 35,
    name: "Pawoo.net",
};

/// Constant for Madokami.
pub const MADOKAMI: Source<'static> = Source {
    index: 36,
    name: "Madokami",
};

/// Constant for MangaDex.
pub const MANGADEX: Source<'static> = Source {
    index: 37,
    name: "MangaDex",
};

/// Constant for E-Hentai.
pub const EHENTAI: Source<'static> = Source {
    index: 38,
    name: "E-Hentai",
};

/// List of all available sources.
pub const LIST_OF_SOURCES: [Source; 32] = [
    H_MAGAZINES,
    H_GAME_CG,
    DOUJINSHI_DB,
    PIXIV,
    NICO_NICO_SEIGA,
    DANBOORU,
    DRAWR,
    NIJIE,
    YANDE_RE,
    SHUTTERSTOCK,
    FAKKU,
    H_MISC,
    TWO_D_MARKET,
    MEDIBANG,
    ANIME,
    H_ANIME,
    MOVIES,
    SHOWS,
    GELBOORU,
    KONACHAN,
    SANKAKU_CHANNEL,
    ANIME_PICTURES_NET,
    E621_NET,
    IDOL_COMPLEX,
    BCY_NET_ILLUST,
    BCY_NET_COSPLAY,
    PORTALGRAPHICS_NET,
    DEVIANTART,
    PAWOO_NET,
    MADOKAMI,
    MANGADEX,
    EHENTAI,
];