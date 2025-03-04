use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotifyAlbum {
    #[serde(rename = "album_type")]
    pub album_type: String,
    pub total_tracks: u32,  // Changed from i64 to u32
    pub available_markets: Vec<String>,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub restrictions: Option<Restrictions>,  // Made optional
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
    pub artists: Vec<Artist>,
    pub tracks: Tracks,
    pub copyrights: Vec<Copyright>,
    pub external_ids: ExternalIds,
    pub genres: Vec<String>,
    pub label: String,
    pub popularity: u8,  // Changed from i64 to u8
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub url: String,
    pub height: i64,
    pub width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Restrictions {
    pub reason: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tracks {
    pub href: String,
    pub limit: i64,
    pub next: Option<String>,  // Made optional
    pub offset: i64,
    pub previous: Option<String>,  // Made optional
    pub total: i64,
    pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub artists: Vec<Artist>,
    pub available_markets: Vec<String>,
    pub disc_number: i64,
    pub duration_ms: i64,
    pub explicit: bool,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub is_playable: Option<bool>,  // Some tracks may not have this
    pub linked_from: Option<LinkedFrom>,  // Made optional
    pub restrictions: Option<Restrictions>,  // Made optional
    pub name: String,
    pub preview_url: Option<String>,  // Made optional
    pub track_number: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
    pub is_local: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkedFrom {
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Copyright {
    pub text: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalIds {
    pub isrc: Option<String>,  // Some tracks may not have these
    pub ean: Option<String>,
    pub upc: Option<String>,
}
