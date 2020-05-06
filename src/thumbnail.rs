use image::DynamicImage;
use crate::generic::GenericThumbnail;
use std::path::Path;
use crate::thumbnail::operations::Operation;

mod operations;

pub struct StaticThumbnail {
    image: DynamicImage,
}

impl StaticThumbnail {
    pub fn as_dyn(&self) -> &DynamicImage {
        &self.image
    }
}

pub trait SingleThumbnail : GenericThumbnail {
    fn as_static_copy(&self) -> &mut StaticThumbnail;
}

pub struct Thumbnail<'a> {
    path: &'a Path,
    height: u32,
    width: u32,
    image: Option<DynamicImage>,
    ops: Vec<&'a dyn Operation>
}


