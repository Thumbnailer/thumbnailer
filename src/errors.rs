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
    UnknownError(UnknownError),
}

impl std::convert::From<InternalError> for FileError {
    fn from(err: InternalError) -> Self {
        match err {
            InternalError::UnknownError(err) => FileError::UnknownError(err),
            InternalError::ImageError(err) => match err {
                ImageError::IoError(err) => FileError::IoError(err),
                _ => FileError::UnknownError(UnknownError),
            },
        }
    }
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
pub struct UnknownError;
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

pub enum CollectionError {
    FileError(FileError),
    GlobError(globwalk::GlobError),
}

impl std::convert::From<FileError> for CollectionError {
    fn from(error: FileError) -> Self {
        CollectionError::FileError(error)
    }
}

impl std::convert::From<globwalk::GlobError> for CollectionError {
    fn from(error: globwalk::GlobError) -> Self {
        CollectionError::GlobError(error)
    }
}
