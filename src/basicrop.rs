use crate::image_crop::{ImageCrop, FinalizedImageCrop};
use crate::misc::{LoadingImage, CroppingMousePosition};
use crate::basicrop_state::BasicropState;
use crate::main_view::render_main_view;
use crate::counter_input;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use gpui::{
    bounds, canvas, div, hsla, img, percentage, point, prelude::*, px, quad, rgb, rgba, size, App, Application, BorderStyle, Bounds, Context, DragMoveEvent, Edges, Entity, ImageAssetLoader, ImageSource, ObjectFit, PathBuilder, Pixels, Point, Position, RenderImage, Resource, SharedString, Size, Subscription, TitlebarOptions, Window, WindowBounds, WindowDecorations, WindowOptions
};
use gpui_component::*;

pub struct Basicrop {
    state: BasicropState,
}

impl Basicrop {
   pub  fn new(window: &mut Window, cx: &mut Context<Self>, image_path: PathBuf, dest_image_path: PathBuf) -> Self {
        let crop_x = cx.new(|cx|
            counter_input::CounterView::new(window, cx, 0));
        let crop_y = cx.new(|cx|
            counter_input::CounterView::new(window, cx, 0));
        let width = cx.new(|cx|
            counter_input::CounterView::new(window, cx, 0));
        let height = cx.new(|cx|
            counter_input::CounterView::new(window, cx, 0));
        let is_selecting = cx.new(|_| false);
        let mouse_initial_pos = cx.new(|_| Point { x: px(0.0), y: px(0.0) });
        let mouse_pos = cx.new(|_| CroppingMousePosition::Moved(Point { x: px(0.0), y: px(0.0) }));
        let image_path: Resource = image_path.into();
        let image_crop = cx.new(|_| ImageCrop::Uninitialized);
        let dest_image_path = cx.new(|_| dest_image_path);
        let image_saved_notification = cx.new(|cx| {
            cx.observe_self(|_, cx| {
                println!("info: exiting");
                cx.shutdown();
            }).detach();
        });

        // Handlers for text input updates
        crop_x.update(cx, {
            let image_crop_clone = image_crop.clone();
            |view, cx| view.subscribe(window, cx, image_crop.clone(), move |new_value, image_crop, cx| {
                if new_value != f32::from(image_crop.crop_x) as u32 {
                    image_crop_clone.write(cx, ImageCrop::Cropped {
                        crop_x: (new_value as f32).into(),
                        crop_y: image_crop.crop_y,
                        width: image_crop.width,
                        height: image_crop.height,
                    });
                }
            })
        });
        crop_y.update(cx, {
            let image_crop_clone = image_crop.clone();
            |view, cx| view.subscribe(window, cx, image_crop.clone(), move |new_value, image_crop, cx| {
                if new_value != f32::from(image_crop.crop_y) as u32 {
                    image_crop_clone.write(cx, ImageCrop::Cropped {
                        crop_x: image_crop.crop_x,
                        crop_y: (new_value as f32).into(),
                        width: image_crop.width,
                        height: image_crop.height,
                    });
                }
            })
        });
        width.update(cx, {
            let image_crop_clone = image_crop.clone();
            |view, cx| view.subscribe(window, cx, image_crop.clone(), move |new_value, image_crop, cx| {
                if new_value != f32::from(image_crop.width) as u32 {
                    image_crop_clone.write(cx, ImageCrop::Cropped {
                        crop_x: image_crop.crop_x,
                        crop_y: image_crop.crop_y,
                        width: (new_value as f32).into(),
                        height: image_crop.height,
                    });
                }
            })
        });
        height.update(cx, {
            let image_crop_clone = image_crop.clone();
            |view, cx| view.subscribe(window, cx, image_crop.clone(), move |new_value, image_crop, cx| {
                if new_value != f32::from(image_crop.height) as u32 {
                    image_crop_clone.write(cx, ImageCrop::Cropped {
                        crop_x: image_crop.crop_x,
                        crop_y: image_crop.crop_y,
                        width: image_crop.width,
                        height: (new_value as f32).into(),
                    });
                }
            })
        });

        Basicrop {
            state: BasicropState {
                crop_x,
                crop_y,
                width,
                height,
                is_selecting,
                mouse_initial_pos,
                mouse_pos,
                image_path,
                image_crop,
                dest_image_path,
                image_saved_notification,
            }
        }
    }
}

impl Render for Basicrop {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = &mut self.state;

        let image_asset = match window.use_asset::<ImageAssetLoader>(&state.image_path, cx) {
            Some(Ok(asset)) => LoadingImage::Image(asset),
            Some(Err(_)) => LoadingImage::Failed,
            _ => LoadingImage::Loading,
        };

        // let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        // println!("[{}] in render", time.as_millis());

        // Update the imagecrop with the initial image dimensions
        if let (
            ImageCrop::Uninitialized,
            LoadingImage::Image(image),
        ) = (state.image_crop.read(cx), &image_asset) {
            let size = image.size(0);
            state.image_crop.write(cx, ImageCrop::Cropped {
                crop_x: px(0.),
                crop_y: px(0.),
                width: (u32::from(size.width) as f32).into(),
                height: (u32::from(size.height) as f32).into(),
            });
            state.height.update(cx, |input, cx| {
                input.get_state().update(cx, |input, cx| {
                    let height = u32::from(size.height).to_string();
                    input.set_value(height, window, cx);
                });
            });
            state.width.update(cx, |input, cx| {
                input.get_state().update(cx, |input, cx| {
                    let width = u32::from(size.width).to_string();
                    input.set_value(width, window, cx);
                });
            });
        }

        render_main_view(state, image_asset, cx)
    }
}
