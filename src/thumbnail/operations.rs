use crate::thumbnail::Thumbnail;
use crate::generic::{Resize, ResampleFilter, Crop, Orientation, Exif, BoxPosition};
use crate::StaticThumbnail;

pub trait Operation{
    fn apply(&self, image: &Thumbnail) -> bool where Self : Sized;
}

pub(crate) struct ResizeOp {
    size: Resize,
    filter: ResampleFilter
}

impl Operation for ResizeOp {
    fn apply(&self, image: &Thumbnail) -> bool {
        unimplemented!()
    }
}

pub(crate) struct CropOp {
    crop: Crop,
}

impl Operation for CropOp {
    fn apply(&self, image: &Thumbnail) -> bool where Self: Sized {
        unimplemented!()
    }
}

pub(crate) struct BlurOp {
    sigma: f32,
}

impl Operation for BlurOp {
    fn apply(&self, image: &Thumbnail) -> bool where Self: Sized {
        unimplemented!()
    }
}

pub(crate) struct BrightenOp {
    value: i32,
}

impl Operation for BrightenOp {
    fn apply(&self, image: &Thumbnail) -> bool where Self: Sized {
        unimplemented!()
    }
}

pub(crate) struct HuerotateOp {
    degree: i32,
}

impl Operation for HuerotateOp {
    fn apply(&self, image: &Thumbnail) -> bool where Self: Sized {
        unimplemented!()
    }
}

pub(crate) struct ContrastOp {
    value: f32,
}

impl Operation for ContrastOp {
    fn apply(&self, image: &Thumbnail) -> bool where Self: Sized {
        unimplemented!()
    }
}

pub(crate) struct FlipOp {
    orientation: Orientation,
}

impl Operation for FlipOp {
    fn apply(&self, image: &Thumbnail) -> bool where Self: Sized {
        unimplemented!()
    }
}

pub(crate) struct InvertOp {
}

impl Operation for InvertOp {
    fn apply(&self, image: &Thumbnail) -> bool where Self: Sized {
        unimplemented!()
    }
}

pub(crate) struct ExifOp {
    metadata: Exif,
}

impl Operation for ExifOp {
    fn apply(&self, image: &Thumbnail) -> bool where Self: Sized {
        unimplemented!()
    }
}

pub(crate) struct TextOp {
    text: String,
    pos: BoxPosition,
}

impl Operation for TextOp {
    fn apply(&self, image: &Thumbnail) -> bool where Self: Sized {
        unimplemented!()
    }
}

pub(crate) struct CombineOp<'a> {
    image: &'a StaticThumbnail,
    pos: BoxPosition
}

impl Operation for CombineOp<'_> {
    fn apply(&self, image: &Thumbnail) -> bool where Self: Sized {
        unimplemented!()
    }
}