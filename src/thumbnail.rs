use image::{DynamicImage, io::Reader, GenericImageView};
use crate::{
    generic::{GenericThumbnail, Crop, Exif, Resize, BoxPosition, Orientation, ResampleFilter},
    thumbnail::operations::{Operation},
    errors::{FileNotFoundError, FileNotSupportedError, FileError, InternalError},
    errors
};
use std::{
    path::Path,
    io::BufReader,
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
    fn to_static_copy(self) -> Option<StaticThumbnail>;
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


impl SingleThumbnail for Thumbnail<'_> {
    fn to_static_copy(self) -> Option<StaticThumbnail> {
        match self.get_dyn_image() {
            Ok(i) => Some(StaticThumbnail {
                image: i.clone(),
            }),
            Err(_) => None,
        }
    }
}

impl GenericThumbnail for Thumbnail<'_> {
    fn resize(&mut self, size: Resize) -> &mut GenericThumbnail {
        unimplemented!()
    }

    fn resize_filter(&mut self, size: Resize, filter: ResampleFilter) -> &mut GenericThumbnail {
        unimplemented!()
    }

    fn blur(&mut self, sigma: f32) -> &mut GenericThumbnail {
        unimplemented!()
    }

    fn brighten(&mut self, value: i32) -> &mut GenericThumbnail {
        unimplemented!()
    }

    fn huerotate(&mut self, degree: i32) -> &mut GenericThumbnail {
        unimplemented!()
    }

    fn contrast(&mut self, value: f32) -> &mut GenericThumbnail {
        unimplemented!()
    }

    fn unsharpen(&mut self, sigma: f32, threshold: u32) -> &mut GenericThumbnail {
        unimplemented!()
    }

    fn crop(&mut self, c: Crop) -> &mut GenericThumbnail {
        unimplemented!()
    }

    fn flip(&mut self, orientation: Orientation) -> &mut GenericThumbnail {
        unimplemented!()
    }

    fn invert(&mut self) -> &mut GenericThumbnail {
        unimplemented!()
    }

    fn exif(&mut self, metadata: Exif) -> &mut GenericThumbnail {
        unimplemented!()
    }

    fn text(&mut self, text: String, pos: BoxPosition) -> &mut GenericThumbnail {
        unimplemented!()
    }

    fn combine(&mut self, image: &StaticThumbnail, pos: BoxPosition) -> &mut GenericThumbnail {
        unimplemented!()
    }
}
