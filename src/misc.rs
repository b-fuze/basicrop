use std::sync::Arc;
use gpui::{Pixels, Point, RenderImage};

#[derive(Clone)]
pub enum LoadingImage {
    Image(Arc<RenderImage>),
    Failed,
    Loading,
}

impl LoadingImage {
    pub fn get_image(&self) -> Option<Arc<RenderImage>> {
        match self {
            LoadingImage::Image(image) => Some(Arc::clone(image)),
            _ => None,
        }
    }
}

pub enum CroppingMousePosition {
    Initial(Point<Pixels>),
    Moved(Point<Pixels>),
}

