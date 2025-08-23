use serde::Deserialize;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum AnimeGenre {
    action,
    adventure,
    cars,
    comedy,
    dementia,
    demons,
    drama,
    ecchi,
    fantasy,
    game,
    harem,
    historical,
    horror,
    josei,
    kids,
    magic,
    martial_arts,
    mecha,
    military,
    music,
    mystery,
    parody,
    police,
    psychological,
    romance,
    samurai,
    school,
    sci_fi,
    seinen,
    shoujo,
    shoujo_ai,
    shounen,
    shounen_ai,
    slice_of_life,
    space,
    sports,
    super_power,
    supernatural,
    thriller,
    vampire,
    yaoi,
    yuri,
}

#[derive(Debug, Copy, Clone, PartialEq, Deserialize)]
#[repr(u8)]
pub enum AnimeType {
    #[serde(rename(deserialize = "tv"))]
    Tv,
    #[serde(rename(deserialize = "special"))]
    Special,
    #[serde(rename(deserialize = "ova"))]
    Ova,
    #[serde(rename(deserialize = "movie"))]
    Movie,
    // will be "music video"
    #[serde(rename(deserialize = "music video"))]
    MusicVideo,
    #[serde(rename(deserialize = "ona"))]
    Ona,
}
