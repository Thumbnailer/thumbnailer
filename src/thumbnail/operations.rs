use crate::generic::{BoxPosition, Crop, Exif, Orientation, ResampleFilter, Resize};
use crate::thumbnail::{ImageData, Thumbnail};
use crate::StaticThumbnail;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView};
use image::Pixel;
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

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
/// Representation of the resizing operation as a struct
pub(crate) struct ResizeOp {
    /// Contains the Resize enum as option
    size: Resize,
    /// Contains an optional filter for the resize operation
    filter: Option<ResampleFilter>,
}

impl ResizeOp {
    /// Returns a new ResizeOp-struct with defined:
    /// * size as instance of Resize-enum
    /// * optional filter
    pub fn new(size: Resize, filter: Option<ResampleFilter>) -> Self {
        ResizeOp { size, filter }
    }
}

impl Operation for ResizeOp {
    /// Logic for the resize-operation
    ///
    /// This function resizes a DynamicImage, depending on the options given by the members of ResizeOp-struct.
    /// It returns true on success and false in case of an error.
    ///
    /// # Arguments
    ///
    /// * '&self' - The ResizeOp-struct
    /// * 'image' - The DynamicImage that should be resized
    ///
    /// # Panic
    ///
    /// This function won't panic ?
    ///
    /// # Examples
    /// '''
    /// use Thumbnailer::generic::{Resize, ResampleFilter};
    /// use Thumbnailer::operations::ResizeOp
    /// use image::DynamicImage;
    ///
    /// let size = Resize::BoundingBox(400, 300);
    /// let filter = ResampleFilter::Gaussian;
    /// let dynamic_image = DynamicImage::new_rgb8(800, 500);
    ///
    /// let resize_op = ResizeOp::new(size, filter);
    /// resize_op.apply(dynamic_image);
    /// '''
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
/// Representation of the crop-operation as a struct
pub(crate) struct CropOp {
    /// contains the Crop-enum as option
    crop: Crop,
}

impl CropOp {
    /// Returns a new CropOp-struct with defined:
    /// * crop as instance of Crop-enum
    pub fn new(crop: Crop) -> Self {
        CropOp { crop }
    }
}

impl Operation for CropOp {
    /// Logic for the crop-operation
    ///
    /// This function crops a DynamicImage, based on the type of the Crop-enum
    /// * with Crop::Box: Exactly crops the image to a rectangle defined by the coordinates of the top-left-corner, a width and a height.
    /// * with Crop::Ratio: Crops the image to a rectangle given by a width-height-ratio. The rectangle is scaled to the maximum that fits
    /// inside the image
    ///
    /// It returns true on success and false in case of an error.
    ///
    /// # Arguments
    ///
    /// * '&self' - The CropOp-struct
    /// * 'image' - The DynamicImage that should be cropped
    ///
    /// # Panic
    ///
    /// This function won't panic ?
    ///
    /// # Examples
    /// '''
    /// use Thumbnailer::generic::Crop;
    /// use Thumbnailer::operations::CropOp
    /// use image::DynamicImage;
    ///
    /// let crop = Crop::Ratio(16, 9);
    /// let dynamic_image = DynamicImage::new_rgb8(800, 500);
    ///
    /// let crop_op = CropOp::new(crop);
    /// crop_op.apply(dynamic_image);
    /// '''
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
        let scale = Scale { x: 12.0, y: 12.0 };

        let font_data: &[u8] = include_bytes!("../../resources/fonts/Roboto-Regular.ttf");
        let font: Font<'static> = match Font::from_bytes(font_data) {
            Ok(font_bytes) => font_bytes,
            Err(_) => return false,
        };

        let mut string_width = 0.0;
        let string_height = font.v_metrics(scale).ascent - font.v_metrics(scale).descent;

        for glyph in font.glyphs_for(self.text.chars()) {
            string_width += glyph.scaled(scale).h_metrics().advance_width;
        }

        let (pos_x, pos_y) = match self.pos {
            BoxPosition::TopLeft(x, y) => (x, y),
            BoxPosition::TopRight(x, y) => {
                if x >= string_width as u32 {
                    (x - string_width as u32, y)
                } else {
                    return false;
                }
            }
            BoxPosition::BottomLeft(x, y) => {
                if y >= string_height as u32 {
                    (x, y - string_height as u32)
                } else {
                    return false;
                }
            }
            BoxPosition::BottomRight(x, y) => {
                if x >= string_width as u32 && y >= string_height as u32 {
                    (x - string_width as u32, y - string_height as u32)
                } else {
                    return false;
                }
            }
        };

        draw_text_mut(
            image,
            Pixel::from_channels(255u8, 255u8, 255u8, 255u8),
            pos_x,
            pos_y,
            scale,
            &font,
            &self.text,
        );

        true
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
            BoxPosition::TopRight(x, y) => {
                if x >= self.image.get_width() {
                    (x - self.image.get_width(), y)
                } else {
                    return false;
                }
            }
            BoxPosition::BottomLeft(x, y) => {
                if y >= self.image.get_height() {
                    (x, y - self.image.get_height())
                } else {
                    return false;
                }
            }
            BoxPosition::BottomRight(x, y) => {
                if x >= self.image.get_width() && y >= self.image.get_height() {
                    (x - self.image.get_width(), y - self.image.get_height())
                } else {
                    return false;
                }
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
