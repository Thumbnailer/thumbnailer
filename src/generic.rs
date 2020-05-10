use crate::errors::ApplyError;
use crate::thumbnail::operations::{
    BlurOp, BrightenOp, CombineOp, ContrastOp, CropOp, ExifOp, FlipOp, HuerotateOp, InvertOp,
    Operation, ResizeOp, RotateOp, TextOp, UnsharpenOp,
};
use crate::{StaticThumbnail, Target};

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

#[derive(Debug, Copy, Clone)]
/// Rotation options as an enum
pub enum Rotation {
    /// Option for a 90 degree clockwise rotation
    Rotate90,
    /// Option for a 180 degree clockwise rotation
    Rotate180,
    /// Option for a 270 degree clockwise rotation
    Rotate270,
}

/// A trait for the queueing of operations
pub trait OperationContainer {
    /// Adds an operation to Thumbnails
    ///
    /// With this function implemented it should be possible to add single operations to the queue of the object
    ///
    /// # Arguments
    ///
    /// * `&mut self`: The object that contains a queue for which the function is implemented
    /// * `op`: The operation that should be added as `Box<dyn Operation>`
    fn add_op(&mut self, op: Box<dyn Operation>);
}

/// A trait for executing operations on a Thumbnail
pub trait GenericThumbnail: GenericThumbnailOperations {
    /// Applies the queued operations of implementors of `GenericImage` and clears the queue
    ///
    /// With this function implemented all the operations queued for an object will be executed
    ///
    /// # Arguments
    ///
    /// * `&mut self`: The object that contains a queue for with operations
    fn apply(&mut self) -> Result<&mut dyn GenericThumbnail, ApplyError>;

    /// Applies the queued operations of implementors of `GenericImage` and stores the result to the given `Target`
    ///
    /// With this function implemented all the operations queued for an object will be executed and the result will be stored.
    /// Returns `true` on succuess and `false` in case of an error.
    ///
    /// # Arguments
    ///
    /// * `self`: The object that contains a queue for with operations
    /// * `target`: The definition of the target image file as `&Target`
    fn apply_store(self, target: &Target) -> bool;

    /// Applies the queued operations of implementors of `GenericImage`, stores the result, and clears the queue
    ///
    /// With this function implemented all the operations queued for an object will be executed and the result will be stored.
    /// Unlike `apply_store()` this function does not consume the object and instead
    /// returns a `Result` with a `GenericThumbnail` on success and an `ApplyError` in case of an error
    ///
    /// # Arguments
    ///
    /// * `&mut self`: The object that contains a queue for with operations
    /// * `target`: The definition of the target image file as `&Target`
    fn apply_store_keep(
        &mut self,
        target: &Target,
    ) -> Result<&mut dyn GenericThumbnail, ApplyError>;

    /// Stores a `GenericImage`
    ///
    /// Returns `true` on success and `false` in case of an error.
    ///
    /// # Arguments
    ///
    /// * `self`: The `GenericImage` to be stored
    /// * `target`: The definition of the target image file as `&Target`
    ///
    /// # Attention
    /// If apply was not called before, the image will be saved unmodified.
    fn store(self, target: &Target) -> bool;

    /// Stores a `GenericImage`
    ///
    /// Unlike `store()` this function does not consume the object
    /// and instead returns a `Result` with a `GenericThumbnail` on success
    /// and an `ApplyError` in case of an error.
    ///
    /// # Arguments
    ///
    /// * `self`: The `GenericImage` to be stored
    /// * `target`: The definition of the target image file as `&Target`
    /// # Attention
    /// If apply was not called before, the image will be saved unmodified.
    fn store_keep(&mut self, target: &Target) -> Result<&mut dyn GenericThumbnail, ApplyError>;
}

