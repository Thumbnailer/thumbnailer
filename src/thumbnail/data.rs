use crate::errors;
use crate::errors::{
    ApplyError, FileError, FileNotFoundError, FileNotSupportedError, InternalError, OperationError,
};
use crate::thumbnail::operations::Operation;
use image::io::Reader;
use image::{DynamicImage, ImageFormat};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub(crate) enum ImageData {
    File(File, ImageFormat),
    Image(DynamicImage),
}

pub struct ThumbnailData {
    path: PathBuf,
    image: ImageData,
}

impl ThumbnailData {
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

    pub(crate) fn new(path: PathBuf, image: ImageData) -> Self {
        ThumbnailData { path, image }
    }

    pub(crate) fn get_dyn_image<'a>(&mut self) -> Result<&mut image::DynamicImage, InternalError> {
        if let ImageData::File(file, _) = &self.image {
            let reader = Reader::new(BufReader::new(file));
            self.image = ImageData::Image(reader.decode()?);
        }

        return match &mut self.image {
            ImageData::Image(image) => Ok(image),
            ImageData::File(_, _) => Err(InternalError::UnknownError(errors::UnknownError)),
        };
    }

    pub fn try_clone_and_load(&mut self) -> Result<ThumbnailData, FileError> {
        let path = self.path.clone();
        let image_data = self.get_dyn_image()?;
        Ok(ThumbnailData {
            path,
            image: ImageData::Image(image_data.clone()),
        })
    }

    fn assert_dynamic_image_loaded(&mut self) -> bool {
        self.get_dyn_image().is_ok()
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

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
