use crate::thumbnail::operations::Operation;
use globwalk::GlobError;
use std::error::Error;
use std::fmt::Formatter;
use std::path::PathBuf;
use std::{fmt, io};

/// Error type while interacting with files or the filesystem
#[derive(Debug)]
pub enum FileError {
    /// Error while parsing the glob
    GlobError(io::Error),
    /// Given file could not be found
    NotFound(FileNotFoundError),
    /// Given file cannot be decoded
    NotSupported(FileNotSupportedError),
    /// General io error
    IoError(io::Error),
    /// Error could not be correctly determined
    UnknownError,
}

impl std::convert::From<globwalk::GlobError> for FileError {
    fn from(err: GlobError) -> Self {
        FileError::GlobError(io::Error::from(err))
    }
}

impl std::convert::From<std::io::Error> for FileError {
    fn from(err: io::Error) -> Self {
        FileError::IoError(err)
    }
}

/// The `FileNotFoundError` type. Provides information for FileError::NotFound
#[derive(Debug, Clone)]
pub struct FileNotFoundError {
    /// Path that could not be found
    pub path: PathBuf,
}

impl fmt::Display for FileNotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "File could not be found at path: {}",
            self.path.display()
        )
    }
}

impl Error for FileNotFoundError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
/// The `FileNotSupportedError` type. Provides information for FileError::NotSupported
#[derive(Debug)]
pub struct FileNotSupportedError {
    /// Path of the file that could not be decoded.
    path: PathBuf,
}

impl FileNotSupportedError {
    /// Creates a new `FileNotSupportedError`
    pub fn new(path: PathBuf) -> Self {
        FileNotSupportedError { path }
    }
    /// Gets the path of the file that caused the error
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
}

impl fmt::Display for FileNotSupportedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "File is not of a supported type: {}",
            self.path.display()
        )
    }
}

impl Error for FileNotSupportedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
/// Error type that can occur while applying operations to a GenericThumbnail instance or storing it.
///
///
///
pub enum ApplyError {
    OperationError(OperationError),
    StoreError(FileError),
    CollectionError(CollectionError),
    LoadingImageError(FileError),
}

/// Error types used as additional information for `OperationError`
#[derive(Debug, Clone)]
pub enum OperationErrorInfo {
    /// Some coordinates given to a function are not supported, for example when referencing coordinates that are not inside an image.
    CoordinatesOutOfRange,
    /// The Conversion of a `DynamicImage` to an `ImageBuffer` was not successful
    ImageBufferConversionFailure,
    /// A font could not be loaded
    FontLoadError,
}

/// Error that can occur while applying a single operation on a GenericThumbnail item
#[derive(Debug, Clone)]
pub struct OperationError {
    /// Operation that failed
    op: Box<dyn Operation>,
    /// Additional information on why it failed
    info: OperationErrorInfo,
}

impl OperationError {
    pub fn new(op: Box<dyn Operation>, info: OperationErrorInfo) -> Self {
        OperationError { op, info }
    }
}

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Applying operation failed")
    }
}

impl Error for OperationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

/// Error that can occur while applying or storing a GenericThumbnail that contains multiple images.
///
pub struct CollectionError {
    /// Output file paths that weren't affected by the error and were successfully stored
    paths: Vec<PathBuf>,
    /// List of all store errors that occurred while storing each item
    store_errors: Vec<FileError>,
    /// List of all operations errors that occurred while applying operations to each item
    operation_errors: Vec<OperationError>,
}

impl CollectionError {
    pub fn new(
        paths: Vec<PathBuf>,
        store_errors: Vec<FileError>,
        operation_errors: Vec<OperationError>,
    ) -> Self {
        CollectionError {
            paths,
            store_errors,
            operation_errors,
        }
    }
    /// Gets all paths that were successful despite errors occurring
    pub fn get_paths(&self) -> &Vec<PathBuf> {
        &self.paths
    }
    /// Gets all StoreErrors that occurred while storing each item
    pub fn get_store_errors(&self) -> &Vec<FileError> {
        &self.store_errors
    }
    /// Gets all OperationErrors that occurred while applying all operations to each item
    pub fn get_operation_errors(&self) -> &Vec<OperationError> {
        &self.operation_errors
    }
}
