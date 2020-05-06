use image::{DynamicImage, io::Reader, ImageFormat};
use crate::{
    generic::{GenericThumbnail, Crop, Exif, Resize, BoxPosition, Orientation, ResampleFilter},
    thumbnail::operations::{Operation},
    errors::{FileNotFoundError, FileNotSupportedError, FileError, InternalError},
    errors,
};
use std::{
    path::Path,
    io::BufReader,
    fs::File,
};
use crate::thumbnail::ImageData::Image;

mod operations;

#[derive(Clone)]
pub struct StaticThumbnail {
    image: DynamicImage,
}

impl StaticThumbnail {
    pub fn as_dyn(&self) -> &DynamicImage {
        &self.image
    }
}

pub trait SingleThumbnail: GenericThumbnail {
    fn to_static_copy(&mut self) -> Option<StaticThumbnail>;
}
//TODO: #[derive(Clone)]
enum ImageData {
    File(File, ImageFormat),
    Image(DynamicImage)
}

//TODO: #[derive(Clone)]
pub struct Thumbnail<'a> {
    path: &'a Path,
    height: u32,
    width: u32,
    image: ImageData,
    ops: Vec<Box<dyn Operation>>,
}

impl Thumbnail<'_> {
    pub fn load(path: &Path) -> Result<Thumbnail, FileError> {
        if !path.is_file() {
            return Err(FileError::NotFound(FileNotFoundError {
                path,
            }));
        }

        let file= match File::open(path) {
            Ok(f) => f,
            Err(e) => return Err(FileError::IoError(e))
        };

        let buffer = BufReader::new(file);

        // This unfortunately needs to be mutable, because we may need to overwrite it with itself,
        // because a method call borrows self and then returns it again within a Result
        let mut reader = Reader::new(buffer);

        let format  = match reader.format() {
            Some(f) => f,
            None => {
                // with_guessed_format() returns Result<Self>,
                // to keep ownership of reader we need to extract it from the result again
                reader = match reader.with_guessed_format() {
                    Err(error) => return Err(FileError::IoError(error)),
                    Ok(reader) => reader
                };

                match reader.format() {
                    Some(f) => f,
                    None => return Err(FileError::NotSupported(FileNotSupportedError {
                        path,
                    }))
                }
            }
        };

        Ok(Thumbnail {
            path,
            image: ImageData::File(reader.into_inner().into_inner(), format),
            height: 0,
            width: 0,
            ops: vec![],
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

    fn get_dyn_image<'a>(&mut self) -> Result<&mut image::DynamicImage, InternalError> {

        if let (ImageData::File(file, _)) = &self.image {
            let reader = Reader::new(BufReader::new(file));
            self.image = ImageData::Image(reader.decode()?);
        }

        return match  &mut self.image {
            ImageData::Image(image) => Ok(image),
            ImageData::File(file, _) => Err(InternalError::UnknownError(errors::UnknownError))
        };
    }

    fn assert_dynamic_image_loaded(&mut self) -> bool {
        match self.get_dyn_image() {
            Ok(_) => true,
            Err(_) => false
        }
    }
}


impl SingleThumbnail for Thumbnail<'_> {
    fn to_static_copy(&mut self) -> Option<StaticThumbnail> {
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
