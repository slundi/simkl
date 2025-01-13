//! When getting images, SIMKL asks to cache it in your app forever.
//! The images with the same URL never change. Do not redownload the same image multiple times.

pub const IMAGE_SERVER: &str = "https://wsrv.nl/?url=https://simkl.in/";

#[repr(u8)]
pub enum Type {
    Poster,
    Fanart,
    Episode,
    Avatars,
}

#[repr(u8)]
pub enum PosterSize {
    W600x338,
    M340x,
    Ca190x279,
    C170x250,
    Cm84x124,
    S40x57,
}

#[repr(u8)]
pub enum EpisodeSize {
    W600x338,
    C210x118,
    M112x63,
}

#[repr(u8)]
pub enum FanartSize {
    Darker,
    Medium1920x1080,
    Mobile950x540,
    W600x338,
    S48x27,
}

#[repr(u8)]
pub enum AvatarSize {
    /// 24x24
    Icon,
    /// 100x100
    Small,
    /// 200x200
    Normal,
    /// 256x256
    Big,
    /// 512
    VeryBig,
}

fn set_extension(mut url: String, webp: bool) -> String {
    url.push_str(if webp { ".webp" } else { ".jpg" });
    url
}

pub fn get_poster_url(prefix: String, image_url: String, size: PosterSize, webp: bool) -> String {
    let mut result = String::from(IMAGE_SERVER);
    result.push_str("posters/");
    result.push_str(&prefix);
    result.push('/');
    result.push_str(&image_url);
    match size {
        PosterSize::W600x338 => result.push_str("_w"),
        PosterSize::M340x => result.push_str("_m"),
        PosterSize::Ca190x279 => result.push_str("_ca"),
        PosterSize::C170x250 => result.push_str("_c"),
        PosterSize::Cm84x124 => result.push_str("_cm"),
        PosterSize::S40x57 => result.push_str("_s"),
    }
    set_extension(result, webp)
}

pub fn get_fanart_url(prefix: String, image_url: String, size: FanartSize, webp: bool) -> String {
    let mut result = String::from(IMAGE_SERVER);
    result.push_str("fanart/");
    result.push_str(&prefix);
    result.push('/');
    result.push_str(&image_url);
    match size {
        FanartSize::Darker => result.push_str("_d"),
        FanartSize::Medium1920x1080 => result.push_str("_medium"),
        FanartSize::Mobile950x540 => result.push_str("_mobile"),
        FanartSize::W600x338 => result.push_str("_w"),
        FanartSize::S48x27 => result.push_str("_s48"),
    }
    set_extension(result, webp)
}

pub fn get_episodes_url(
    prefix: String,
    image_url: String,
    size: EpisodeSize,
    webp: bool,
) -> String {
    let mut result = String::from(IMAGE_SERVER);
    result.push_str("episodes/");
    result.push_str(&prefix);
    result.push('/');
    result.push_str(&image_url);
    match size {
        EpisodeSize::W600x338 => result.push_str("_w"),
        EpisodeSize::C210x118 => result.push_str("_c"),
        EpisodeSize::M112x63 => result.push_str("_m"),
    }
    set_extension(result, webp)
}

pub fn get_avatars_url(prefix: String, image_url: String, size: AvatarSize, webp: bool) -> String {
    let mut result = String::from(IMAGE_SERVER);
    result.push_str("episodes/");
    result.push_str(&prefix);
    result.push('/');
    result.push_str(&image_url);
    match size {
        AvatarSize::Icon => result.push_str("_24"),
        AvatarSize::Small => result.push_str("_100"),
        AvatarSize::Normal => {}
        AvatarSize::Big => result.push_str("_256"),
        AvatarSize::VeryBig => result.push_str("_512"),
    }
    set_extension(result, webp)
}
