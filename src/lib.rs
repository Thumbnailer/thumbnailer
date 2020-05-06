#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub use crate::generic::GenericThumbnail;
pub use crate::generic::{Resize, BoxPosition, Crop, Orientation, Exif, ResampleFilter};
pub use crate::thumbnail::StaticThumbnail;
pub use crate::thumbnail::SingleThumbnail;

mod generic;
mod thumbnail;