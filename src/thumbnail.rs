use image::DynamicImage;
use crate::generic::GenericThumbnail;

pub struct StaticThumbnail {
    image: DynamicImage,
}

impl StaticThumbnail {
    pub fn as_dyn(&self) -> &DynamicImage {
        &self.image
    }
}

