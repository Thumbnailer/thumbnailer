use crate::errors::{ApplyError, CollectionError, FileError};
use crate::generic::OperationContainer;
use crate::thumbnail::data::ThumbnailData;
use crate::thumbnail::operations::Operation;
use crate::{GenericThumbnail, Target, Thumbnail};
use rayon::prelude::*;
use std::path::{Path, PathBuf};

/// The `ThumbnailCollectionBuilder` type. Allows to create a `ThumbnailCollection`
///
/// Provides method to construct a `ThumbnailCollection` from various image sources.
#[derive(Debug)]
pub struct ThumbnailCollectionBuilder {
    /// The collection being built
    collection: ThumbnailCollection,
}

impl ThumbnailCollectionBuilder {
    /// Creates a new instance of `ThumbnailCollectionBuilder`
    pub fn new() -> ThumbnailCollectionBuilder {
        ThumbnailCollectionBuilder {
            collection: ThumbnailCollection {
                images: vec![],
                ops: vec![],
            },
        }
    }
    /// Adds a single image by path to the collection.
    ///
    /// This internally calls the `ThumbnailData::load` method, and stores the result.
    ///
    /// # Errors
    /// Can return a `FileError::NotFound` if the file could not be found
    /// Can return a `FileError::NotSupported` if the file is of an unsupported type
    /// Can return a `FileError::IoError` if an error occurred while accessing the file
    ///
    /// # Examples
    /// ```
    /// use thumbnailer::thumbnail::ThumbnailCollectionBuilder;
    /// let mut builder = ThumbnailCollectionBuilder::new();
    /// builder.add_path("path/to/file.jpg").is_ok();
    /// ```
    pub fn add_path(&mut self, path: &str) -> Result<&mut Self, FileError> {
        let t = ThumbnailData::load(Path::new(path).to_path_buf())?;
        self.collection.images.push(t);
        Ok(self)
    }
    /// Adds a multiple images by (unix) glob to the collection.
    ///
    /// This uses the (globwalk)[https://docs.rs/globwalk/0.8.0/globwalk/] to parse the glob and find the files.
    /// See its documentation on how to construct globs.
    ///
    /// This internally calls the `ThumbnailData::load` method, and stores the result.
    ///
    /// * glob: &str - the glob to match files on the filesystem. See [glob (programming)](https://en.wikipedia.org/wiki/Glob_(programming))
    ///
    /// # Attention
    /// It stops parsing the found files on the first error loading a file
    ///
    /// # Errors
    /// Can return a `FileError::NotFound` if the file could not be found
    /// Can return a `FileError::NotSupported` if the file is of an unsupported type
    /// Can return a `FileError::IoError` if an error occurred while accessing the file
    /// Can return a `FileError::GlobError` if parsing the glob fails
    /// # Examples
    /// ```
    /// use thumbnailer::thumbnail::ThumbnailCollectionBuilder;
    /// let mut builder = ThumbnailCollectionBuilder::new();
    /// builder.add_path("resources/tests/*.{png,jpg}").is_ok();
    /// ```
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

    /// Adds a single, already existing `Thumbnail` to the collection
    ///
    /// * thumb: Thumbnail - The image to add.
    ///
    /// # Errors
    /// Cannot return a type. The Result return type is for consistency.
    ///
    /// # Examples
    /// ```
    /// use thumbnailer::thumbnail::ThumbnailCollectionBuilder;
    /// use thumbnailer::Thumbnail;
    /// use std::path::{PathBuf, Path};
    /// let mut builder = ThumbnailCollectionBuilder::new();
    /// let thumb = Thumbnail::load(Path::new("resources/tests/test.jpg").to_path_buf()).unwrap();
    /// builder.add_thumb(thumb).is_ok();
    /// ```
    pub fn add_thumb(&mut self, thumb: Thumbnail) -> Result<&mut Self, FileError> {
        self.collection.images.push(thumb.into_data());
        Ok(self)
    }

    /// Consumes the `ThumbnailCollectionBuilder` and returns the constructed `ThumbnailCollection`
    ///
    /// A collection can be used analogous to a single `Thumbnail`
    ///
    /// # Examples
    /// ```
    /// use thumbnailer::thumbnail::ThumbnailCollectionBuilder;
    /// let mut builder = ThumbnailCollectionBuilder::new();
    /// builder.add_path("resources/tests/*.{png,jpg}").is_ok();
    ///
    /// let mut collection = builder.finalize();
    /// ```
    pub fn finalize(self) -> ThumbnailCollection {
        self.collection
    }
}

impl Default for ThumbnailCollectionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// The `ThumbnailCollection` type.
///
/// This type represents a set of images.
#[derive(Debug)]
pub struct ThumbnailCollection {
    /// List of the actual image data
    images: Vec<ThumbnailData>,
    /// List of operations to apply to all images in the collection
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
