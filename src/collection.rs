use crate::errors::{ApplyError, CollectionError};
use crate::generic::OperationContainer;
use crate::thumbnail::data::ThumbnailData;
use crate::thumbnail::operations::Operation;
use crate::{GenericThumbnail, Target, Thumbnail};
use std::path::Path;

pub struct ThumbnailCollectionBuilder {
    collection: ThumbnailCollection,
}

impl ThumbnailCollectionBuilder {
    pub fn new() -> ThumbnailCollectionBuilder {
        ThumbnailCollectionBuilder {
            collection: ThumbnailCollection {
                images: vec![],
                ops: vec![],
            },
        }
    }

    pub fn add_path(&mut self, path: &str) -> Result<&mut Self, CollectionError> {
        let t = ThumbnailData::load(Path::new(path).to_path_buf())?;
        self.collection.images.push(t);
        Ok(self)
    }

    pub fn add_glob(&mut self, glob: &str) -> Result<&mut Self, CollectionError> {
        let files = globwalk::glob(glob)?;
        let mut new_thumbs = vec![];
        for file in files {
            if let Ok(file) = file {
                new_thumbs.push(ThumbnailData::load(Path::new(file.path()).to_path_buf())?);
            }
        }
        self.collection.images.append(new_thumbs.as_mut());
        Ok(self)
    }

    pub fn add_thumb(&mut self, thumb: Thumbnail) -> Result<&mut Self, CollectionError> {
        self.collection.images.push(thumb.into_data());
        Ok(self)
    }

    pub fn finalize(self) -> ThumbnailCollection {
        self.collection
    }
}

pub struct ThumbnailCollection {
    images: Vec<ThumbnailData>,
    ops: Vec<Box<dyn Operation>>,
}

impl OperationContainer for ThumbnailCollection {
    fn add_op(&mut self, op: Box<dyn Operation>) {
        self.ops.push(op);
    }
}

impl GenericThumbnail for ThumbnailCollection {
    fn apply(&mut self) -> Result<&mut dyn GenericThumbnail, ApplyError> {
        for thumb in &mut self.images {
            thumb.apply_ops_list(&self.ops)?;
        }

        self.ops.clear();

        Ok(self)
    }

    fn apply_store(mut self, target: &Target) -> bool {
        for (n, mut thumb) in self.images.drain(0..).enumerate() {
            if thumb.apply_ops_list(&self.ops).is_ok() {
                if target.store(&mut thumb, Some(n as u32)).is_err() {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    fn apply_store_keep(
        &mut self,
        target: &Target,
    ) -> Result<&mut dyn GenericThumbnail, ApplyError> {
        for (n, thumb) in &mut self.images.iter_mut().enumerate() {
            thumb.apply_ops_list(&self.ops)?;
            target.store(thumb, Some(n as u32));
        }
        self.ops.clear();

        Ok(self)
    }

    fn store(self, target: &Target) -> bool {
        unimplemented!()
    }

    fn store_keep(&mut self, target: &Target) -> Result<&mut dyn GenericThumbnail, ApplyError> {
        unimplemented!()
    }
}
