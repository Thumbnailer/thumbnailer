use crate::thumbnail::Thumbnail;
use crate::generic::{Resize, ResampleFilter, Crop, Orientation, Exif, BoxPosition};
use crate::StaticThumbnail;
use image::imageops::FilterType;

pub trait Operation{
    fn apply(&self, image: &mut Thumbnail) -> bool where Self : Sized;
}

pub(crate) struct ResizeOp {
    size: Resize,
    filter: Option<ResampleFilter>,
}

impl Operation for ResizeOp {
    fn apply(&self, image: &mut Thumbnail) -> bool {
        let dynamic_image = match &image.image {
            Some(dyn_img) => dyn_img,
            None => return false,
        };
    
        let aspect_ratio = match dynamic_image.as_rgb8() {
            Some(rgb_image) => rgb_image.width() as f32 / rgb_image.height() as f32,
            _ => return false,
        };
    
        let filter_type = match self.filter {
            Some(ResampleFilter::Nearest) => Some(FilterType::Nearest),
            Some(ResampleFilter::Triangle) => Some(FilterType::Triangle),
            Some(ResampleFilter::CatmullRom) => Some(FilterType::CatmullRom),
            Some(ResampleFilter::Gaussian) => Some(FilterType::Gaussian),
            Some(ResampleFilter::Lanczos3) => Some(FilterType::Lanczos3),
            None => None,
        };
    
        match filter_type {
            Some(image_filter) => {
                match self.size {
                    Resize::Height(y) => {
                        let x: u32 = (aspect_ratio * y as f32) as u32 + 1;
                        image.image = Some(dynamic_image.resize(x, y, image_filter));
                    },
                    Resize::Width(x) => {
                        let y: u32 = (x as f32 / aspect_ratio) as u32 + 1;
                        image.image = Some(dynamic_image.resize(x, y, image_filter));
                    },
                    Resize::BoundingBox(x, y) => {
                        image.image = Some(dynamic_image.resize(x, y, image_filter));
                    },
                    Resize::ExactBox(x, y) => {
                        image.image = Some(dynamic_image.resize_exact(x, y, image_filter));
                    },
                };
            },
            None => {
                match self.size {
                    Resize::Height(y) => {
                        let x: u32 = (aspect_ratio * y as f32) as u32 + 1;
                        image.image = Some(dynamic_image.thumbnail(x, y));
                    },
                    Resize::Width(x) => {
                        let y: u32 = (x as f32 / aspect_ratio) as u32 + 1;
                        image.image = Some(dynamic_image.thumbnail(x, y));
                    },
                    Resize::BoundingBox(x, y) => {
                        image.image = Some(dynamic_image.thumbnail(x, y));
                    },
                    Resize::ExactBox(x, y) => {
                        image.image = Some(dynamic_image.thumbnail_exact(x, y));
                    },
                };
            },
        };

        true
    }
}

pub(crate) struct CropOp {
    crop: Crop,
}

impl Operation for CropOp {
    fn apply(&self, image: &mut Thumbnail) -> bool where Self: Sized {
        let dynamic_image = match &image.image {
            Some(dyn_img) => dyn_img,
            None => return false,
        };
    }
    
    match self.crop {
        Box(x, y, w, h) => {
            image.image = Some(dynamic_image.crop(x, y, w, h));
        },
        Ratio(w_r, h_r) => {
            let ratio_old = image.width / image.height;
            let ratio_new = w_r / h_r;

            if ratio_old <= ratio_new {
                let height_new = (ratio_old / ratio_new) * image.height;
                let y_new = (image.height - height_new) / 2;

                image.image = Some(dynamic_image.crop(0, y_new, image.width, height_new));
            }
            else {
                let width_new = (ratio_new / ratio_old) * image.width;
                let x_new = (image.width - width_new) / 2;

                image.image = Some(dynamic_image.crop(x_new, 0, width_new, image.height));
            }
        },
    }

    true
}

pub(crate) struct BlurOp {
    sigma: f32,
}

impl Operation for BlurOp {
    fn apply(&self, image: &mut Thumbnail) -> bool where Self: Sized {
        let dynamic_image = match &image.image {
            Some(dyn_img) => dyn_img,
            None => return false,
        };

        image.image = Some(dynamic_image.blur(self.sigma));
        true
    }
}

pub(crate) struct BrightenOp {
    value: i32,
}

impl Operation for BrightenOp {
    fn apply(&self, image: &mut Thumbnail) -> bool where Self: Sized {
        let dynamic_image = match &image.image {
            Some(dyn_img) => dyn_img,
            None => return false,
        };

        image.image = Some(dynamic_image.brighten(self.value));
        true
    }
}

pub(crate) struct HuerotateOp {
    degree: i32,
}

impl Operation for HuerotateOp {
    fn apply(&self, image: &mut Thumbnail) -> bool where Self: Sized {
        let dynamic_image = match &image.image {
            Some(dyn_img) => dyn_img,
            None => return false,
        };

        image.image = Some(dynamic_image.huerotate(self.degree));
        true
    }
}

pub(crate) struct ContrastOp {
    value: f32,
}

impl Operation for ContrastOp {
    fn apply(&self, image: &mut Thumbnail) -> bool where Self: Sized {
        unimplemented!()
    }
}

pub(crate) struct FlipOp {
    orientation: Orientation,
}

impl Operation for FlipOp {
    fn apply(&self, image: &mut Thumbnail) -> bool where Self: Sized {
        unimplemented!()
    }
}

pub(crate) struct InvertOp {
}

impl Operation for InvertOp {
    fn apply(&self, image: &mut Thumbnail) -> bool where Self: Sized {
        unimplemented!()
    }
}

pub(crate) struct ExifOp {
    metadata: Exif,
}

impl Operation for ExifOp {
    fn apply(&self, image: &mut Thumbnail) -> bool where Self: Sized {
        unimplemented!()
    }
}

pub(crate) struct TextOp {
    text: String,
    pos: BoxPosition,
}

impl Operation for TextOp {
    fn apply(&self, image: &mut Thumbnail) -> bool where Self: Sized {
        unimplemented!()
    }
}

pub(crate) struct CombineOp<'a> {
    image: &'a StaticThumbnail,
    pos: BoxPosition
}

impl Operation for CombineOp<'_> {
    fn apply(&self, image: &mut Thumbnail) -> bool where Self: Sized {
        unimplemented!()
    }
}