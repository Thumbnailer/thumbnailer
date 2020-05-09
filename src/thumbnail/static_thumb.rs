use image::DynamicImage;
use std::fmt;
use std::fmt::Formatter;
use std::path::PathBuf;

#[derive(Clone)]
pub struct StaticThumbnail {
    src_path: PathBuf,
    image: DynamicImage,
}

impl fmt::Debug for StaticThumbnail {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "StaticThumbnail {{ {:?}, DynamicImage}}", self.src_path)
    }
}

impl StaticThumbnail {
    pub fn new(src_path: PathBuf, image: DynamicImage) -> Self {
        StaticThumbnail { src_path, image }
    }

    pub fn as_dyn(&self) -> &DynamicImage {
        &self.image
    }

    pub fn get_width(&self) -> u32 {
        match self.as_dyn().as_rgb8() {
            Some(rgb_image) => rgb_image.width(),
            None => 0,
        }
    }

    pub fn get_height(&self) -> u32 {
        match self.as_dyn().as_rgb8() {
            Some(rgb_image) => rgb_image.height(),
            None => 0,
        }
    }
    pub fn get_src_path(&self) -> PathBuf {
        self.src_path.clone()
    }
}
