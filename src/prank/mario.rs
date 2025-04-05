use anyhow::{anyhow, Result};
use rand::seq::IteratorRandom;
use rust_embed::RustEmbed;
use std::{collections::HashMap, sync::LazyLock};
use teloxide::types::InputFile;
#[derive(RustEmbed)]
#[folder = "assets/images/"]
struct Asset;

static IMAGES_MAP: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut map: HashMap<&'static str, &'static str> = HashMap::new();

    // Map jaime images to descriptions
    map.insert("jaime1.png", "Jaimito te ve mientras te tocas con amor 😏");
    map.insert("mario1.png", "Un Mario salvaje aparece 😱");
    map.insert("mario2.png", "Mario te saca la lengua 😜");
    map.insert("mario3.png", "Mario te mira con desprecio 😒");
    map.insert("mario4.png", "Mario se toma una cerveza 🍺");
    map.insert("mario5.png", "Tienes fuego amigo? 🔥");
    map.insert("mario6.png", "Te gustan los platanos? 🍌");

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
        .choose(&mut rand::rng())
        .ok_or_else(|| anyhow!("No images available"))?;

    let file = Asset::get(image).ok_or_else(|| anyhow!("Failed to load image: {}", image))?;

    Ok((description, InputFile::memory(file.data.into_owned())))
}
