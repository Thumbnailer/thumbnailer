pub use crate::errors::{OperationError, OperationErrorInfo};
use crate::thumbnail::operations::Operation;
use crate::{ResampleFilter, Resize};
use image::imageops::FilterType;
use image::DynamicImage;

#[derive(Debug, Copy, Clone)]
/// Representation of the resizing operation as a struct
pub struct ResizeOp {
    /// Contains the `Resize` enum as option
    size: Resize,
    /// Contains an optional filter for the resize operation
    filter: Option<ResampleFilter>,
}

impl ResizeOp {
    /// Returns a new `ResizeOp` struct with defined:
    /// * `size` as instance of `Resize` enum
    /// * optional `filter`
    pub fn new(size: Resize, filter: Option<ResampleFilter>) -> Self {
        ResizeOp { size, filter }
    }
}

impl Operation for ResizeOp {
    /// Logic for the resize-operation
    ///
    /// This function resizes a `DynamicImage`, depending on the options given by the members of `ResizeOp` struct.
    /// It returns `Ok(())` on success and `Err(OperationError)` in case of an error.
    ///
    /// # Arguments
    ///
    /// * `&self` - The `ResizeOp` struct
    /// * `image` - The `DynamicImage` that should be resized
    ///
    /// # Panic
    ///
    /// This function won't panic.
    ///
    /// # Examples
    /// ```
    /// use thumbnailer::generic::{Resize, ResampleFilter};
    /// use thumbnailer::thumbnail::operations::Operation;
    /// use thumbnailer::thumbnail::operations::ResizeOp;
    /// use image::DynamicImage;
    ///
    /// let size = Resize::BoundingBox(400, 300);
    /// let filter = ResampleFilter::Gaussian;
    /// let mut dynamic_image = DynamicImage::new_rgb8(800, 500);
    ///
    /// let resize_op = ResizeOp::new(size, Some(filter));
    /// let res = resize_op.apply(&mut dynamic_image);
    ///
    /// assert!(res.is_ok());
    /// ```
    fn apply(&self, image: &mut DynamicImage) -> Result<(), OperationError> {
        let aspect_ratio = match image.as_rgb8() {
            Some(rgb_image) => rgb_image.width() as f32 / rgb_image.height() as f32,
            _ => {
                return Err(OperationError::new(
                    Box::new(*self),
                    OperationErrorInfo::RgbImageConversionFailure,
                ))
            }
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

        Ok(())
    }
}
