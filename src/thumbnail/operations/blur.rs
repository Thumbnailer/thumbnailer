pub use crate::errors::OperationError;
use crate::thumbnail::operations::Operation;
use image::DynamicImage;

#[derive(Debug, Copy, Clone)]
/// Representation of the blur-operation as a struct
pub struct BlurOp {
    /// Value that specifies how much the image should be blurred.
    /// More Information: [Gaussian Blur](https://en.wikipedia.org/wiki/Gaussian_blur)
    sigma: f32,
}

impl BlurOp {
    /// Returns a new `BlurOp` struct with defined:
    /// * `sigma`: More Information: [Gaussian Blur](https://en.wikipedia.org/wiki/Gaussian_blur)
    pub fn new(sigma: f32) -> Self {
        BlurOp { sigma }
    }
}

impl Operation for BlurOp {
    /// Logic for the blur-operation
    ///
    /// This function blurs a `DynamicImage` based on a given `sigma` in `BlurOp`
    /// Mathematical background: [Gaussian Blur](https://en.wikipedia.org/wiki/Gaussian_blur).
    /// It returns `true` on success and `false` in case of an error.
    ///
    /// # Arguments
    ///
    /// * `&self` - The `BlurOp` struct
    /// * `image` - The `DynamicImage` that should be blurred
    ///
    /// # Panic
    ///
    /// This function won't panic ?
    ///
    /// # Examples
    /// ```
    /// use thumbnailer::thumbnail::operations::Operation;
    /// use thumbnailer::thumbnail::operations::BlurOp;
    /// use image::DynamicImage;
    ///
    /// let mut dynamic_image = DynamicImage::new_rgb8(800, 500);
    ///
    /// let blur_op = BlurOp::new(3.5);
    /// blur_op.apply(&mut dynamic_image);
    /// ```
    fn apply(&self, image: &mut DynamicImage) -> Result<(), OperationError>
    where
        Self: Sized,
    {
        *image = image.blur(self.sigma);
        Ok(())
    }
}
