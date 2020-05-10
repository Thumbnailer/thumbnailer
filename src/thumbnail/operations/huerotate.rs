pub use crate::errors::OperationError;
use crate::thumbnail::operations::Operation;
use image::DynamicImage;

#[derive(Debug, Copy, Clone)]
pub struct HuerotateOp {
    degree: i32,
}

impl HuerotateOp {
    pub fn new(degree: i32) -> Self {
        HuerotateOp { degree }
    }
}

impl Operation for HuerotateOp {
    fn apply(&self, image: &mut DynamicImage) -> Result<(), OperationError>
    where
        Self: Sized,
    {
        *image = image.huerotate(self.degree);
        Ok(())
    }
}
