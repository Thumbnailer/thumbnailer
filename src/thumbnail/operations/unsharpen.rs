use crate::thumbnail::operations::Operation;
use image::DynamicImage;

#[derive(Debug, Copy, Clone)]
/// Representation of the unsharpen-operation as a struct
pub struct UnsharpenOp {
    /// amount to blur the image by
    sigma: f32,
    /// control of how much to sharpen
    threshold: i32,
}

impl UnsharpenOp {
    /// Returns a new `UnsharpenOp` struct with defined:
    /// * `sigma` as amount to blur the 'DynamicImage'
    /// * `threshold` as control of how much to sharpen
    ///
    /// More information: [Digital unsharp masking](https://en.wikipedia.org/wiki/Unsharp_masking#Digital_unsharp_masking)
    pub fn new(sigma: f32, threshold: i32) -> Self {
        UnsharpenOp { sigma, threshold }
    }
}

impl Operation for UnsharpenOp {
    /// Logic for the unsharpen-operation
    ///
    /// This function unsharpens a `DynamicImage` based on the given `UnsharpenOp`
    /// Mathematical background: [Digital unsharp masking](https://en.wikipedia.org/wiki/Unsharp_masking#Digital_unsharp_masking).
    /// It returns `true` on success and `false` in case of an error.
    ///
    /// # Arguments
    ///
    /// * `&self` - The `UnsharpenOp` struct
    /// * `image` - The `DynamicImage` that should be unsharpened
    ///
    /// # Panic
    ///
    /// This function won't panic ?
    ///
    /// # Examples
    /// ```
    /// use thumbnailer::thumbnail::operations::Operation;
    /// use thumbnailer::thumbnail::operations::UnsharpenOp;
    /// use image::DynamicImage;
    ///
    /// let mut dynamic_image = DynamicImage::new_rgb8(800, 500);
    ///
    /// let unsharpen_op = UnsharpenOp::new(3.5, 5);
    /// unsharpen_op.apply(&mut dynamic_image);
    /// ```
    fn apply(&self, image: &mut DynamicImage) -> bool
    where
        Self: Sized,
    {
        *image = image.unsharpen(self.sigma, self.threshold);
        true
    }
}
