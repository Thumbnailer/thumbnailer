use crate::generic::{BoxPosition, Crop, Exif, Orientation, ResampleFilter, Resize};
use crate::thumbnail::{ImageData, Thumbnail};
use crate::StaticThumbnail;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView};

pub trait Operation: OperationClone {
    fn apply(&self, image: &mut DynamicImage) -> bool;
}

pub trait OperationClone {
    fn box_clone(&self) -> Box<dyn Operation>;
}

impl<T> OperationClone for T
where
    T: 'static + Operation + Clone,
{
    fn box_clone(&self) -> Box<dyn Operation> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Operation> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

#[derive(Copy, Clone)]
pub(crate) struct ResizeOp {
    size: Resize,
    filter: Option<ResampleFilter>,
}

impl ResizeOp {
    pub fn new(size: Resize, filter: Option<ResampleFilter>) -> Self {
        ResizeOp { size, filter }
    }
}

impl Operation for ResizeOp {
    fn apply(&self, image: &mut DynamicImage) -> bool {
        let aspect_ratio = match image.as_rgb8() {
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
                        *image = image.resize(x, y, image_filter);
                    }
                    Resize::Width(x) => {
                        let y: u32 = (x as f32 / aspect_ratio) as u32 + 1;
                        *image = image.resize(x, y, image_filter);
                    }
                    Resize::BoundingBox(x, y) => {
                        *image = image.resize(x, y, image_filter);
                    }
                    Resize::ExactBox(x, y) => {
                        *image = image.resize_exact(x, y, image_filter);
                    }
                };
            }
            None => {
                match self.size {
                    Resize::Height(y) => {
                        let x: u32 = (aspect_ratio * y as f32) as u32 + 1;
                        *image = image.thumbnail(x, y);
                    }
                    Resize::Width(x) => {
                        let y: u32 = (x as f32 / aspect_ratio) as u32 + 1;
                        *image = image.thumbnail(x, y);
                    }
                    Resize::BoundingBox(x, y) => {
                        *image = image.thumbnail(x, y);
                    }
                    Resize::ExactBox(x, y) => {
                        *image = image.thumbnail_exact(x, y);
                    }
                };
            }
        };

        true
    }
}
#[derive(Copy, Clone)]
pub(crate) struct CropOp {
    crop: Crop,
}

impl CropOp {
    pub fn new(crop: Crop) -> Self {
        CropOp { crop }
    }
}

impl Operation for CropOp {
    fn apply(&self, image: &mut DynamicImage) -> bool {
        let (width, height) = image.dimensions();

        match self.crop {
            Crop::Box(x, y, w, h) => {
                *image = image.crop(x, y, w, h);
            }
            Crop::Ratio(w_r, h_r) => {
                let ratio_old = width as f32 / height as f32;
                let ratio_new = w_r / h_r;

                if ratio_old <= ratio_new {
                    let height_new = ((ratio_old / ratio_new) * height as f32) as u32;
                    let y_new = (height - height_new) / 2;

                    *image = image.crop(0, y_new, width, height_new);
                } else {
                    let width_new = ((ratio_new / ratio_old) * width as f32) as u32;
                    let x_new = (width - width_new) / 2;

                    *image = image.crop(x_new, 0, width_new, height);
                }
            }
        }
        true
    }
}

#[derive(Copy, Clone)]
pub(crate) struct BlurOp {
    sigma: f32,
}

impl BlurOp {
    pub fn new(sigma: f32) -> Self {
        BlurOp { sigma }
    }
}

impl Operation for BlurOp {
    fn apply(&self, image: &mut DynamicImage) -> bool
    where
        Self: Sized,
    {
        *image = image.blur(self.sigma);
        true
    }
}

#[derive(Copy, Clone)]
pub(crate) struct BrightenOp {
    value: i32,
}

impl BrightenOp {
    pub fn new(value: i32) -> Self {
        BrightenOp { value }
    }
}

impl Operation for BrightenOp {
    fn apply(&self, image: &mut DynamicImage) -> bool
    where
        Self: Sized,
    {
        *image = image.brighten(self.value);
        true
    }
}

