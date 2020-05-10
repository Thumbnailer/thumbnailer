pub use crate::errors::OperationError;
use crate::thumbnail::operations::Operation;
use crate::Rotation;
use image::DynamicImage;

#[derive(Debug, Copy, Clone)]
/// Representation of the rotate-operation as struct
pub struct RotateOp {
    /// contains the `Rotation` enum
    rotation: Rotation,
}

impl RotateOp {
    /// Returns a new `RotateOp` struct with defined:
    /// * `rotation` as instance of `Rotation` enum
    pub fn new(rotation: Rotation) -> Self {
        RotateOp { rotation }
    }
}

impl Operation for RotateOp {
    /// Logic for the rotate-operation
    ///
    /// This function rotates a `DynamicImage` based on the option selected in the `Rotation`-enum:
    /// * with `Rotation::Rotate90`: Rotates the image 90 degrees clockwise.
    /// * with `Rotation::Rotate180`: Rotates the image 180 degrees clockwise.
    /// * with `Rotation::Rotate270`: Rotates the image 270 degrees clockwise.
    ///
    /// It returns `Ok(())` on success and `Err(OperationError)` in case of an error.
    ///
    /// # Arguments
    ///
    /// * `&self` - The `RotateOp` struct
    /// * `image` - The `DynamicImage` that should be rotated
    ///
    /// # Panic
    ///
    /// This function won't panic
    ///
    /// # Examples
    /// ```
    /// use thumbnailer::generic::Rotation;
    /// use thumbnailer::thumbnail::operations::Operation;
    /// use thumbnailer::thumbnail::operations::RotateOp;
    /// use image::DynamicImage;
    ///
    /// let rotation = Rotation::Rotate90;
    /// let mut dynamic_image = DynamicImage::new_rgb8(800, 500);
    ///
    /// let rotate_op = RotateOp::new(rotation);
    /// let res = rotate_op.apply(&mut dynamic_image);
    ///
    /// assert!(res.is_ok());
    /// ```
    fn apply(&self, image: &mut DynamicImage) -> Result<(), OperationError>
    where
        Self: Sized,
    {
        match self.rotation {
            Rotation::Rotate90 => *image = image.rotate90(),
            Rotation::Rotate180 => *image = image.rotate180(),
            Rotation::Rotate270 => *image = image.rotate270(),
        }
        Ok(())
    }
}
