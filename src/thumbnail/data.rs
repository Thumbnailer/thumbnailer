use crate::errors;
use crate::errors::{
    ApplyError, FileError, FileNotFoundError, FileNotSupportedError, InternalError, OperationError,
};
use crate::thumbnail::operations::Operation;
use image::io::Reader;
use image::{DynamicImage, ImageFormat};
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

/// The `ImageData` type
///
/// This type either holds a file handle with a format, the file has been determined to be,
/// or the actual image data in memory.
/// This allows to dynamically load the data only then when it's being used.
/// Before that only a reference to the image is store, from which the data will be read.
pub(crate) enum ImageData {
    /// File which holds a file handle and the files image format information
    File(File, ImageFormat),
    /// Image data in memory
    Image(DynamicImage),
}

impl fmt::Debug for ImageData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ImageData::File(file, format) => write!(f, "ImageData::File( {:?}, {:?}", file, format),
            ImageData::Image(_) => write!(f, "ImageData::Image(DynamicImage)"),
        }
    }
}

/// The `ThumbnailData` type
///
/// Holds the source path of the file and the variable image data in the form of a `ImageData` instance.
#[derive(Debug)]
pub struct ThumbnailData {
    /// Path from where the file was loaded
    path: PathBuf,
    /// The image data
    image: ImageData,
}

impl ThumbnailData {
    /// Creates a new `ThumbnailData` from the given file path
    ///
    /// * path: PathBuf - The path to the image file
    ///
    /// # Errors
    /// Returns a FileError of there was a problem opening the file.
    pub(crate) fn load(path: PathBuf) -> Result<ThumbnailData, FileError> {
        if !path.is_file() {
            return Err(FileError::NotFound(FileNotFoundError { path }));
        }

        let file = match File::open(path.clone()) {
            Ok(f) => f,
            Err(e) => return Err(FileError::IoError(e)),
        };

        let buffer = BufReader::new(file);

        // This unfortunately needs to be mutable, because we may need to overwrite it with itself,
        // because a method call borrows self and then returns it again within a Result
        let mut reader = Reader::new(buffer);

        let format = match reader.format() {
            Some(f) => f,
            None => {
                // with_guessed_format() returns Result<Self>,
                // to keep ownership of reader we need to extract it from the result again
                reader = match reader.with_guessed_format() {
                    Err(error) => return Err(FileError::IoError(error)),
                    Ok(reader) => reader,
                };

                match reader.format() {
                    Some(f) => f,
                    None => return Err(FileError::NotSupported(FileNotSupportedError { path })),
                }
            }
        };

        Ok(ThumbnailData {
            path: path.to_path_buf(),
            image: ImageData::File(reader.into_inner().into_inner(), format),
        })
    }

    /// Creates a new `ThumbnailData` from the given ImageData.
    ///
    /// While this takes a path, this is just additional information, nothing is read from that path.
    pub(crate) fn new(path: PathBuf, image: ImageData) -> Self {
        ThumbnailData { path, image }
    }

    /// Gets the `DynamicImage` stored inside a `ImageData` instance.
    ///
    /// If the dynamic image has not yet been loaded,
    /// and the `ImageData` instance still contains the file handle,
    /// the data will be loaded and the `ImageData` instance will be converted, if possible.
    ///
    /// # Errors
    /// Returns an InternalError of there was a problem loading the image data from the file system
    /// or accessing the `DynamicImage` instance
    pub(crate) fn get_dyn_image<'a>(&mut self) -> Result<&mut image::DynamicImage, InternalError> {
        if let ImageData::File(file, format) = &self.image {
            let mut reader = Reader::new(BufReader::new(file));
            reader.set_format(*format);
            self.image = ImageData::Image(reader.decode()?);
        }

        return match &mut self.image {
            ImageData::Image(image) => Ok(image),
            ImageData::File(_, _) => Err(InternalError::UnknownError(errors::UnknownError)),
        };
    }

    /// Ensures the image data is in memory then clones the `ThumbnailData` instance
    ///
    /// As `ImageData` initially only holds a file handle, cloning would be tricky,
    /// as multiple instances of the same file handle could lead to weird situations.
    /// Therefore ThumbnailData only allows cloning if the image data is already in memory,
    /// to ensure that, this methods loads the data into memory before it clones.
    ///
    /// # Errors
    /// Returns a `FileError` if an error occurs while loading the data from the disk
    pub fn try_clone_and_load(&mut self) -> Result<ThumbnailData, FileError> {
        let path = self.path.clone();
        let image_data = self.get_dyn_image()?;
        Ok(ThumbnailData {
            path,
            image: ImageData::Image(image_data.clone()),
        })
    }
    /// Ensures that the image data is loaded into memory.
    ///
    /// This checks whether the image data is already loaded to memory. If not it loads it.
    /// If the loading fails it returns false.
    fn assert_dynamic_image_loaded(&mut self) -> bool {
        self.get_dyn_image().is_ok()
    }

    /// Gets the original path of the image (from where it has been loaded)
    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    /// Takes a vector of `Operation` objects and applies each to the image.
    ///
    /// This passes the underlying `DynamicImage` to the `Operation::apply`
    /// method of each given `Operation` object.
    ///
    /// # Errors
    /// Returns a `ApplyError` if a operation fails.
    pub(crate) fn apply_ops_list(
        &mut self,
        ops: &Vec<Box<dyn Operation>>,
    ) -> Result<&mut Self, ApplyError> {
        if !self.assert_dynamic_image_loaded() {
            return Err(ApplyError::LoadingImageError);
        }

        if let Ok(image) = &mut self.get_dyn_image() {
            for operation in ops {
                if !operation.apply(image) {
                    return Err(ApplyError::OperationError(OperationError::new(
                        operation.clone(),
                    )));
                }
            }
        }
        Ok(self)
    }
}
