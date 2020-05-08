use crate::errors::{CollectionError, FileError};
use crate::generic::OperationContainer;
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
        if let t = Thumbnail::load(Path::new(path).to_path_buf())? {
            self.collection.images.push(t);
        }
        Ok(self)
    }

    pub fn add_glob(&mut self, glob: &str) -> Result<&mut Self, CollectionError> {
        let files = globwalk::glob(glob)?;
        let mut new_thumbs = vec![];
        for file in files {
            if let Ok(file) = file {
                new_thumbs.push(Thumbnail::load(Path::new(file.path()).to_path_buf())?);
            }
        }
        self.collection.images.append(new_thumbs.as_mut());
        Ok(self)
    }

    pub fn add_thumb(&mut self, thumb: Thumbnail) -> Result<&mut Self, CollectionError> {
        self.collection.images.push(thumb);
        Ok(self)
    }

    pub fn finalize(self) -> ThumbnailCollection {
        self.collection
    }
}

pub struct ThumbnailCollection {
    images: Vec<Thumbnail>,
    ops: Vec<Box<dyn Operation>>,
}

impl OperationContainer for ThumbnailCollection {
    fn add_op(&mut self, op: Box<dyn Operation>) {
        self.ops.push(op);
    }
}

