use crate::thumbnail::operations::{
    BlurOp, BrightenOp, CombineOp, ContrastOp, CropOp, ExifOp, FlipOp, HuerotateOp, InvertOp,
    ResizeOp, TextOp, UnsharpenOp,
};
use crate::thumbnail::ImageData::Image;
use crate::{
    errors,
    errors::{FileError, FileNotFoundError, FileNotSupportedError, InternalError},
    generic::{BoxPosition, Crop, Exif, GenericThumbnail, Orientation, ResampleFilter, Resize},
    thumbnail::operations::Operation,
};
use image::{io::Reader, DynamicImage, ImageFormat};
use std::path::PathBuf;
use std::{fs::File, io::BufReader, path::Path};

pub mod operations;

#[derive(Clone)]
pub struct StaticThumbnail {
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
}

pub trait SingleThumbnail: GenericThumbnail {
    fn to_static_copy(&mut self) -> Option<StaticThumbnail>;
}
//TODO: #[derive(Clone)]
enum ImageData {
    File(File, ImageFormat),
    Image(DynamicImage),
}

//TODO: #[derive(Clone)]
pub struct Thumbnail {
    path: PathBuf,
    height: u32,
    width: u32,
    image: ImageData,
    ops: Vec<Box<dyn Operation>>,
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
            height: 0,
            width: 0,
            ops: vec![],
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
        if let (ImageData::File(file, _)) = &self.image {
            let reader = Reader::new(BufReader::new(file));
            self.image = ImageData::Image(reader.decode()?);
        }

        return match &mut self.image {
            ImageData::Image(image) => Ok(image),
            ImageData::File(file, _) => Err(InternalError::UnknownError(errors::UnknownError)),
        };
    }

    fn assert_dynamic_image_loaded(&mut self) -> bool {
        match self.get_dyn_image() {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

impl SingleThumbnail for Thumbnail {
    fn to_static_copy(&mut self) -> Option<StaticThumbnail> {
        match self.get_dyn_image() {
            Ok(i) => Some(StaticThumbnail { image: i.clone() }),
            Err(_) => None,
        }
    }
}

impl GenericThumbnail for Thumbnail<'_> {
    fn resize(&mut self, size: Resize) -> &mut dyn GenericThumbnail {
        self.ops.push(Box::new(ResizeOp::new(size, None)));
        self
    }

    fn resize_filter(&mut self, size: Resize, filter: ResampleFilter) -> &mut dyn GenericThumbnail {
        self.ops
            .push(Box::new(ResizeOp::new(size, Option::from(filter))));
        self
    }

    fn blur(&mut self, sigma: f32) -> &mut dyn GenericThumbnail {
        self.ops.push(Box::new(BlurOp::new(sigma)));
        self
    }

    fn brighten(&mut self, value: i32) -> &mut dyn GenericThumbnail {
        self.ops.push(Box::new(BrightenOp::new(value)));
        self
    }

    fn huerotate(&mut self, degree: i32) -> &mut dyn GenericThumbnail {
        self.ops.push(Box::new(HuerotateOp::new(degree)));
        self
    }

    fn contrast(&mut self, value: f32) -> &mut dyn GenericThumbnail {
        self.ops.push(Box::new(ContrastOp::new(value)));
        self
    }

    fn unsharpen(&mut self, sigma: f32, threshold: i32) -> &mut dyn GenericThumbnail {
        self.ops.push(Box::new(UnsharpenOp::new(sigma, threshold)));
        self
    }

    fn crop(&mut self, c: Crop) -> &mut dyn GenericThumbnail {
        self.ops.push(Box::new(CropOp::new(c)));
        self
    }

    fn flip(&mut self, orientation: Orientation) -> &mut dyn GenericThumbnail {
        self.ops.push(Box::new(FlipOp::new(orientation)));
        self
    }

    fn invert(&mut self) -> &mut dyn GenericThumbnail {
        self.ops.push(Box::new(InvertOp::new()));
        self
    }

    fn exif(&mut self, metadata: Exif) -> &mut dyn GenericThumbnail {
        self.ops.push(Box::new(ExifOp::new(metadata)));
        self
    }

    fn text(&mut self, text: String, pos: BoxPosition) -> &mut dyn GenericThumbnail {
        self.ops.push(Box::new(TextOp::new(text, pos)));
        self
    }

    fn combine(&mut self, image: StaticThumbnail, pos: BoxPosition) -> &mut dyn GenericThumbnail {
        self.ops.push(Box::new(CombineOp::new(image, pos)));
        self
    }

    fn apply(&mut self) -> &mut dyn GenericThumbnail {
        self.assert_dynamic_image_loaded();

        if let ImageData::Image(image) = &mut self.image {
            for operation in &self.ops {
                operation.apply(image);
            }
        }

        self.ops.clear();

        self
    }
}
