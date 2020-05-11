use image::{DynamicImage, GenericImageView};
use std::fmt;
use std::fmt::Formatter;
use std::path::PathBuf;

/// The `StaticThumbnail` type.
///
/// This type is a non modifiable image. No operations can be performed on it.
/// It is used in certain operations as an argument itself (e.g. the combine operation).
#[derive(Clone)]
pub struct StaticThumbnail {
    /// The path from which this image originates from
    src_path: PathBuf,
    /// The actual image data
    image: DynamicImage,
}

impl fmt::Debug for StaticThumbnail {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "StaticThumbnail {{ {:?}, DynamicImage}}", self.src_path)
    }
}

impl StaticThumbnail {
    /// Constructs a new `StaticThumbnail`from a path and image data
    ///
    /// * src_path: PathBuf - The origin path of the image
    /// * image: DynamicImage - The actual image data
    pub fn new(src_path: PathBuf, image: DynamicImage) -> Self {
        StaticThumbnail { src_path, image }
    }

    /// Gets the actual image data
    pub fn as_dyn(&self) -> &DynamicImage {
        &self.image
    }

    /// Gets dimensions of the image data
    pub fn dimensions(&self) -> (u32, u32) {
        self.as_dyn().dimensions()
    }

    /// Gets the stored origin path of the image
    pub fn get_src_path(&self) -> PathBuf {
        self.src_path.clone()
    }
}
