#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub use crate::generic::GenericThumbnail;
pub use crate::generic::{BoxPosition, Crop, Exif, Orientation, ResampleFilter, Resize};
pub use crate::thumbnail::SingleThumbnail;
pub use crate::thumbnail::StaticThumbnail;
pub use crate::thumbnail::Thumbnail;

pub mod errors;
pub mod generic;
pub mod thumbnail;
