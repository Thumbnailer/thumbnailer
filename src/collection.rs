use crate::errors::{ApplyError, CollectionError};
use crate::generic::OperationContainer;
use crate::thumbnail::data::ThumbnailData;
use crate::thumbnail::operations::Operation;
use crate::{GenericThumbnail, Target, Thumbnail};
use rayon::prelude::*;
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
        let ops = self.ops.clone();
        self.ops.clear();

        self.images.par_iter_mut().for_each(|data| {
            data.apply_ops_list(&ops);
        });

        Ok(self)
    }

    fn apply_store(mut self, target: &Target) -> bool {
        self.apply_store_keep(target).is_ok()
    }

    fn apply_store_keep(
        &mut self,
        target: &Target,
    ) -> Result<&mut dyn GenericThumbnail, ApplyError> {
        let ops = self.ops.clone();
        self.ops.clear();

        self.images
            .par_iter_mut()
            .enumerate()
            .for_each(|(n, data)| {
                data.apply_ops_list(&ops);
                target.store(data, Some(n as u32));
            });

        Ok(self)
    }

    fn store(mut self, target: &Target) -> bool {
        self.store_keep(target).is_ok()
    }

    fn store_keep(&mut self, target: &Target) -> Result<&mut dyn GenericThumbnail, ApplyError> {
        self.images
            .par_iter_mut()
            .enumerate()
            .for_each(|(n, data)| {
                target.store(data, Some(n as u32));
            });

        Ok(self)
    }
}
