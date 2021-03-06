pub use crate::errors::{OperationError, OperationErrorInfo};
use crate::thumbnail::operations::Operation;
use crate::{BoxPosition, StaticThumbnail};
use image::{DynamicImage, GenericImageView};
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone)]
/// Representation of the combine operation as a struct
pub struct CombineOp {
    /// The overlay image as `StaticThumbnail`
    image: StaticThumbnail,
    /// Specifies the position of the Text, represented by `BoxPosition` enum
    pos: BoxPosition,
}

impl<'a> CombineOp {
    /// Returns a new `CombineOp` struct with defined:
    /// * `image` as the image that should be drawn on the 'DynamicImage'
    /// * `pos` as the position of the text represented by `BoxPosition` enum
    pub fn new(image: StaticThumbnail, pos: BoxPosition) -> Self {
        CombineOp { image, pos }
    }
}

impl Operation for CombineOp {
    /// Logic for the operation of drawing an image on top of another image
    ///
    /// This function draws a `StaticThumbnail` on top of a `DynamicImage` at the position defined in the `BoxPosition`-enum:
    /// * with `BoxPosition::TopLeft`: The top-left-corner of the overlayed image is placed at the defined coordinates
    /// * with `BoxPosition::TopRight`: The top-right-corner of the overlayed image is placed at the defined coordinates
    /// * with `BoxPosition::BottomLeft`: The bottom-left-corner of the overlayed image is placed at the defined coordinates
    /// * with `BoxPosition::BottomRight`: The bottom-right-corner of the overlayed image is placed at the defined coordinates
    ///
    /// It returns `Ok(())` on success and `Err(OperationError)` in case of an error.
    ///
    /// # Arguments
    ///
    /// * `&self` - The `CombineOp` struct
    /// * `image` - The `DynamicImage` where the text should be drawn on
    ///
    /// # Errors
    ///
    /// * CoordinatesOutOfRange - The coordinates for the overlayed image are not inside the background image
    /// * ImageBufferConversionFailure - The supplied background image cannot be converted to an 'ImageBuffer'
    ///
    /// # Panic
    ///
    /// This function won't panic.
    ///
    /// # Examples
    /// ```
    /// use thumbnailer::generic::BoxPosition;
    /// use thumbnailer::thumbnail::Thumbnail;
    /// use thumbnailer::thumbnail::StaticThumbnail;
    /// use thumbnailer::thumbnail::operations::Operation;
    /// use thumbnailer::thumbnail::operations::CombineOp;
    /// use image::DynamicImage;
    ///
    /// let position = BoxPosition::BottomRight(750, 450);
    /// let mut dynamic_image = DynamicImage::new_rgba8(800, 500);
    /// let dynamic_image_2 = DynamicImage::new_rgb8(100, 100);
    ///
    /// let mut thumbnail = Thumbnail::from_dynamic_image("test.jpg", dynamic_image_2);
    /// let mut static_thumbnail = match thumbnail.clone_static_copy() {
    ///     Some(static_tn) => static_tn,
    ///     None => panic!("Error!"),
    /// };
    ///
    /// let combine_op = CombineOp::new(static_thumbnail, position);
    /// let res = combine_op.apply(&mut dynamic_image);
    ///
    /// assert!(res.is_ok());
    /// ```
    fn apply(&self, image: &mut DynamicImage) -> Result<(), OperationError>
    where
        Self: Sized,
    {
        let (overlay_width, overlay_height) = self.image.dimensions();
        let (x_pos_overlay_image, y_pos_overlay_image) = match self.pos {
            BoxPosition::TopLeft(x, y) => (x, y),
            BoxPosition::TopRight(x, y) => {
                if x >= overlay_width {
                    (x - overlay_width, y)
                } else {
                    return Err(OperationError::new(
                        Box::new(self.clone()),
                        OperationErrorInfo::CoordinatesOutOfRange,
                    ));
                }
            }
            BoxPosition::BottomLeft(x, y) => {
                if y >= overlay_height {
                    (x, y - overlay_height)
                } else {
                    return Err(OperationError::new(
                        Box::new(self.clone()),
                        OperationErrorInfo::CoordinatesOutOfRange,
                    ));
                }
            }
            BoxPosition::BottomRight(x, y) => {
                if x >= overlay_width && y >= overlay_height {
                    (x - overlay_width, y - overlay_height)
                } else {
                    return Err(OperationError::new(
                        Box::new(self.clone()),
                        OperationErrorInfo::CoordinatesOutOfRange,
                    ));
                }
            }
        };

        let overlay_image_buffer = self.image.as_dyn().to_rgba();
        let (bg_width, bg_height) = image.dimensions();

        match image.as_mut_rgba8() {
            Some(background_buffer) => {
                // Insertion of the overlay if the background ist a RgbaImage
                for (x, y, pixel) in overlay_image_buffer.enumerate_pixels() {
                    let x_pos_current_pixel = x + x_pos_overlay_image;
                    let y_pos_current_pixel = y + y_pos_overlay_image;

                    if x_pos_current_pixel < bg_width && y_pos_current_pixel < bg_height {
                        let background_pixel = background_buffer
                            .get_pixel_mut(x_pos_current_pixel, y_pos_current_pixel);
                        let alpha = pixel[3] as f32 / 255.0;
                        let alpha_inv = 1.0 - alpha;

                        for index in 0..2 {
                            background_pixel[index] = (alpha * pixel[index] as f32
                                + alpha_inv * background_pixel[index] as f32)
                                as u8;
                        }
                    }
                }
            }
            None => match image.as_mut_rgb8() {
                Some(background_buffer) => {
                    // Insertion of the overlay if the background is a RgbImage
                    for (x, y, pixel) in overlay_image_buffer.enumerate_pixels() {
                        let x_pos_current_pixel = x + x_pos_overlay_image;
                        let y_pos_current_pixel = y + y_pos_overlay_image;

                        if x_pos_current_pixel < bg_width && y_pos_current_pixel < bg_height {
                            let background_pixel = background_buffer
                                .get_pixel_mut(x_pos_current_pixel, y_pos_current_pixel);
                            let alpha = pixel[3] as f32 / 255.0;
                            let alpha_inv = 1.0 - alpha;

                            for index in 0..2 {
                                background_pixel[index] = (alpha * pixel[index] as f32
                                    + alpha_inv * background_pixel[index] as f32)
                                    as u8;
                            }
                        }
                    }
                }
                None => {
                    return Err(OperationError::new(
                        Box::new(self.clone()),
                        OperationErrorInfo::ImageBufferConversionFailure,
                    ))
                }
            },
        };

        Ok(())
    }
}

impl fmt::Debug for CombineOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CombineOp: StaticThumbnail {} at pos {:?}",
            self.image.get_src_path().to_str().unwrap_or_default(),
            self.pos
        )
    }
}
