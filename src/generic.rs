use crate::errors::{ApplyError, OperationError};
use crate::thumbnail::operations::{
    BlurOp, BrightenOp, CombineOp, ContrastOp, CropOp, ExifOp, FlipOp, HuerotateOp, InvertOp,
    Operation, ResizeOp, TextOp, UnsharpenOp,
};
use crate::StaticThumbnail;

#[derive(Debug, Copy, Clone)]
/// The different options for the resize-operation as an enum
pub enum Resize {
    /// Option: scale to a given height, keep aspect ratio.
    /// ### Arguments:
    /// * height: `u32`
    Height(u32),
    /// Option: scale to a given width, keep aspect ratio.
    /// ### Arguments:
    /// * width: `u32`
    Width(u32),
    /// Option: scale the image so that it fits inside the box given by width and height, keep aspect ratio.
    /// ### Arguments:
    /// * width: `u32`
    /// * height: `u32`
    BoundingBox(u32, u32),
    /// Option: scale the image to the given width and height exactly, aspect ratio may be changed.
    /// ### Arguments:
    /// * width: `u32`
    /// * height: `u32`
    ExactBox(u32, u32),
}

#[derive(Debug, Copy, Clone)]
/// Different positioning-options for overlays as an enum
pub enum BoxPosition {
    /// Coordinates of the top-left-corner in the background image of the overlayed object.
    /// ### Arguments:
    /// * position_x: `u32`
    /// * position_y: `u32`
    TopLeft(u32, u32),
    /// Coordinates of the top-right-corner in the background image of the overlayed object.
    /// ### Arguments:
    /// * position_x: `u32`
    /// * position_y: `u32`
    TopRight(u32, u32),
    /// Coordinates of the bottom-left-corner in the background image of the overlayed object.
    /// ### Arguments:
    /// * position_x: `u32`
    /// * position_y: `u32`
    BottomLeft(u32, u32),
    /// Coordinates of the bottom-right-corner in the background image of the overlayed object.
    /// ### Arguments:
    /// * position_x: `u32`
    /// * position_y: `u32`
    BottomRight(u32, u32),
}

#[derive(Debug, Copy, Clone)]
/// Different options for cropping as an enum
pub enum Crop {
    /// Options for exactly cropping the image to a rectangle given by the coordinates of the top-left-corner and width and height.
    /// ### Arguments:
    /// * position_x: `u32`
    /// * position_y: `u32`
    /// * width: `u32`
    /// * height: `u32`
    Box(u32, u32, u32, u32),
    /// Option for cropping the image to a rectangle given by a ratio of width and height.
    /// The rectangle is scaled to the maximum that fits inside the origin image.
    /// ### Arguments:
    /// * ratio_width: `u32`
    /// * ratio_height: `u32`
    Ratio(f32, f32),
}

#[derive(Debug, Copy, Clone)]
/// Orientation options as an enum
pub enum Orientation {
    /// Option for a vertical orientation
    Vertical,
    /// Option for a horizontal orientation
    Horizontal,
}

#[derive(Debug, Clone)]
pub enum Exif {
    Keep,
    Clear,
    Whitelist(Vec<u16>),
    Blacklist(Vec<u16>),
}

#[derive(Debug, Copy, Clone)]
/// Collection of filters that can be applied to images
pub enum ResampleFilter {
    /// Nearest Neighbor Filter
    Nearest,
    /// Linear Filter
    Triangle,
    /// Cubic Filter
    CatmullRom,
    /// Gaussian Filter
    Gaussian,
    /// Lanczos with window 3
    Lanczos3,
}

pub trait OperationContainer {
    fn add_op(&mut self, op: Box<dyn Operation>);
}

pub trait GenericThumbnail: GenericThumbnailOperations {
    fn apply(&mut self) -> Result<&mut dyn GenericThumbnail, ApplyError>;
}

pub trait GenericThumbnailOperations {
    fn resize(&mut self, size: Resize) -> &mut dyn GenericThumbnail;
    fn resize_filter(&mut self, size: Resize, filter: ResampleFilter) -> &mut dyn GenericThumbnail;

    fn blur(&mut self, sigma: f32) -> &mut dyn GenericThumbnail;
    fn brighten(&mut self, value: i32) -> &mut dyn GenericThumbnail;
    fn huerotate(&mut self, degree: i32) -> &mut dyn GenericThumbnail;
    fn contrast(&mut self, value: f32) -> &mut dyn GenericThumbnail;
    fn unsharpen(&mut self, sigma: f32, threshold: i32) -> &mut dyn GenericThumbnail;

    fn crop(&mut self, c: Crop) -> &mut dyn GenericThumbnail;
    fn flip(&mut self, orientation: Orientation) -> &mut dyn GenericThumbnail;

    fn invert(&mut self) -> &mut dyn GenericThumbnail;

    fn exif(&mut self, metadata: Exif) -> &mut dyn GenericThumbnail;
    fn text(&mut self, text: String, pos: BoxPosition) -> &mut dyn GenericThumbnail;

    fn combine(&mut self, image: StaticThumbnail, pos: BoxPosition) -> &mut dyn GenericThumbnail;
}

impl<T> GenericThumbnailOperations for T
where
    T: OperationContainer + GenericThumbnail,
{
    fn resize(&mut self, size: Resize) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(ResizeOp::new(size, None)));
        self
    }

    fn resize_filter(&mut self, size: Resize, filter: ResampleFilter) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(ResizeOp::new(size, Option::from(filter))));
        self
    }

    fn blur(&mut self, sigma: f32) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(BlurOp::new(sigma)));
        self
    }

    fn brighten(&mut self, value: i32) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(BrightenOp::new(value)));
        self
    }

    fn huerotate(&mut self, degree: i32) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(HuerotateOp::new(degree)));
        self
    }

    fn contrast(&mut self, value: f32) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(ContrastOp::new(value)));
        self
    }

    fn unsharpen(&mut self, sigma: f32, threshold: i32) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(UnsharpenOp::new(sigma, threshold)));
        self
    }

    fn crop(&mut self, c: Crop) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(CropOp::new(c)));
        self
    }

    fn flip(&mut self, orientation: Orientation) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(FlipOp::new(orientation)));
        self
    }

    fn invert(&mut self) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(InvertOp::new()));
        self
    }

    fn exif(&mut self, metadata: Exif) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(ExifOp::new(metadata)));
        self
    }

    fn text(&mut self, text: String, pos: BoxPosition) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(TextOp::new(text, pos)));
        self
    }

    fn combine(&mut self, image: StaticThumbnail, pos: BoxPosition) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(CombineOp::new(image, pos)));
        self
    }
}
