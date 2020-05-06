use std::path::Path;
use std::{fmt, io};
use std::fmt::{Formatter};
use std::error::Error;
use image::ImageError;


#[derive(Debug)]
pub enum FileError<'a> {
    NotFound(FileNotFoundError<'a>),
    NotSupported(FileNotSupportedError<'a>),
    IoError(io::Error),
}

pub(crate) enum InternalError<'a> {
    FileError(FileError<'a>),
    ImageError(ImageError),
    UnknownError(UnknownError),
}

impl std::convert::From<image::error::ImageError> for InternalError<'_> {
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
pub struct FileNotFoundError<'a> {
    pub path: &'a Path,
}

impl fmt::Display for FileNotFoundError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "File could not be found at path: {}", self.path.display())
    }
}

impl Error for FileNotFoundError<'_> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
#[derive(Debug, Clone)]
pub struct FileNotSupportedError<'a> {
    pub path: &'a Path,
}

impl fmt::Display for FileNotSupportedError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "File is not of a supported type: {}", self.path.display())
    }
}

impl Error for FileNotSupportedError<'_> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}