/// The trait for the representation of the operations for a `GenericThumbnail`. These functions contain no logic.
/// They are used for queueing operations.
pub trait GenericThumbnailOperations {
    /// Representation of the resize-operation
    ///
    /// This function adds the resize operation to the queue of the oject represented by `&mut self`.
    /// It returns a `GenericThumbnail`.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which resize should be applied
    /// * `size` - operation options represented by the `Resize` enum
    fn resize(&mut self, size: Resize) -> &mut dyn GenericThumbnail;

    /// Representation of the resize-operation with custom filter
    ///
    /// This function adds the resize operation with a custom filter to the queue of the oject represented by `&mut self`.
    /// It returns a `GenericThumbnail`.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which resize should be applied
    /// * `size` - operation options represented by the `Resize` enum
    /// * `filter` - the custom filter represented by the `ResampleFilter` enum
    fn resize_filter(&mut self, size: Resize, filter: ResampleFilter) -> &mut dyn GenericThumbnail;

    /// Representation of the blur-operation
    ///
    /// This function adds the blur operation to the queue of the oject represented by `&mut self`.
    /// It returns a `GenericThumbnail`.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which blur should be applied
    /// * `sigma` - value of how much the image should be blurred. [Gaussian Blur] (https://en.wikipedia.org/wiki/Gaussian_blur)
    fn blur(&mut self, sigma: f32) -> &mut dyn GenericThumbnail;

    /// Representation of the brighten-operation
    ///
    /// This function adds the brighten operation to the queue of the oject represented by `&mut self`.
    /// It returns a `GenericThumbnail`.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which brighten should be applied
    /// * `value` - how much the image should be brightened. Positiv values will increase, negative values will decrease brightness.
    fn brighten(&mut self, value: i32) -> &mut dyn GenericThumbnail;

    /// Representation of the hue rotate operation
    ///
    /// This function adds the hue rotate operation to the queue of the oject represented by `&mut self`.
    /// It returns a `GenericThumbnail`.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which hue rotate should be applied
    /// * `degree` - value of degrees to rotate each pixel by
    fn huerotate(&mut self, degree: i32) -> &mut dyn GenericThumbnail;

    /// Representation of the contrast operation
    ///
    /// This function adds the contrast operation to the queue of the oject represented by `&mut self`.
    /// It returns a `GenericThumbnail`.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which contrast should be applied
    /// * `value` - Amount of adjusted contrast. Positiv values will increase, negative values will decrease contrast.
    fn contrast(&mut self, value: f32) -> &mut dyn GenericThumbnail;

    /// Representation of the unsharpen operation
    ///
    /// This function adds the unsharpen operation to the queue of the oject represented by `&mut self`.
    /// It returns a `GenericThumbnail`.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which unsharpen should be applied
    /// * `sigma` as amount to blur the 'DynamicImage'
    /// * `threshold` as control of how much to sharpen
    ///
    /// More information: [Digital unsharp masking](https://en.wikipedia.org/wiki/Unsharp_masking#Digital_unsharp_masking)
    fn unsharpen(&mut self, sigma: f32, threshold: i32) -> &mut dyn GenericThumbnail;

    /// Representation of the crop operation
    ///
    /// This function adds the crop operation to the queue of the oject represented by `&mut self`.
    /// It returns a `GenericThumbnail`.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which crop should be applied
    /// * `c` - Options for the operation represented by the `Crop` enum
    fn crop(&mut self, c: Crop) -> &mut dyn GenericThumbnail;

    /// Representation of the flip operation
    ///
    /// This function adds the crop operation to the queue of the oject represented by `&mut self`.
    /// It returns a `GenericThumbnail`.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which flip should be applied
    /// * `orientation` - Options for the operation represented by the `Orientation` enum
    fn flip(&mut self, orientation: Orientation) -> &mut dyn GenericThumbnail;

    /// Representation of the invert operation
    ///
    /// This function adds the invert operation to the queue of the oject represented by `&mut self`.
    /// It returns a `GenericThumbnail`.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which invert should be applied
    fn invert(&mut self) -> &mut dyn GenericThumbnail;

    fn exif(&mut self, metadata: Exif) -> &mut dyn GenericThumbnail;

