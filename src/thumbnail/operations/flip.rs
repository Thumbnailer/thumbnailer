pub use crate::errors::OperationError;
use crate::thumbnail::operations::Operation;
use crate::Orientation;
use image::DynamicImage;

#[derive(Debug, Copy, Clone)]
/// Representation of the flip-operation as struct
pub struct FlipOp {
    /// contains the `Orientation` enum
    orientation: Orientation,
}

impl FlipOp {
    /// Returns a new `FlipOp` struct with defined:
    /// * `orientation` as instance of `Orientation` enum
    pub fn new(orientation: Orientation) -> Self {
        FlipOp { orientation }
    }
}

impl Operation for FlipOp {
    /// Logic for the flip-operation
    ///
    /// This function flips a `DynamicImage` based on the option selected in the `Orientation`-enum:
    /// * with `Orientation::Vertical`: Flips the image vertically.
    /// * with `Orientation::Horizontal`: Flips the image horizontally.
    ///
    /// It returns `Ok(())` on success and `Err(OperationError)` in case of an error.
    ///
    /// # Arguments
    ///
    /// * `&self` - The `FlipOp` struct
    /// * `image` - The `DynamicImage` that should be flipped
    ///
    /// # Panic
    ///
    /// This function won't panic ?
    ///
    /// # Examples
    /// ```
    /// use thumbnailer::generic::Orientation;
    /// use thumbnailer::thumbnail::operations::Operation;
    /// use thumbnailer::thumbnail::operations::FlipOp;
    /// use image::DynamicImage;
    ///
    /// let orientation = Orientation::Vertical;
    /// let mut dynamic_image = DynamicImage::new_rgb8(800, 500);
    ///
    /// let flip_op = FlipOp::new(orientation);
    /// flip_op.apply(&mut dynamic_image);
    /// ```
    fn apply(&self, image: &mut DynamicImage) -> Result<(), OperationError>
    where
        Self: Sized,
    {
        match self.orientation {
            Orientation::Vertical => *image = image.flipv(),
            Orientation::Horizontal => *image = image.fliph(),
        }
        Ok(())
    }
}
