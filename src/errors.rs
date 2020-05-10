use crate::thumbnail::operations::Operation;
use globwalk::GlobError;
use std::error::Error;
use std::fmt::Formatter;
use std::path::PathBuf;
use std::{fmt, io};

#[derive(Debug)]
pub enum FileError {
    GlobError(io::Error),
    NotFound(FileNotFoundError),
    NotSupported(FileNotSupportedError),
    IoError(io::Error),
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

#[derive(Debug, Clone)]
pub struct FileNotFoundError {
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
#[derive(Debug)]
pub struct FileNotSupportedError {
    path: PathBuf,
}

impl FileNotSupportedError {
    pub fn new(path: PathBuf) -> Self {
        FileNotSupportedError { path }
    }

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

pub enum ApplyError {
    OperationError(OperationError),
    StoreError(FileError),
    CollectionError(CollectionError),
    LoadingImageError(FileError),
}

#[derive(Debug, Clone)]
pub enum OperationErrorInfo {
    CoordinatesOutOfRange,
    RgbaImageConversionFailure,
    RgbImageConversionFailure,
    FontLoadError,
}

#[derive(Debug, Clone)]
pub struct OperationError {
    op: Box<dyn Operation>,
    info: OperationErrorInfo,
}

impl OperationError {
    pub fn new(op: Box<dyn Operation>, info: OperationErrorInfo) -> Self {
        OperationError { op: op, info: info }
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

pub struct CollectionError {
    paths: Vec<PathBuf>,
    store_errors: Vec<FileError>,
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

    pub fn get_paths(&self) -> &Vec<PathBuf> {
        &self.paths
    }

    pub fn get_store_errors(&self) -> &Vec<FileError> {
        &self.store_errors
    }

    pub fn get_operation_errors(&self) -> &Vec<OperationError> {
        &self.operation_errors
    }
}
