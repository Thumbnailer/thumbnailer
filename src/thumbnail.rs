use image::{DynamicImage, image_dimensions, ImageResult, ImageError, io::Reader, GenericImageView};
use crate::{
    generic::{GenericThumbnail, Crop, Exif, Resize, BoxPosition, Orientation, ResampleFilter},
    thumbnail::operations::{Operation, ResizeOp},
    errors::{FileNotFoundError, FileNotSupportedError, FileError, InternalError},
    errors
};
use std::{
    io,
    path::Path,
    error::Error,
    io::{Read, BufReader},
    fs::File
};

mod operations;

pub struct StaticThumbnail {
    image: DynamicImage,
}

impl StaticThumbnail {
    pub fn as_dyn(&self) -> &DynamicImage {
        &self.image
    }
}

pub trait SingleThumbnail : GenericThumbnail {
    fn as_static_copy(&self) -> &mut StaticThumbnail;
}

pub struct Thumbnail<'a> {
    path: &'a Path,
    reader: Reader<BufReader<File>>,
    height: u32,
    width: u32,
    image: Option<DynamicImage>,
    ops: Vec<&'a dyn Operation>
}

impl Thumbnail<'_> {
    pub fn load(path: &Path) -> Result<Thumbnail, FileError> {
        if !path.is_file() {
            return Err(FileError::NotFound(FileNotFoundError {
                path,
            }));
        }

        // This unfortunately needs to be mutable, because we may need to overwrite it with itself,
        // because a method call borrows self and then returns it again within a Result
        let mut reader = match Reader::open(path) {
            Ok(reader) => reader,
            Err(error) => return Err(FileError::IoError(error)),
        };

        if reader.format().is_none() {
            // with_guessed_format() returns Result<Self>,
            // to keep ownership of reader we need to extract it from the result again
            reader = match reader.with_guessed_format() {
                Err(error) => return return Err(FileError::IoError(error)),
                Ok(reader) => reader
            };

            if reader.format().is_none(){
                return Err(FileError::NotSupported(FileNotSupportedError {
                    path,
                }));
            }
        }

        Ok(Thumbnail {
            path,
            reader,
            height: 0,
            width: 0,
            image: None,
            ops: vec![]
        })
    }

    pub fn can_load(path: &Path) -> bool {
        if !path.is_file() {
            return false;
        }

        match Reader::open(path) {
            Err(_) => return false,
            Ok(reader) => match reader.format() {
                Some(_) => true,
                None => false
            },
        }
    }

    fn get_dyn_image<'a>(mut self) -> Result<image::DynamicImage, InternalError<'a>> {
        if self.image.is_none() {
            let image = self.reader.decode()?;
            let (width, height) = match self.image {
                Some(i) => i.dimensions(),
                None => (0, 0)
            };
            self.width = width;
            self.height = height;
            self.image = Option::from(image.clone());
        }

        let image = self.image.ok_or_else(|| InternalError::UnknownError(errors::UnknownError))?;

        Ok(image)
    }
}


