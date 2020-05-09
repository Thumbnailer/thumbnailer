use crate::errors::ApplyError;
use crate::generic::OperationContainer;
use crate::thumbnail::data::ThumbnailData;
use crate::{
    errors::{FileError, InternalError},
    generic::GenericThumbnail,
    thumbnail::operations::Operation,
    Target,
};
use image::{io::Reader, DynamicImage};
use std::path::Path;
use std::path::PathBuf;

pub mod data;
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

pub struct Thumbnail {
    data: ThumbnailData,
    ops: Vec<Box<dyn Operation>>,
}

impl OperationContainer for Thumbnail {
    fn add_op(&mut self, op: Box<dyn Operation>) {
        self.ops.push(op);
    }
}

impl Thumbnail {
    pub fn load(path: PathBuf) -> Result<Thumbnail, FileError> {
        Ok(Thumbnail {
            data: ThumbnailData::load(path)?,
            ops: vec![],
        })
    }

    pub fn into_data(self) -> ThumbnailData {
        self.data
    }

    pub fn get_path(&self) -> PathBuf {
        self.data.get_path()
    }

    pub fn clone_static_copy(&mut self) -> Option<StaticThumbnail> {
        let src_path = self.data.get_path();
        match self.get_dyn_image() {
            Ok(i) => Some(StaticThumbnail {
                src_path,
                image: i.clone(),
            }),
            Err(_) => None,
        }
    }

    pub fn try_clone_and_load(&mut self) -> Result<Thumbnail, FileError> {
        let ops = self.ops.clone();
        let image = self.data.try_clone_and_load()?;
        Ok(Thumbnail { data: image, ops })
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

    pub(crate) fn get_dyn_image<'a>(&mut self) -> Result<&mut image::DynamicImage, InternalError> {
        return self.data.get_dyn_image();
    }

    fn assert_dynamic_image_loaded(&mut self) -> bool {
        self.get_dyn_image().is_ok()
    }
}

impl GenericThumbnail for Thumbnail {
    fn apply(&mut self) -> Result<&mut dyn GenericThumbnail, ApplyError> {
        self.data.apply_ops_list(&self.ops)?;

        self.ops.clear();

        Ok(self)
    }

    fn apply_store(mut self, target: &Target) -> bool {
        if self.apply().is_ok() {
            self.store(target)
        } else {
            false
        }
    }

    fn apply_store_keep(
        &mut self,
        target: &Target,
    ) -> Result<&mut dyn GenericThumbnail, ApplyError> {
        self.apply()?;
        self.store_keep(target)?;
        Ok(self)
    }

    fn store(self, target: &Target) -> bool {
        return match target.store(&mut self.into_data(), None) {
            Ok(_) => true,
            Err(_) => false,
        };
    }

    fn store_keep(&mut self, target: &Target) -> Result<&mut dyn GenericThumbnail, ApplyError> {
        match target.store(&mut self.data, None) {
            Ok(_) => Ok(self),
            Err(e) => Err(ApplyError::LoadingImageError),
        }
    }
}
