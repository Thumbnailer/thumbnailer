pub use crate::errors::OperationError;
use crate::thumbnail::operations::Operation;
use image::DynamicImage;

#[derive(Debug, Copy, Clone, Default)]
/// Representation of the invert-operation as struct
pub struct InvertOp;

impl InvertOp {
    /// Returns a new `InvertOp` struct
    pub fn new() -> Self {
        InvertOp {}
    }
}

impl Operation for InvertOp {
    /// Logic for the invert-operation
    ///
    /// This function inverts the colors in a `Dynamic-Image`.
    /// More information: [Negative colors](https://en.wikipedia.org/wiki/Negative_(photography))
    /// It returns `Ok(())` on success and `Err(OperationError)` in case of an error.
    ///
    /// # Arguments
    ///
    /// * `&self` - The `InvertOp` struct
    /// * `image` - The `DynamicImage` that should be inverted
    ///
    /// # Panic
    ///
    /// This function won't panic.
    ///
    /// # Examples
    /// ```
    /// use thumbnailer::thumbnail::operations::Operation;
    /// use thumbnailer::thumbnail::operations::InvertOp;
    /// use image::DynamicImage;
    ///
    /// let mut dynamic_image = DynamicImage::new_rgb8(800, 500);
    ///
    /// let invert_op = InvertOp::new();
    /// let res = invert_op.apply(&mut dynamic_image);
    ///
    /// assert!(res.is_ok());
    /// ```
    fn apply(&self, image: &mut DynamicImage) -> Result<(), OperationError>
    where
        Self: Sized,
    {
        image.invert();
        Ok(())
    }
}
