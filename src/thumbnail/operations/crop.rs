use crate::thumbnail::operations::Operation;
use crate::Crop;
use image::{DynamicImage, GenericImageView};

#[derive(Debug, Copy, Clone)]
/// Representation of the crop-operation as a struct
pub struct CropOp {
    /// contains the `Crop` enum as option
    crop: Crop,
}

impl CropOp {
    /// Returns a new `CropOp` struct with defined:
    /// * `crop' as instance of `Crop` enum
    pub fn new(crop: Crop) -> Self {
        CropOp { crop }
    }
}

impl Operation for CropOp {
    /// Logic for the crop-operation
    ///
    /// This function crops a `DynamicImage`, based on the type of the `Crop` enum
    /// * with `Crop::Box`: Exactly crops the image to a rectangle defined by the coordinates of the top-left-corner, a width and a height.
    /// * with `Crop::Ratio`: Crops the image to a rectangle given by a width-height-ratio. The rectangle is scaled to the maximum that fits
    /// inside the image
    ///
    /// It returns `true` on success and `false` in case of an error.
    ///
    /// # Arguments
    ///
    /// * `&self` - The `CropOp` struct
    /// * `image` - The `DynamicImage` that should be cropped
    ///
    /// # Panic
    ///
    /// This function won't panic ?
    ///
    /// # Examples
    /// ```
    /// use thumbnailer::generic::Crop;
    /// use thumbnailer::thumbnail::operations::Operation;
    /// use thumbnailer::thumbnail::operations::CropOp;
    /// use image::DynamicImage;
    ///
    /// let crop = Crop::Ratio(16.0, 9.0);
    /// let mut dynamic_image = DynamicImage::new_rgb8(800, 500);
    ///
    /// let crop_op = CropOp::new(crop);
    /// crop_op.apply(&mut dynamic_image);
    /// ```
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
