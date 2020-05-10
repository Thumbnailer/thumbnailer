pub use crate::errors::OperationError;
use crate::thumbnail::operations::Operation;
use image::DynamicImage;

#[derive(Debug, Copy, Clone)]
/// Representation of the contrast-operation as a struct.
pub struct ContrastOp {
    /// Value of how much the contrast should be adjusted.
    /// Positive values will increase and negative values will decrease the contrast.
    value: f32,
}

impl ContrastOp {
    /// Returns a new `ContrastOp` struct with defined:
    /// * `value: f32`
    pub fn new(value: f32) -> Self {
        ContrastOp { value }
    }
}

impl Operation for ContrastOp {
    /// Logic for the contrast-operation
    ///
    /// This function adjusts the contrast in a `Dynamic-Image`.
    /// Positive values will increase the contrast and negative values will decrease the contrast.
    /// It returns `Ok(())` on success and `Err(OperationError)` in case of an error.
    ///
    /// # Arguments
    ///
    /// * `&self` - The `ContrastOp` struct
    /// * `image` - The `DynamicImage` where the contrast should be adjusted
    ///
    /// # Panic
    ///
    /// This function won't panic ?
    ///
    /// # Examples
    /// ```
    /// use thumbnailer::thumbnail::operations::Operation;
    /// use thumbnailer::thumbnail::operations::ContrastOp;
    /// use image::DynamicImage;
    ///
    /// let mut dynamic_image = DynamicImage::new_rgb8(800, 500);
    ///
    /// let contrast_op = ContrastOp::new(5.0);
    /// contrast_op.apply(&mut dynamic_image);
    /// ```
    fn apply(&self, image: &mut DynamicImage) -> Result<(), OperationError>
    where
        Self: Sized,
    {
        *image = image.adjust_contrast(self.value);
        Ok(())
    }
}
