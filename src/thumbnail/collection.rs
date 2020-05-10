use crate::errors::{ApplyError, CollectionError, FileError};
use crate::generic::OperationContainer;
use crate::thumbnail::data::ThumbnailData;
use crate::thumbnail::operations::Operation;
use crate::{GenericThumbnail, Target, Thumbnail};
use rayon::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Debug)]
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

    pub fn add_path(&mut self, path: &str) -> Result<&mut Self, FileError> {
        let t = ThumbnailData::load(Path::new(path).to_path_buf())?;
        self.collection.images.push(t);
        Ok(self)
    }

    pub fn add_glob(&mut self, glob: &str) -> Result<&mut Self, FileError> {
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

    pub fn add_thumb(&mut self, thumb: Thumbnail) -> Result<&mut Self, FileError> {
        self.collection.images.push(thumb.into_data());
        Ok(self)
    }

    pub fn finalize(self) -> ThumbnailCollection {
        self.collection
    }
}

#[derive(Debug)]
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

        let results: Vec<Option<ApplyError>> = self
            .images
            .par_iter_mut()
            .map(|data| -> Option<ApplyError> {
                match data.apply_ops_list(&ops) {
                    Ok(_) => None,
                    Err(err) => Some(err),
                }
            })
            .collect();

        let errors = results
            .iter()
            .filter_map(|r| match r {
                None => None,
                Some(apply_error) => match apply_error {
                    ApplyError::OperationError(err) => Some(err.clone()),
                    _ => None,
                },
            })
            .collect();

        if results.is_empty() {
            Ok(self)
        } else {
            Err(ApplyError::CollectionError(CollectionError::new(
                vec![],
                vec![],
                errors,
            )))
        }
    }

    fn apply_store(mut self, target: &Target) -> Result<Vec<PathBuf>, ApplyError> {
        self.apply_store_keep(target)
    }

    fn apply_store_keep(&mut self, target: &Target) -> Result<Vec<PathBuf>, ApplyError> {
        let ops = self.ops.clone();
        self.ops.clear();

        let results: Vec<Result<Vec<PathBuf>, ApplyError>> = self
            .images
            .par_iter_mut()
            .enumerate()
            .map(|(n, data)| -> Result<Vec<PathBuf>, ApplyError> {
                if let Err(err) = data.apply_ops_list(&ops) {
                    return Err(err);
                }
                match target.store(data, Some(n as u32)) {
                    Ok(paths) => Ok(paths),
                    Err(err) => Err(ApplyError::StoreError(err)),
                }
            })
            .collect();

        let mut paths = vec![];
        let mut store_errors = vec![];
        let mut operation_errors = vec![];

        for result in results {
            match result {
                Ok(mut p) => paths.append(&mut p),
                Err(err) => match err {
                    ApplyError::OperationError(op_err) => operation_errors.push(op_err),
                    ApplyError::StoreError(store_err) => store_errors.push(store_err),
                    _ => {}
                },
            }
        }

        if store_errors.is_empty() && operation_errors.is_empty() {
            Ok(paths)
        } else {
            Err(ApplyError::CollectionError(CollectionError::new(
                paths,
                store_errors,
                operation_errors,
            )))
        }
    }

    fn store(mut self, target: &Target) -> Result<Vec<PathBuf>, ApplyError> {
        self.store_keep(target)
    }

    fn store_keep(&mut self, target: &Target) -> Result<Vec<PathBuf>, ApplyError> {
        let results: Vec<Result<Vec<PathBuf>, FileError>> = self
            .images
            .par_iter_mut()
            .enumerate()
            .map(|(n, data)| target.store(data, Some(n as u32)))
            .collect();

        let mut paths = vec![];
        let mut store_errors = vec![];

        for result in results {
            match result {
                Ok(mut p) => paths.append(&mut p),
                Err(err) => store_errors.push(err),
            }
        }

        if store_errors.is_empty() {
            Ok(paths)
        } else {
            Err(ApplyError::CollectionError(CollectionError::new(
                paths,
                store_errors,
                vec![],
            )))
        }
    }
}
