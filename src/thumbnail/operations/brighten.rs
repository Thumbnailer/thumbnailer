pub use crate::errors::OperationError;
use crate::thumbnail::operations::Operation;
use image::DynamicImage;

#[derive(Debug, Copy, Clone)]
/// Representation of the brighten-operation as a struct.
pub struct BrightenOp {
    /// Value of how much the image should be brightened.
    /// Positive values will increase, negative values will decrease brigthness.
    value: i32,
}

impl BrightenOp {
    /// Returns a new `BrightenOp` struct with defined:
    /// * `value: i32`
    pub fn new(value: i32) -> Self {
        BrightenOp { value }
    }
}

impl Operation for BrightenOp {
    /// Logic for the brighten-operation
    ///
    /// This function brightens a `DynamicImage` based on the given `value` in `BrightenOp`
    /// Positive values will brighten the image up and negative values will decrease the brightess.
    /// It returns `Ok(())` on success and `Err(OperationError)` in case of an error.
    ///
    /// # Arguments
    ///
    /// * `&self` - The `BrightenOp` struct
    /// * `image` - The `DynamicImage` that should be brightened
    ///
    /// # Panic
    ///
    /// This function won't panic.
    ///
    /// # Examples
    /// ```
    /// use thumbnailer::thumbnail::operations::Operation;
    /// use thumbnailer::thumbnail::operations::BrightenOp;
    /// use image::DynamicImage;
    ///
    /// let mut dynamic_image = DynamicImage::new_rgb8(800, 500);
    ///
    /// let brighten_op = BrightenOp::new(5);
    /// let res = brighten_op.apply(&mut dynamic_image);
    ///
    /// assert!(res.is_ok());
    /// ```
    fn apply(&self, image: &mut DynamicImage) -> Result<(), OperationError>
    where
        Self: Sized,
    {
        *image = image.brighten(self.value);
        Ok(())
    }
}
