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

pub fn get_avatar_url(prefix: String, image_url: String, size: AvatarSize, webp: bool) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poster_urls() {
        let prefix = String::from("74");
        let image_url = String::from("74415673dcdc9cdd");
        assert_eq!(
            get_poster_url(
                prefix.clone(),
                image_url.clone(),
                PosterSize::W600x338,
                true
            ),
            "https://wsrv.nl/?url=https://simkl.in/posters/74/74415673dcdc9cdd_w.webp"
        );
        assert_eq!(
            get_poster_url(prefix.clone(), image_url.clone(), PosterSize::M340x, false),
            "https://wsrv.nl/?url=https://simkl.in/posters/74/74415673dcdc9cdd_m.jpg"
        );
        assert_eq!(
            get_poster_url(
                prefix.clone(),
                image_url.clone(),
                PosterSize::Ca190x279,
                true
            ),
            "https://wsrv.nl/?url=https://simkl.in/posters/74/74415673dcdc9cdd_ca.webp"
        );
        assert_eq!(
            get_poster_url(
                prefix.clone(),
                image_url.clone(),
                PosterSize::C170x250,
                true
            ),
            "https://wsrv.nl/?url=https://simkl.in/posters/74/74415673dcdc9cdd_c.webp"
        );
        assert_eq!(
            get_poster_url(
                prefix.clone(),
                image_url.clone(),
                PosterSize::Cm84x124,
                true
            ),
            "https://wsrv.nl/?url=https://simkl.in/posters/74/74415673dcdc9cdd_cm.webp"
        );
        assert_eq!(
            get_poster_url(prefix.clone(), image_url.clone(), PosterSize::S40x57, true),
            "https://wsrv.nl/?url=https://simkl.in/posters/74/74415673dcdc9cdd_s.webp"
        );
    }

    #[test]
    fn test_fanart_urls() {
        let prefix = String::from("71");
        let image_url = String::from("710408ec0a1bd207");
        assert_eq!(
            get_fanart_url(prefix.clone(), image_url.clone(), FanartSize::Darker, true),
            "https://wsrv.nl/?url=https://simkl.in/fanart/71/710408ec0a1bd207_d.webp"
        );
        assert_eq!(
            get_fanart_url(
                prefix.clone(),
                image_url.clone(),
                FanartSize::Medium1920x1080,
                true
            ),
            "https://wsrv.nl/?url=https://simkl.in/fanart/71/710408ec0a1bd207_medium.webp"
        );
        assert_eq!(
            get_fanart_url(
                prefix.clone(),
                image_url.clone(),
                FanartSize::Mobile950x540,
                true
            ),
            "https://wsrv.nl/?url=https://simkl.in/fanart/71/710408ec0a1bd207_mobile.webp"
        );
        assert_eq!(
            get_fanart_url(
                prefix.clone(),
                image_url.clone(),
                FanartSize::W600x338,
                true
            ),
            "https://wsrv.nl/?url=https://simkl.in/fanart/71/710408ec0a1bd207_w.webp"
        );
        assert_eq!(
            get_fanart_url(prefix.clone(), image_url.clone(), FanartSize::S48x27, true),
            "https://wsrv.nl/?url=https://simkl.in/fanart/71/710408ec0a1bd207_s48.webp"
        );
    }

    #[test]
    fn test_episode_urls() {
        let prefix = String::from("26");
        let image_url = String::from("265319260301d2ee2");
        assert_eq!(
            get_episodes_url(
                prefix.clone(),
                image_url.clone(),
                EpisodeSize::W600x338,
                true
            ),
            "https://wsrv.nl/?url=https://simkl.in/episodes/26/265319260301d2ee2_w.webp"
        );
        assert_eq!(
            get_episodes_url(
                prefix.clone(),
                image_url.clone(),
                EpisodeSize::C210x118,
                true
            ),
            "https://wsrv.nl/?url=https://simkl.in/episodes/26/265319260301d2ee2_c.webp"
        );
        assert_eq!(
            get_episodes_url(
                prefix.clone(),
                image_url.clone(),
                EpisodeSize::M112x63,
                true
            ),
            "https://wsrv.nl/?url=https://simkl.in/episodes/26/265319260301d2ee2_s.webp"
        );
    }

    #[test]
    fn test_avatar_urls() {
        let prefix = String::from("1");
        let image_url = String::from("1");
        assert_eq!(
            get_avatar_url(prefix.clone(), image_url.clone(), AvatarSize::Icon, true),
            "https://wsrv.nl/?url=https://simkl.in/episodes/1/1_24.webp"
        );
        assert_eq!(
            get_avatar_url(prefix.clone(), image_url.clone(), AvatarSize::Small, true),
            "https://wsrv.nl/?url=https://simkl.in/episodes/1/1_100.webp"
        );
        assert_eq!(
            get_avatar_url(prefix.clone(), image_url.clone(), AvatarSize::Normal, true),
            "https://wsrv.nl/?url=https://simkl.in/episodes/1/1.webp"
        );
        assert_eq!(
            get_avatar_url(prefix.clone(), image_url.clone(), AvatarSize::Big, true),
            "https://wsrv.nl/?url=https://simkl.in/episodes/1/1_200.webp"
        );
        assert_eq!(
            get_avatar_url(prefix.clone(), image_url.clone(), AvatarSize::VeryBig, true),
            "https://wsrv.nl/?url=https://simkl.in/episodes/1/1_256.webp"
        );
    }
}
