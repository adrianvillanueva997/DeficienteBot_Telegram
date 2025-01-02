use std::{collections::HashSet, path::Path};
use walkdir::WalkDir;

#[derive(Debug)]
struct AssetValidator {
    images: HashSet<String>,
    videos: HashSet<String>,
    assets_dir: &'static Path,
}

impl AssetValidator {
    fn new() -> Self {
        let images = ["jaime1.png", "mario1.png", "mario2.png", "mario3.png",
                     "mario4.png", "mario5.png", "mario6.png"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let videos = ["viernes.mp4"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        Self {
            images,
            videos,
            assets_dir: Path::new("assets"),
        }
    }

    fn validate(&self) -> Result<(), String> {
        self.check_directory("images", &self.images)?;
        self.check_directory("videos", &self.videos)?;
        Ok(())
    }

    fn check_directory(&self, dir_name: &str, required: &HashSet<String>) -> Result<(), String> {
        let dir_path = self.assets_dir.join(dir_name);

        if !dir_path.exists() {
            return Err(format!("Directory {} does not exist", dir_path.display()));
        }

        let found_files: HashSet<String> = WalkDir::new(&dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter_map(|e| e.file_name().to_str().map(String::from))
            .collect();

        let missing: Vec<_> = required
            .difference(&found_files)
            .collect();

        if !missing.is_empty() {
            return Err(format!("Missing {} files: {:?}", dir_name, missing));
        }

        Ok(())
    }
}

fn main() {
    let validator = AssetValidator::new();
    if let Err(e) = validator.validate() {
        panic!("Asset validation failed: {}", e);
    }
    println!("cargo:rerun-if-changed=assets/");
}
