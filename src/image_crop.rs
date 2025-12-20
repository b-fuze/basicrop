use std::path::PathBuf;
use std::sync::Arc;
use gpui::{Entity, Global, Pixels, RenderImage};

#[derive(Clone, Debug)]
pub enum ImageCrop {
    Cropped {
        crop_x: Pixels,
        crop_y: Pixels,
        width: Pixels,
        height: Pixels,
    },
    Uninitialized,
}

#[derive(Clone, Debug)]
pub struct InitializedImageCrop {
    pub crop_x: Pixels,
    pub crop_y: Pixels,
    pub width: Pixels,
    pub height: Pixels,
}

#[derive(Clone, Debug)]
pub struct FinalizedImageCrop {
    pub crop_x: u32,
    pub crop_y: u32,
    pub width: u32,
    pub height: u32,
}

impl ImageCrop {
    pub fn to_final(&self) -> Option<FinalizedImageCrop> {
        match self {
            ImageCrop::Cropped {
                crop_x,
                crop_y,
                width,
                height,
            } => Some(FinalizedImageCrop {
                crop_x: crop_x.into(),
                crop_y: crop_y.into(),
                width: width.into(),
                height: height.into(),
            }),
            ImageCrop::Uninitialized => None,
        }
    }

    pub fn to_initialized(&self) -> Option<InitializedImageCrop> {
        match self {
            ImageCrop::Cropped {
                crop_x,
                crop_y,
                width,
                height,
            } => Some(InitializedImageCrop {
                crop_x: crop_x.clone(),
                crop_y: crop_y.clone(),
                width: width.clone(),
                height: height.clone(),
            }),
            ImageCrop::Uninitialized => None,
        }
    }
}

impl PartialEq for ImageCrop {
    fn eq(&self, rhs: &ImageCrop) -> bool {
        match (&self, rhs) {
            (ImageCrop::Uninitialized, ImageCrop::Uninitialized) => true,
            (ImageCrop::Uninitialized, _) => false,
            (_, ImageCrop::Uninitialized) => false,
            (
                ImageCrop::Cropped {
                    crop_x,
                    crop_y,
                    width,
                    height,
                },
                ImageCrop::Cropped {
                    crop_x: rhs_crop_x,
                    crop_y: rhs_crop_y,
                    width: rhs_width,
                    height: rhs_height,
                },
            ) => {
                crop_x == rhs_crop_x
                    && crop_y == rhs_crop_y
                    && width == rhs_width
                    && height == rhs_height
            }
        }
    }
}

pub struct CroppingState {
    pub image_crop: Option<ImageCrop>,
    pub image_initial: Option<ImageCrop>,
    pub image: Option<Arc<RenderImage>>,
    pub dest_path: PathBuf,
}

impl Global for CroppingState {}
