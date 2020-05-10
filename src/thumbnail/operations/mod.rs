use image::DynamicImage;
use std::fmt::Debug;

// Include all submodules
pub mod blur;
pub mod brighten;
pub mod combine;
pub mod contrast;
pub mod crop;
pub mod exif;
pub mod flip;
pub mod huerotate;
pub mod invert;
pub mod resize;
pub mod text;
pub mod unsharpen;

pub use blur::BlurOp;
pub use brighten::BrightenOp;
pub use combine::CombineOp;
pub use contrast::ContrastOp;
pub use crop::CropOp;
pub use exif::ExifOp;
pub use flip::FlipOp;
pub use huerotate::HuerotateOp;
pub use invert::InvertOp;
pub use resize::ResizeOp;
pub use text::TextOp;
pub use unsharpen::UnsharpenOp;

/// The `Operation` trait.
///
/// This trait allows the dynamic implementation of the actual methods which apply modifications to the image.
/// Passing the image to the apply function should perform the desired modifications to it.
pub trait Operation: OperationClone + Debug + Send + Sync {
    fn apply(&self, image: &mut DynamicImage) -> bool;
}

pub trait OperationClone {
    /// Clones the `Operation` instance into a `Box<dyn Operation>`
    fn box_clone(&self) -> Box<dyn Operation>;
}

impl<T> OperationClone for T
where
    T: 'static + Operation + Clone,
{
    fn box_clone(&self) -> Box<dyn Operation> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Operation> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}