    /// Representation of the draw-text operation
    ///
    /// This function adds the draw-text operation to the queue of the oject represented by `&mut self`.
    /// It returns a `GenericThumbnail`.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which draw-text should be applied
    /// * `text` - The text that should be drawn
    /// * `pos` - The position of the text represented by the `BoxPosition` enum
    fn text(&mut self, text: String, pos: BoxPosition) -> &mut dyn GenericThumbnail;

    /// Representation of the combine operation
    ///
    /// This function adds the combine operation to the queue of the oject represented by `&mut self`.
    /// It returns a `GenericThumbnail`.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which combine should be applied
    /// * `image` - The image that should be drawn on `self`
    /// * `pos` - The position of `image` represented by the `BoxPosition` enum
    fn combine(&mut self, image: StaticThumbnail, pos: BoxPosition) -> &mut dyn GenericThumbnail;

    /// Representation of the rotate operation
    ///
    /// This function adds the rotate operation to the queue of the oject represented by `&mut self`.
    /// It returns a `GenericThumbnail`.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which rotate should be applied
    /// * `rotation` - Options for the operation represented by the `Rotation` enum
    fn rotate(&mut self, rotation: Rotation) -> &mut dyn GenericThumbnail;
}

impl<T> GenericThumbnailOperations for T
where
    T: OperationContainer + GenericThumbnail,
{
    /// Representation of the resize operation without custom filter
    ///
    /// This function adds `ResizeOp` without the optional filter to the queue of a `GenericThumbnail` represented by `&mut self`.
    /// It returns itself after that.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which `ResizeOp` should be applied
    /// * `size` - operation options represented by the `Resize` enum
    ///
    /// # Panic
    ///
    /// This function won't panic
    fn resize(&mut self, size: Resize) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(ResizeOp::new(size, None)));
        self
    }

    /// Representation of the resize operation with custom filter
    ///
    /// This function adds `ResizeOp` with the optional filter to the queue of a `GenericThumbnail` represented by `&mut self`.
    /// It returns itself after that.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which `ResizeOp` should be applied
    /// * `size` - operation options represented by the `Resize` enum
    /// * `filter` - the custom filter represented by the `ResampleFilter` enum
    ///
    /// # Panic
    ///
    /// This function won't panic
    fn resize_filter(&mut self, size: Resize, filter: ResampleFilter) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(ResizeOp::new(size, Option::from(filter))));
        self
    }

    /// Representation of the blur operation
    ///
    /// This function adds `BlurOp` to the queue of a `GenericThumbnail` represented by `&mut self`.
    /// It returns itself after that.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which `BlurOp` should be applied
    /// * `sigma` - value of how much the image should be blurred. [Gaussian Blur] (https://en.wikipedia.org/wiki/Gaussian_blur)
    ///
    /// # Panic
    ///
    /// This function won't panic
    fn blur(&mut self, sigma: f32) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(BlurOp::new(sigma)));
        self
    }

    /// Representation of the brighten operation
    ///
    /// This function adds `BrightenOp` to the queue of a `GenericThumbnail` represented by `&mut self`.
    /// It returns itself after that.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which `BrightenOp` should be applied
    /// * `value` - how much the image should be brightened. Positiv values will increase, negative values will decrease brightness.
    ///
    /// # Panic
    ///
    /// This function won't panic
    fn brighten(&mut self, value: i32) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(BrightenOp::new(value)));
        self
    }

    /// Representation of the hue rotate operation
    ///
    /// This function adds `HuerotateOp` to the queue of a `GenericThumbnail` represented by `&mut self`.
    /// It returns itself after that.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which `HuerotateOp` should be applied
    /// * `degree` - value of degrees to rotate each pixel by
    ///
    /// # Panic
    ///
    /// This function won't panic
    fn huerotate(&mut self, degree: i32) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(HuerotateOp::new(degree)));
        self
    }

    /// Representation of the contrast operation
    ///
    /// This function adds `ContrastOp` to the queue of a `GenericThumbnail` represented by `&mut self`.
    /// It returns itself after that.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which `ContrastOp` should be applied
    /// * `value` - Amount of adjusted contrast. Positiv values will increase, negative values will decrease contrast.
    ///
    /// # Panic
    ///
    /// This function won't panic
    fn contrast(&mut self, value: f32) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(ContrastOp::new(value)));
        self
    }

    /// Representation of the unsharpen operation
    ///
    /// This function adds `UnsharpenOp` to the queue of a `GenericThumbnail` represented by `&mut self`.
    /// It returns itself after that.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which `UnsharpenOp` should be applied
    /// * `sigma` as amount to blur the 'DynamicImage'
    /// * `threshold` as control of how much to sharpen
    ///
    /// More information: [Digital unsharp masking](https://en.wikipedia.org/wiki/Unsharp_masking#Digital_unsharp_masking)
    ///
    /// # Panic
    ///
    /// This function won't panic
    fn unsharpen(&mut self, sigma: f32, threshold: i32) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(UnsharpenOp::new(sigma, threshold)));
        self
    }

    /// Representation of the crop operation
    ///
    /// This function adds `CropOp` to the queue of a `GenericThumbnail` represented by `&mut self`.
    /// It returns itself after that.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which `CropOp` should be applied
    /// * `c` - Options for the operation represented by the `Crop` enum
    ///
    /// # Panic
    ///
    /// This function won't panic
    fn crop(&mut self, c: Crop) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(CropOp::new(c)));
        self
    }

    /// Representation of the flip operation
    ///
    /// This function adds `FlipOp` to the queue of a `GenericThumbnail` represented by `&mut self`.
    /// It returns itself after that.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which `InvertOp` should be applied
    /// * `orientation` -  Options for the operation represented by the `Orientation` enum
    ///
    /// # Panic
    ///
    /// This function won't panic
    fn flip(&mut self, orientation: Orientation) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(FlipOp::new(orientation)));
        self
    }

    /// Representation of the invert operation
    ///
    /// This function adds `InvertOp` to the queue of a `GenericThumbnail` represented by `&mut self`.
    /// It returns itself after that.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which `InvertOp` should be applied
    ///
    /// # Panic
    ///
    /// This function won't panic
    fn invert(&mut self) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(InvertOp::new()));
        self
    }

    fn exif(&mut self, metadata: Exif) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(ExifOp::new(metadata)));
        self
    }

    /// Representation of the draw-text operation
    ///
    /// This function adds `TextOp` to the queue of a `GenericThumbnail` represented by `&mut self`.
    /// It returns itself after that.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which `TextOp` should be applied
    /// * `text` - The text that should be drawn on `self`
    /// * `pos` - The position of `text` represented by the `BoxPosition` enum
    ///
    /// # Panic
    ///
    /// This function won't panic
    fn text(&mut self, text: String, pos: BoxPosition) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(TextOp::new(text, pos)));
        self
    }

    /// Representation of the combine operation
    ///
    /// This function adds `CombineOp` to the queue of a `GenericThumbnail` represented by `&mut self`.
    /// It returns itself after that.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which `CombineOp` should be applied
    /// * `image` - The image that should be drawn on `self`
    /// * `pos` - The position of `image` represented by the `BoxPosition` enum
    ///
    /// # Panic
    ///
    /// This function won't panic
    fn combine(&mut self, image: StaticThumbnail, pos: BoxPosition) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(CombineOp::new(image, pos)));
        self
    }

    /// Representation of the rotate operation
    ///
    /// This function adds `RotateOp` to the queue of a `GenericThumbnail` represented by `&mut self`.
    /// It returns itself after that.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The object on which `RotateOp` should be applied
    /// * `rotation` -  Options for the operation represented by the `Rotation` enum
    ///
    /// # Panic
    ///
    /// This function won't panic
    fn rotate(&mut self, rotation: Rotation) -> &mut dyn GenericThumbnail {
        self.add_op(Box::new(RotateOp::new(rotation)));
        self
    }
}
