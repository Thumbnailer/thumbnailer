use crate::errors::ApplyError;
use crate::generic::OperationContainer;
use crate::thumbnail::data::ThumbnailData;
use crate::{
    errors::FileError, generic::GenericThumbnail, thumbnail::operations::Operation, Target,
};
use image::io::Reader;
use image::DynamicImage;
use std::path::Path;
use std::path::PathBuf;

pub mod collection;
pub mod data;
pub mod operations;
pub mod static_thumb;

pub use collection::ThumbnailCollection;
pub use collection::ThumbnailCollectionBuilder;
pub use static_thumb::StaticThumbnail;

/// The `Thumbnail` type
///
/// Represents a single, modifiable image
#[derive(Debug)]
pub struct Thumbnail {
    /// The actual image data
    data: ThumbnailData,
    /// List of all operations to be applied to the image
    ops: Vec<Box<dyn Operation>>,
}

impl OperationContainer for Thumbnail {
    fn add_op(&mut self, op: Box<dyn Operation>) {
        self.ops.push(op);
    }
}

impl Thumbnail {
    /// Creates a new `Thumbnail` from the image at the given path
    ///
    /// The given path is queried whether it exists and if it can be opened.
    /// It it is then tried to determine the the format of the file, first by using the file extension
    /// or if that fails by actually looking into the file.
    ///
    /// If the file is found, a file handle is opened and store in the `Thumbnail` instance.
    /// The actual binary data is not yet loaded into memory. This happens when the operations are applied to the image.
    ///
    /// # Errors
    /// Can return a `FileError::NotFound` if the file could not be found
    /// Can return a `FileError::NotSupported` if the file is of an unsupported type
    /// Can return a `FileError::IoError` if an error occurred while accessing the file
    ///
    /// # Examples
    /// ```
    /// use std::path::{PathBuf, Path};
    /// use thumbnailer::Thumbnail;
    /// let thumb = match Thumbnail::load(Path::new("resources/tests/test.jpg").to_path_buf()) {
    ///     Ok(image) => image,
    ///     Err(_) => panic!("Could not load image!")
    /// };
    /// ```
    ///
    pub fn load(path: PathBuf) -> Result<Thumbnail, FileError> {
        Ok(Thumbnail {
            data: ThumbnailData::load(path)?,
            ops: vec![],
        })
    }

    /// This function creates and returns a new `Thumbnail` from an existing DynamicImage.
    ///
    /// # Arguments
    ///
    /// * `path_name` - A custom path for the new `Thumbnail`
    /// * `dynamic_image` - The `DynamicImage` that should be contained in the `Thumbnail`
    ///
    /// # Panic
    ///
    /// This function won't panic.
    pub fn from_dynamic_image(path_name: &str, dynamic_image: DynamicImage) -> Self {
        Thumbnail {
            data: ThumbnailData::from_dynamic_image(path_name, dynamic_image),
            ops: vec![],
        }
    }

    /// Turns into the internal `ThumbnailData` struct
    pub fn into_data(self) -> ThumbnailData {
        self.data
    }

    /// Gets the path stored in the `Thumbnail`. Usually the path from which the image was loaded.
    pub fn get_path(&self) -> PathBuf {
        self.data.get_path()
    }

    /// Clones an instance of `StaticThumbnail` from this instance.
    ///
    /// This first loads the actual image data to memory, to allow cloning in the first place.
    ///
    /// Returns `Some(StaticThumbnail)` if the image was loaded and therefore cloning was possible,
    /// otherwise returns `None`
    ///
    /// # Attention
    /// After calling this successfully the binary data for image will be in memory!
    ///
    pub fn clone_static_copy(&mut self) -> Option<StaticThumbnail> {
        let src_path = self.data.get_path();
        match self.get_dyn_image() {
            Ok(i) => Some(StaticThumbnail::new(src_path, i.clone())),
            Err(_) => None,
        }
    }

    /// Tries to load the binary data to memory and then clone the instance.
    ///
    /// This load the data first, because otherwise both instances would hold the same file handle,
    /// this could lead to weird problems we rather avoid.
    ///
    /// # Errors
    /// Can return a `FileError::NotSupported` if the file could not be loaded to memory
    ///
    ///
    pub fn try_clone_and_load(&mut self) -> Result<Thumbnail, FileError> {
        let ops = self.ops.clone();
        let image = self.data.try_clone_and_load()?;
        Ok(Thumbnail { data: image, ops })
    }

    /// Checks if the given path is a file which could be loaded
    ///
    /// * path: &Path - Path to check
    pub fn can_load(path: &Path) -> bool {
        if !path.is_file() {
            return false;
        }

        match Reader::open(path) {
            Err(_) => false,
            Ok(reader) => reader.format().is_some(),
        }
    }
    /// Loads the `DynamicImage` from the internal `ThumbnailData` instance
    ///
    /// # Errors
    /// Can return a `FileError::NotSupported` if the file could not be loaded to memory
    pub(crate) fn get_dyn_image(&mut self) -> Result<&mut image::DynamicImage, FileError> {
        self.data.get_dyn_image()
    }
}

impl GenericThumbnail for Thumbnail {
    fn apply(&mut self) -> Result<&mut dyn GenericThumbnail, ApplyError> {
        self.data.apply_ops_list(&self.ops)?;

        self.ops.clear();

        Ok(self)
    }

    fn apply_store(mut self, target: &Target) -> Result<Vec<PathBuf>, ApplyError> {
        self.apply()?;
        self.store(target)
    }

    fn apply_store_keep(&mut self, target: &Target) -> Result<Vec<PathBuf>, ApplyError> {
        self.apply()?;
        self.store_keep(target)
    }

    fn store(self, target: &Target) -> Result<Vec<PathBuf>, ApplyError> {
        match target.store(&mut self.into_data(), None) {
            Ok(files) => Ok(files),
            Err(err) => Err(ApplyError::StoreError(err)),
        }
    }

    fn store_keep(&mut self, target: &Target) -> Result<Vec<PathBuf>, ApplyError> {
        match target.store(&mut self.data, None) {
            Ok(files) => Ok(files),
            Err(err) => Err(ApplyError::StoreError(err)),
        }
    }
}