#[derive(Copy, Clone)]
pub(crate) struct HuerotateOp {
    degree: i32,
}

impl HuerotateOp {
    pub fn new(degree: i32) -> Self {
        HuerotateOp { degree }
    }
}

impl Operation for HuerotateOp {
    fn apply(&self, image: &mut DynamicImage) -> bool
    where
        Self: Sized,
    {
        *image = image.huerotate(self.degree);
        true
    }
}

#[derive(Copy, Clone)]
pub(crate) struct ContrastOp {
    value: f32,
}

impl ContrastOp {
    pub fn new(value: f32) -> Self {
        ContrastOp { value }
    }
}

impl Operation for ContrastOp {
    fn apply(&self, image: &mut DynamicImage) -> bool
    where
        Self: Sized,
    {
        *image = image.adjust_contrast(self.value);
        true
    }
}

#[derive(Copy, Clone)]
pub(crate) struct FlipOp {
    orientation: Orientation,
}

impl FlipOp {
    pub fn new(orientation: Orientation) -> Self {
        FlipOp { orientation }
    }
}

impl Operation for FlipOp {
    fn apply(&self, image: &mut DynamicImage) -> bool
    where
        Self: Sized,
    {
        match self.orientation {
            Orientation::Vertical => *image = image.flipv(),
            Orientation::Horizontal => *image = image.fliph(),
        }

        true
    }
}

#[derive(Copy, Clone)]
pub(crate) struct InvertOp;

impl InvertOp {
    pub fn new() -> Self {
        InvertOp {}
    }
}

impl Operation for InvertOp {
    fn apply(&self, image: &mut DynamicImage) -> bool
    where
        Self: Sized,
    {
        image.invert();
        true
    }
}

#[derive(Clone)]
pub(crate) struct ExifOp {
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

#[derive(Clone)]
pub(crate) struct TextOp {
    text: String,
    pos: BoxPosition,
}

impl TextOp {
    pub fn new(text: String, pos: BoxPosition) -> Self {
        TextOp { text, pos }
    }
}

impl Operation for TextOp {
    fn apply(&self, image: &mut DynamicImage) -> bool
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

#[derive(Clone)]
pub(crate) struct CombineOp {
    image: StaticThumbnail,
    pos: BoxPosition,
}

impl<'a> CombineOp {
    pub fn new(image: StaticThumbnail, pos: BoxPosition) -> Self {
        CombineOp { image, pos }
    }
}

impl Operation for CombineOp {
    fn apply(&self, image: &mut DynamicImage) -> bool
    where
        Self: Sized,
    {
        let (pos_x, pos_y) = match self.pos {
            BoxPosition::TopLeft(x, y) => (x, y),
            BoxPosition::TopRight(x, y) => (x - self.image.get_width(), y),
            BoxPosition::BottomLeft(x, y) => (x, y - self.image.get_height()),
            BoxPosition::BottomRight(x, y) => {
                (x - self.image.get_width(), y - self.image.get_height())
            }
        };

        let buffer_background = match image.as_mut_rgba8() {
            Some(rgba_image) => rgba_image,
            None => return false,
        };

        match self.image.as_dyn().as_rgba8() {
            Some(rgba_image) => {
                for (x, y, pixel) in rgba_image.enumerate_pixels() {
                    let background_pixel = buffer_background.get_pixel_mut(x + pos_x, y + pos_y);
                    for index in 0..2 {
                        background_pixel[index] =
                            pixel[3] * pixel[index] + (1 - pixel[3]) * background_pixel[index];
                    }
                }
            }
            None => return false,
        };

        true
    }
}

#[derive(Copy, Clone)]
pub(crate) struct UnsharpenOp {
    sigma: f32,
    threshold: i32,
}

impl UnsharpenOp {
    pub fn new(sigma: f32, threshold: i32) -> Self {
        UnsharpenOp { sigma, threshold }
    }
}

impl Operation for UnsharpenOp {
    fn apply(&self, image: &mut DynamicImage) -> bool
    where
        Self: Sized,
    {
        *image = image.unsharpen(self.sigma, self.threshold);
        true
    }
}
