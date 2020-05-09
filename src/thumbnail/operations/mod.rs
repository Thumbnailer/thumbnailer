use image::DynamicImage;
use std::fmt::Debug;

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

pub trait Operation: OperationClone + Debug + Send + Sync {
    fn apply(&self, image: &mut DynamicImage) -> bool;
}

pub trait OperationClone {
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
