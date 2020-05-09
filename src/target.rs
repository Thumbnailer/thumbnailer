use crate::errors::FileError;
use crate::thumbnail::data::ThumbnailData;
use image::{DynamicImage, ImageFormat};
use std::ffi::OsStr;
use std::path::PathBuf;

pub enum TargetMethod {
    Jpeg,
    Png,
    Tiff,
    Bmp,
    Gif,
}

pub struct TargetItem {
    path: PathBuf,
    flatten: bool,
    method: TargetMethod,
}

pub struct Target {
    items: Vec<TargetItem>,
}

pub struct TargetBuilder {
    target: Target,
}

impl TargetBuilder {
    pub fn new() -> Self {
        TargetBuilder {
            target: Target { items: vec![] },
        }
    }

    pub fn add_target(&mut self, method: TargetMethod, dst: PathBuf) -> &mut Self {
        self.target.items.push(TargetItem {
            path: dst,
            flatten: false,
            method,
        });

        self
    }

    pub fn add_target_flatten(&mut self, method: TargetMethod, dst: PathBuf) -> &mut Self {
        self.target.items.push(TargetItem {
            path: dst,
            flatten: true,
            method,
        });

        self
    }

    pub fn finalize(self) -> Target {
        self.target
    }
}

impl Target {
    pub fn store(
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
                TargetMethod::Jpeg => store_jpg(dyn_image, path),
                TargetMethod::Png => store_png(dyn_image, path),
                TargetMethod::Tiff => store_tiff(dyn_image, path),
                TargetMethod::Bmp => store_bmp(dyn_image, path),
                TargetMethod::Gif => store_gif(dyn_image, path),
            };
        }

        Ok(PathBuf::new())
    }
}

fn ensure_ext(ext: Option<&OsStr>, expected: &str) -> bool {
    match ext {
        None => false,
        Some(ext) => ext.to_string_lossy().to_lowercase().as_str() == expected,
    }
}

fn store_jpg(image: &DynamicImage, mut dst: PathBuf) -> PathBuf {
    if !ensure_ext(dst.extension(), "jpg") && !ensure_ext(dst.extension(), "jpeg") {
        dst.set_extension(OsStr::new("jpg"));
    }

    image.save_with_format(dst.clone(), ImageFormat::Jpeg);

    dst
}

fn store_png(image: &DynamicImage, mut dst: PathBuf) -> PathBuf {
    if !ensure_ext(dst.extension(), "png") {
        dst.set_extension(OsStr::new("png"));
    }

    image.save_with_format(dst.clone(), ImageFormat::Png);

    dst
}

fn store_tiff(image: &DynamicImage, mut dst: PathBuf) -> PathBuf {
    if !ensure_ext(dst.extension(), "tif") && !ensure_ext(dst.extension(), "tiff") {
        dst.set_extension(OsStr::new("tiff"));
    }

    image.save_with_format(dst.clone(), ImageFormat::Tiff);

    dst
}

fn store_bmp(image: &DynamicImage, mut dst: PathBuf) -> PathBuf {
    if !ensure_ext(dst.extension(), "bmp") {
        dst.set_extension(OsStr::new("bmp"));
    }

    image.save_with_format(dst.clone(), ImageFormat::Bmp);

    dst
}

fn store_gif(image: &DynamicImage, mut dst: PathBuf) -> PathBuf {
    if !ensure_ext(dst.extension(), "gif") {
        dst.set_extension(OsStr::new("gif"));
    }

    image.save_with_format(dst.clone(), ImageFormat::Gif);

    dst
}
