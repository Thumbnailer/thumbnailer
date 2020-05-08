use image::ImageError;
use std::error::Error;
use std::fmt::Formatter;
use std::path::PathBuf;
use std::{fmt, io};

#[derive(Debug)]
pub enum FileError {
    NotFound(FileNotFoundError),
    NotSupported(FileNotSupportedError),
    IoError(io::Error),
}

pub(crate) enum InternalError {
    ImageError(ImageError),
    UnknownError(UnknownError),
}

impl std::convert::From<image::error::ImageError> for InternalError {
    fn from(err: ImageError) -> Self {
        InternalError::ImageError(err)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct UnknownError;
impl fmt::Display for UnknownError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown error")
    }
}

impl Error for UnknownError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
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
#[derive(Debug, Clone)]
pub struct FileNotSupportedError {
    pub path: PathBuf,
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
