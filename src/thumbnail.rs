use crate::errors::{ApplyError, OperationError};
use crate::generic::OperationContainer;
use crate::{
    errors,
    errors::{FileError, FileNotFoundError, FileNotSupportedError, InternalError},
    generic::GenericThumbnail,
    thumbnail::operations::Operation,
};
use image::{io::Reader, DynamicImage, ImageFormat};
use std::path::PathBuf;
use std::{fs::File, io::BufReader, path::Path};

pub mod operations;

#[derive(Clone)]
pub struct StaticThumbnail {
    src_path: PathBuf,
    image: DynamicImage,
}

impl StaticThumbnail {
    pub fn as_dyn(&self) -> &DynamicImage {
        &self.image
    }

    pub fn get_width(&self) -> u32 {
        match self.as_dyn().as_rgb8() {
            Some(rgb_image) => rgb_image.width(),
            None => 0,
        }
    }

    pub fn get_height(&self) -> u32 {
        match self.as_dyn().as_rgb8() {
            Some(rgb_image) => rgb_image.height(),
            None => 0,
        }
    }
    pub fn get_src_path(&self) -> PathBuf {
        self.src_path.clone()
    }
}

enum ImageData {
    File(File, ImageFormat),
    Image(DynamicImage),
}

pub struct Thumbnail {
    path: PathBuf,
    image: ImageData,
    ops: Vec<Box<dyn Operation>>,
}

impl OperationContainer for Thumbnail {
    fn add_op(&mut self, op: Box<dyn Operation>) {
        self.ops.push(op);
    }
}

impl Thumbnail {
    pub fn load(path: PathBuf) -> Result<Thumbnail, FileError> {
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

        Ok(Thumbnail {
            path: path.to_path_buf(),
            image: ImageData::File(reader.into_inner().into_inner(), format),
            ops: vec![],
        })
    }

    pub fn to_static_copy(&mut self) -> Option<StaticThumbnail> {
        let src_path = self.path.clone();
        match self.get_dyn_image() {
            Ok(i) => Some(StaticThumbnail {
                src_path,
                image: i.clone(),
            }),
            Err(_) => None,
        }
    }
    pub fn try_clone_and_load(&mut self) -> Result<Thumbnail, FileError> {
        let path = self.path.clone();
        let ops = self.ops.clone();
        let image_data = self.get_dyn_image()?;
        Ok(Thumbnail {
            path,
            image: ImageData::Image(image_data.clone()),
            ops,
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
                None => false,
            },
        }
    }

    fn get_dyn_image<'a>(&mut self) -> Result<&mut image::DynamicImage, InternalError> {
        if let ImageData::File(file, _) = &self.image {
            let reader = Reader::new(BufReader::new(file));
            self.image = ImageData::Image(reader.decode()?);
        }

        return match &mut self.image {
            ImageData::Image(image) => Ok(image),
            ImageData::File(_, _) => Err(InternalError::UnknownError(errors::UnknownError)),
        };
    }

    fn assert_dynamic_image_loaded(&mut self) -> bool {
        self.get_dyn_image().is_ok()
    }

    pub(crate) fn apply_ops_list(
        &mut self,
        ops: &Vec<Box<dyn Operation>>,
    ) -> Result<(), ApplyError> {
        if !self.assert_dynamic_image_loaded() {
            return Err(ApplyError::LoadingImageError);
        }

        if let ImageData::Image(image) = &mut self.image {
            for operation in ops {
                if !operation.apply(image) {
                    return Err(ApplyError::OperationError(OperationError::new(
                        operation.clone(),
                    )));
                }
            }
        }

        Ok(())
    }
}

impl GenericThumbnail for Thumbnail {
    fn apply(&mut self) -> Result<&mut dyn GenericThumbnail, ApplyError> {
        self.assert_dynamic_image_loaded();

        if let ImageData::Image(image) = &mut self.image {
            for operation in &self.ops {
                if !operation.apply(image) {
                    return Err(ApplyError::OperationError(OperationError::new(
                        operation.clone(),
                    )));
                }
            }
        }

        self.ops.clear();

        Ok(self)
    }
}
