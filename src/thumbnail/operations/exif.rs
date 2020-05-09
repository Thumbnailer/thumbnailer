use crate::thumbnail::operations::Operation;
use crate::Exif;
use image::DynamicImage;

#[derive(Debug, Clone)]
pub struct ExifOp {
    metadata: Exif,
}

impl ExifOp {
    pub fn new(metadata: Exif) -> Self {
        ExifOp { metadata }
    }
}

impl Operation for ExifOp {
    fn apply(&self, image: &mut DynamicImage) -> bool
    where
        Self: Sized,
    {
        unimplemented!()
    }
}
