use crate::thumbnail::operations::Operation;
use crate::{BoxPosition, StaticThumbnail};
use image::DynamicImage;
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
    /// It returns `true` on success and `false` in case of an error.
    ///
    /// # Arguments
    ///
    /// * `&self` - The `CombineOp` struct
    /// * `image` - The `DynamicImage` where the text should be drawn on
    ///
    /// # Panic
    ///
    /// This function won't panic ?
    ///
    /// # Examples
    /// ```
    /// use thumbnailer::generic::BoxPosition;
    /// use thumbnailer::thumbnail::StaticThumbnail;
    /// use thumbnailer::thumbnail::operations::Operation;
    /// use thumbnailer::thumbnail::operations::CombineOp;
    /// use image::DynamicImage;
    ///
    /// let position = BoxPosition::TopLeft(23, 40);
    /// let mut dynamic_image = DynamicImage::new_rgb8(800, 500);/*
    /// let mut static_image = StaticThumbnail{
    ///     image: DynamicImage::new_rgb8(100, 100),
    /// };
    ///
    /// let combine_op = CombineOp::new(static_image, position);
    /// combine_op.apply(&mut dynamic_image);*/
    /// ```
    fn apply(&self, image: &mut DynamicImage) -> bool
    where
        Self: Sized,
    {
        let (pos_x, pos_y) = match self.pos {
            BoxPosition::TopLeft(x, y) => (x, y),
            BoxPosition::TopRight(x, y) => {
                if x >= self.image.get_width() {
                    (x - self.image.get_width(), y)
                } else {
                    return false;
                }
            }
            BoxPosition::BottomLeft(x, y) => {
                if y >= self.image.get_height() {
                    (x, y - self.image.get_height())
                } else {
                    return false;
                }
            }
            BoxPosition::BottomRight(x, y) => {
                if x >= self.image.get_width() && y >= self.image.get_height() {
                    (x - self.image.get_width(), y - self.image.get_height())
                } else {
                    return false;
                }
            }
        };

        let buffer_background = match image.as_mut_rgba8() {
            Some(rgba_image) => rgba_image,
            None => return false,
        };

        match self.image.as_dyn().as_rgba8() {
            Some(rgba_image) => {
                for (x, y, pixel) in rgba_image.enumerate_pixels() {
                    let background_pixel = buffer_background.get_pixel_mut(x + pos_x, y + pos_y);
                    for index in 0..2 {
                        background_pixel[index] =
                            pixel[3] * pixel[index] + (1 - pixel[3]) * background_pixel[index];
                    }
                }
            }
            None => return false,
        };

        true
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
