use crate::errors::FileError;
use crate::thumbnail::data::ThumbnailData;
use image::{DynamicImage, ImageFormat};
use std::ffi::OsStr;
use std::path::PathBuf;

/// The `TargetMethod` type. This sets the file type of the output file.
#[derive(Debug)]
pub enum TargetFormat {
    /// Jpeg file
    Jpeg,
    /// PNG file
    Png,
    /// Tiff file
    Tiff,
    /// BMP file
    Bmp,
    /// GIF file
    Gif,
}
/// The `TargetItem` type. This basically defines one single actual target.
#[derive(Debug)]
pub struct TargetItem {
    /// The file destination path
    path: PathBuf,
    // flatten: bool,
    /// The file type of the target file
    method: TargetFormat,
}
/// The `Target` type. This defines a list of path and file type combinations, the given image will be stored to.
#[derive(Debug)]
pub struct Target {
    items: Vec<TargetItem>,
}

impl Target {
    /// Constructs a new `Target with a first single entry.
    ///
    /// A single target or `TargetItem` is a tuple consisting of a file type/format and
    /// a path. When the target is used to store the resulting image.
    /// For every item in this set a file with the corresponding file type will be created.
    ///
    /// The path (`dst`) can be either a directory, in which case the old file name will be kept.
    /// Or a file path, in which case the file will be saved under that path.
    /// If the file extensions does not match the type, a matching one will be added
    ///
    ///
    /// * `method: TargetMethod` - The target file type
    /// *  `dst: PathBuf` - The path to save the file to.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use thumbnailer::target::TargetFormat;
    /// use thumbnailer::Target;
    /// Target::new(TargetFormat::Jpeg, Path::new("image.jpg").to_path_buf());
    /// ```
    pub fn new(method: TargetFormat, dst: PathBuf) -> Self {
        Target { items: vec![] }.add_target(method, dst)
    }

    /// Adds another actual target to the target set.
    ///
    /// Returns Self to allow method chaining.
    ///
    /// * `method: TargetMethod` - The target file type
    /// *  `dst: PathBuf` - The path to save the file to.  Can be either a directory, in which case the old file name will be kept. \
    ///                     Or a file path, in which case the file will be saved under that path. \
    ///                     If the file extensions does not match the type, a matching one will be added
    ///
    /// # Attention
    /// This method takes self as a move and then returns self again.
    /// Therefore to continue using the `Target` instance, the return value of this method has to be reassigned.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use thumbnailer::target::TargetFormat;
    /// use thumbnailer::Target;
    /// Target::new(TargetFormat::Jpeg, Path::new("image.jpg").to_path_buf());
    /// ```
    pub fn add_target(mut self, method: TargetFormat, dst: PathBuf) -> Self {
        self.items.push(TargetItem {
            path: dst,
            // flatten: false,
            method,
        });

        self
    }

    // pub fn add_target_flatten(&mut self, method: TargetMethod, dst: PathBuf) -> &mut Self {
    //     self.target.items.push(TargetItem {
    //         path: dst,
    //         flatten: true,
    //         method,
    //     });
    //
    //     self
    // }

