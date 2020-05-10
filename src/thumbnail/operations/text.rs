pub use crate::errors::{OperationError, OperationErrorInfo};
use crate::thumbnail::operations::Operation;
use crate::BoxPosition;
use image::{DynamicImage, Pixel};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

#[derive(Debug, Clone)]
/// Representation of the operation of drawing texts as a struct
pub struct TextOp {
    /// The text that should be drawn
    text: String,
    /// Specifies the position of the Text, represented by `BoxPosition` enum
    pos: BoxPosition,
}

impl TextOp {
    /// Returns a new `TextOp` struct with defined:
    /// * `text` as the text that should be drawn
    /// * `pos` as the position of the text represented by `BoxPosition` enum
    pub fn new(text: String, pos: BoxPosition) -> Self {
        TextOp { text, pos }
    }
}

impl Operation for TextOp {
    /// Logic for the operation of drawing texts on an image
    ///
    /// This function draws a `String` in a `DynamicImage` at the position defined in the `BoxPosition`-enum:
    /// * with `BoxPosition::TopLeft`: The top-left-corner of the text is placed at the defined coordinates
    /// * with `BoxPosition::TopRight`: The top-right-corner of the text is placed at the defined coordinates
    /// * with `BoxPosition::BottomLeft`: The bottom-left-corner of the text is placed at the defined coordinates
    /// * with `BoxPosition::BottomRight`: The bottom-right-corner of the text is placed at the defined coordinates
    ///
    /// It returns `true` on success and `false` in case of an error.
    ///
    /// # Arguments
    ///
    /// * `&self` - The `TextOp` struct
    /// * `image` - The `DynamicImage` where the text should be drawn on
    ///
    /// # Panic
    ///
    /// This function won't panic ?
    ///
    /// # Examples
    /// ```
    /// use thumbnailer::generic::BoxPosition;
    /// use thumbnailer::thumbnail::operations::Operation;
    /// use thumbnailer::thumbnail::operations::TextOp;
    /// use image::DynamicImage;
    ///
    /// let position = BoxPosition::TopLeft(23, 40);
    /// let mut dynamic_image = DynamicImage::new_rgb8(800, 500);
    ///
    /// let text_op = TextOp::new("Hello world!".to_string(), position);
    /// text_op.apply(&mut dynamic_image);
    /// ```
    fn apply(&self, image: &mut DynamicImage) -> Result<(), OperationError>
    where
        Self: Sized,
    {
        let scale = Scale { x: 12.0, y: 12.0 };

        let font_data: &[u8] = include_bytes!("../../../resources/fonts/Roboto-Regular.ttf");
        let font: Font<'static> = match Font::from_bytes(font_data) {
            Ok(font_bytes) => font_bytes,
            Err(_) => {
                return Err(OperationError::new(
                    Box::new(self.clone()),
                    OperationErrorInfo::FontLoadError,
                ))
            }
        };

        let mut string_width = 0.0;
        let string_height = font.v_metrics(scale).ascent - font.v_metrics(scale).descent;

        for glyph in font.glyphs_for(self.text.chars()) {
            string_width += glyph.scaled(scale).h_metrics().advance_width;
        }

        let (pos_x, pos_y) = match self.pos {
            BoxPosition::TopLeft(x, y) => (x, y),
            BoxPosition::TopRight(x, y) => {
                if x >= string_width as u32 {
                    (x - string_width as u32, y)
                } else {
                    return Err(OperationError::new(
                        Box::new(self.clone()),
                        OperationErrorInfo::CoordinatesOutOfRange,
                    ));
                }
            }
            BoxPosition::BottomLeft(x, y) => {
                if y >= string_height as u32 {
                    (x, y - string_height as u32)
                } else {
                    return Err(OperationError::new(
                        Box::new(self.clone()),
                        OperationErrorInfo::CoordinatesOutOfRange,
                    ));
                }
            }
            BoxPosition::BottomRight(x, y) => {
                if x >= string_width as u32 && y >= string_height as u32 {
                    (x - string_width as u32, y - string_height as u32)
                } else {
                    return Err(OperationError::new(
                        Box::new(self.clone()),
                        OperationErrorInfo::CoordinatesOutOfRange,
                    ));
                }
            }
        };

        draw_text_mut(
            image,
            Pixel::from_channels(255u8, 255u8, 255u8, 255u8),
            pos_x,
            pos_y,
            scale,
            &font,
            &self.text,
        );

        Ok(())
    }
}
