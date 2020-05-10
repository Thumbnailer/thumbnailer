pub use crate::errors::OperationError;
use crate::thumbnail::operations::Operation;
use image::DynamicImage;

#[derive(Debug, Copy, Clone)]
/// Representation of the hue rotate operation as a struct.
pub struct HuerotateOp {
    /// Degrees each picel will be hue rotated by.
    degree: i32,
}

impl HuerotateOp {
    /// Returns a new `HuerotateOp` struct with defined:
    /// * `degree: i32`
    pub fn new(degree: i32) -> Self {
        HuerotateOp { degree }
    }
}

impl Operation for HuerotateOp {
    /// Logic for the hue rotate operation
    ///
    /// This function hue rotates a `Dynamic-Image`.
    /// It returns `Ok(())` on success and `Err(OperationError)` in case of an error.
    ///
    /// # Arguments
    ///
    /// * `&self` - The `HuerotateOp` struct
    /// * `image` - The `DynamicImage` that should be hue rotated
    ///
    /// # Panic
    ///
    /// This function won't panic.
    ///
    /// # Examples
    /// ```
    /// use thumbnailer::thumbnail::operations::Operation;
    /// use thumbnailer::thumbnail::operations::HuerotateOp;
    /// use image::DynamicImage;
    ///
    /// let mut dynamic_image = DynamicImage::new_rgb8(800, 500);
    ///
    /// let huerotate_op = HuerotateOp::new(90);
    /// huerotate_op.apply(&mut dynamic_image);
    /// ```
    fn apply(&self, image: &mut DynamicImage) -> Result<(), OperationError>
    where
        Self: Sized,
    {
        *image = image.huerotate(self.degree);
        Ok(())
    }
}