    /// Stores the given image to the configured targets
    ///
    /// This takes the image data and saves it to the given path
    /// and type for all configures targets in this `Target` instance.
    ///
    /// This can be based a `u32` number, which will be added to the end of the file name, before the extension.
    ///
    /// * thumb: &mut ThumbnailData - The image data
    /// * count: Option<u32> - If not None, the given number will be added to the end of the file name, before the extension.
    ///
    pub(crate) fn store(
        &self,
        thumb: &mut ThumbnailData,
        count: Option<u32>,
    ) -> Result<PathBuf, FileError> {
        let orig_path = thumb.get_path();
        let filename = match orig_path.file_stem() {
            None => OsStr::new("NAME_MISSING"),
            Some(name) => name.clone(),
        };

        for item in &self.items {
            let mut path = if !item.path.is_file() {
                let mut new_path = item.path.clone();
                new_path.set_file_name(filename);
                new_path
            } else {
                item.path.clone()
            };

            if let Some(count) = count {
                let filename = format!(
                    "{}-{}.{}",
                    path.file_stem()
                        .unwrap_or(OsStr::new("NAME_MISSING"))
                        .to_string_lossy(),
                    count,
                    path.extension().unwrap_or(OsStr::new("")).to_string_lossy()
                );
                path.set_file_name(filename);
            }

            let dyn_image = thumb.get_dyn_image()?;

            let path = match item.method {
                TargetFormat::Jpeg => store_jpg(dyn_image, path),
                TargetFormat::Png => store_png(dyn_image, path),
                TargetFormat::Tiff => store_tiff(dyn_image, path),
                TargetFormat::Bmp => store_bmp(dyn_image, path),
                TargetFormat::Gif => store_gif(dyn_image, path),
            };
        }

        Ok(PathBuf::new())
    }
}
/// Check if ext matches the expected extension
///
/// * ext: Option<&OsStr> - The actual extension as returned by Path::extension()
/// * expected: &str - The desired file extension
fn ensure_ext(ext: Option<&OsStr>, expected: &str) -> bool {
    match ext {
        None => false,
        Some(ext) => ext.to_string_lossy().to_lowercase().as_str() == expected,
    }
}

/// Stores `DynamicImage` as JPEG to the given path.
///
/// Returns the actual path the file has been saved to. (Path might be extended by the correct file extension.
///
/// * image: &DynamicImage - The image data
/// * dst: PathBuf - The destination path
fn store_jpg(image: &DynamicImage, mut dst: PathBuf) -> PathBuf {
    if !ensure_ext(dst.extension(), "jpg") && !ensure_ext(dst.extension(), "jpeg") {
        dst.set_extension(OsStr::new("jpg"));
    }

    image.save_with_format(dst.clone(), ImageFormat::Jpeg);

    dst
}
/// Stores `DynamicImage` as PNG to the given path.
///
/// Returns the actual path the file has been saved to. (Path might be extended by the correct file extension.
///
/// * image: &DynamicImage - The image data
/// * dst: PathBuf - The destination path
fn store_png(image: &DynamicImage, mut dst: PathBuf) -> PathBuf {
    if !ensure_ext(dst.extension(), "png") {
        dst.set_extension(OsStr::new("png"));
    }

    image.save_with_format(dst.clone(), ImageFormat::Png);

    dst
}

/// Stores `DynamicImage` as TIFF to the given path.
///
/// Returns the actual path the file has been saved to. (Path might be extended by the correct file extension.
///
/// * image: &DynamicImage - The image data
/// * dst: PathBuf - The destination path
fn store_tiff(image: &DynamicImage, mut dst: PathBuf) -> PathBuf {
    if !ensure_ext(dst.extension(), "tif") && !ensure_ext(dst.extension(), "tiff") {
        dst.set_extension(OsStr::new("tiff"));
    }

    image.save_with_format(dst.clone(), ImageFormat::Tiff);

    dst
}

/// Stores `DynamicImage` as BMP to the given path.
///
/// Returns the actual path the file has been saved to. (Path might be extended by the correct file extension.
///
/// * image: &DynamicImage - The image data
/// * dst: PathBuf - The destination path
fn store_bmp(image: &DynamicImage, mut dst: PathBuf) -> PathBuf {
    if !ensure_ext(dst.extension(), "bmp") {
        dst.set_extension(OsStr::new("bmp"));
    }

    image.save_with_format(dst.clone(), ImageFormat::Bmp);

    dst
}
/// Stores `DynamicImage` as GIF to the given path.
///
/// Returns the actual path the file has been saved to. (Path might be extended by the correct file extension.
///
/// * image: &DynamicImage - The image data
/// * dst: PathBuf - The destination path
fn store_gif(image: &DynamicImage, mut dst: PathBuf) -> PathBuf {
    if !ensure_ext(dst.extension(), "gif") {
        dst.set_extension(OsStr::new("gif"));
    }

    image.save_with_format(dst.clone(), ImageFormat::Gif);

    dst
}
