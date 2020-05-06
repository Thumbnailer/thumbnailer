use crate::StaticThumbnail;

#[derive(Debug, Copy, Clone)]
pub enum Resize {
    Height(u32),
    Width(u32),
    BoundingBox(u32, u32),
    ExactBox(u32, u32)
}

#[derive(Debug, Copy, Clone)]
pub enum BoxPosition {
     TopLeft(u32, u32),
     TopRight(u32, u32),
     BottomLeft(u32, u32),
     BottomRight(u32, u32)
}

#[derive(Debug, Copy, Clone)]
pub enum Crop {
    Box(u32, u32, u32, u32),
    Ratio(f32, f32)
}

#[derive(Debug, Copy, Clone)]
pub enum Orientation {
    Vertical,
    Horizontal
}

#[derive(Debug, Clone)]
pub enum Exif {
     Keep,
     Clear,
     Whitelist(Vec<u16>),
     Blacklist(Vec<u16>)
}

#[derive(Debug, Copy, Clone)]
pub enum ResampleFilter {
    Nearest,
    Triangle,
    CatmullRom,
    Gaussian,
    Lanczos3,
}


pub trait GenericThumbnail{
    fn resize(&mut self, size: Resize) -> &mut dyn GenericThumbnail;
    fn resize_filter(&mut self, size: Resize, filter: ResampleFilter) -> &mut dyn GenericThumbnail;


    fn blur(&mut self, sigma: f32) -> &mut dyn GenericThumbnail;
    fn brighten(&mut self, value: i32) -> &mut dyn GenericThumbnail;
    fn huerotate(&mut self, degree: i32) -> &mut dyn GenericThumbnail;
    fn contrast(&mut self, value: f32) -> &mut dyn GenericThumbnail;
    fn unsharpen(&mut self, sigma: f32, threshold: u32) -> &mut dyn GenericThumbnail;

    fn crop(&mut self, c: Crop) -> &mut dyn GenericThumbnail;
    fn flip(&mut self, orientation: Orientation) -> &mut dyn GenericThumbnail;

    fn invert(&mut self) -> &mut dyn GenericThumbnail;

    fn exif(&mut self, metadata: Exif) -> &mut dyn GenericThumbnail;
    fn text(&mut self, text: String, pos: BoxPosition) -> &mut dyn GenericThumbnail;

    fn combine(&mut self, image: StaticThumbnail, pos: BoxPosition) -> &mut dyn GenericThumbnail;

    fn apply(&mut self) -> &mut dyn GenericThumbnail;
}