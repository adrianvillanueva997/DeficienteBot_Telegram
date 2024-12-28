use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use rand::seq::IteratorRandom;
use rust_embed::RustEmbed;
use std::collections::HashMap;
use teloxide::types::InputFile;
#[derive(RustEmbed)]
#[folder = "assets/images/"]
struct Asset;

static IMAGES_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map: HashMap<&'static str, &'static str> = HashMap::new();

    // Map jaime images to descriptions
    map.insert("jaime1.png", "Description for jaime1");
    map.insert("mario1.png", "Description for jaime2");
    map.insert("mario2.png", "Description for jaime2");
    map.insert("mario3.png", "Description for jaime2");
    map.insert("mario4.png", "Description for jaime2");
    map.insert("mario5.png", "Description for jaime2");
    map.insert("mario6.png", "Description for jaime2");

    // Verify files exist
    debug_assert!(Asset::get("jaime2.png").is_some());
    debug_assert!(Asset::get("mario2.png").is_some());
    debug_assert!(Asset::get("mario3.png").is_some());
    map
});

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn fetch_random_image() -> Result<(&'static str, InputFile)> {
    let (image, description) = IMAGES_MAP
        .iter()
        .choose(&mut rand::thread_rng())
        .ok_or_else(|| anyhow!("No images available"))?;

    let file = Asset::get(image).ok_or_else(|| anyhow!("Failed to load image: {}", image))?;

    Ok((description, InputFile::memory(file.data.into_owned())))
}
