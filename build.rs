use std::path::Path;

fn main() {
    let images_dir = Path::new("assets/images");

    // List of required images
    let required_images = [
        "jaime1.png",
        "mario1.png",
        "mario2.png",
        "mario3.png",
        "mario4.png",
        "mario5.png",
        "mario6.png",
    ];

    for image in required_images {
        let path = images_dir.join(image);
        println!("Checking image: {:?}", path);
        if !path.exists() {
            panic!("Required image missing: {}", image);
        }
    }
}
