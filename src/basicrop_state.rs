use crate::counter_input::CounterView;
use crate::image_crop::ImageCrop;
use crate::misc::CroppingMousePosition;
use gpui::{Entity, Pixels, Point, Resource};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct BasicropState {
    pub crop_x: Entity<CounterView>,
    pub crop_y: Entity<CounterView>,
    pub width: Entity<CounterView>,
    pub height: Entity<CounterView>,
    pub mouse_initial_pos: Entity<Point<Pixels>>,
    pub mouse_pos: Entity<CroppingMousePosition>,
    pub is_selecting: Entity<bool>,
    pub image_crop: Entity<ImageCrop>,
    pub image_crop_initial: Entity<ImageCrop>,
    pub image_path: Resource,
    pub dest_image_path: Entity<PathBuf>,
    pub image_saved_notification: Entity<()>,
}
