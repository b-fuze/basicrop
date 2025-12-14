use crate::counter_input::CounterView;
use crate::image_crop::ImageCrop;
use crate::misc::{CroppingMousePosition, LoadingImage};
use gpui::{
    BorderStyle, Bounds, Entity, IntoElement, PathBuilder, Pixels, Point, Size, Styled, canvas,
    point, px, quad, rgba, size,
};

pub fn selection_canvas(
    crop_x: Entity<CounterView>,
    crop_y: Entity<CounterView>,
    width: Entity<CounterView>,
    height: Entity<CounterView>,
    image_asset: LoadingImage,
    image_crop: Entity<ImageCrop>,
    is_selecting: Entity<bool>,
    mouse_pos: Entity<CroppingMousePosition>,
    mouse_initial_pos: Entity<Point<Pixels>>,
) -> impl IntoElement + Styled {
    canvas(
        |_, _, _| {},
        move |bounds, _, window, cx| {
            // println!("Origin {{ x: {}, y: {} }}", bounds.origin.x, bounds.origin.y);
            // println!("Bounds {{ width: {}, height: {} }}", bounds.size.width, bounds.size.height);
            let image = match image_asset.get_image() {
                Some(image) => image,
                None => {
                    return;
                }
            };

            let image_size = image.size(0);
            let is_selecting_value = *is_selecting.read(cx);

            // Calculate current location of image in viewport relative to
            // the canvas bounds
            let image_width = u64::from(image_size.width);
            let image_height = u64::from(image_size.height);
            let viewport_aspect_ratio = bounds.size.width.to_f64() / bounds.size.height.to_f64();
            let image_aspect_ratio = (image_width as f64) / (image_height as f64);
            let image_visible_scale = if viewport_aspect_ratio > image_aspect_ratio {
                bounds.size.height.to_f64() / u64::from(image_size.height) as f64
            } else {
                bounds.size.width.to_f64() / u64::from(image_size.width) as f64
            };
            let image_visible_scale_inverse = 1.0 / image_visible_scale;
            let image_visible_width = (image_width as f64) * image_visible_scale;
            let image_visible_height = (image_height as f64) * image_visible_scale;

            // We need this because only the canvas' height might not match the
            // image's height, but the width will always match
            let bounds_padding_x = (image_visible_width.max(bounds.size.width.to_f64())
                - image_visible_width.min(bounds.size.width.to_f64()))
                / 2.0;
            let bounds_padding_y = (image_visible_height.max(bounds.size.height.to_f64())
                - image_visible_height.min(bounds.size.height.to_f64()))
                / 2.0;

            // If we're selecting then we want to base coordinates off of
            // the mouse, otherwise we want to use image_crop
            let (mouse_initial, mouse_cur) = if is_selecting_value {
                let mouse_initial_pos = mouse_initial_pos.read(cx).clone();
                let mouse_pos = match mouse_pos.read(cx) {
                    CroppingMousePosition::Initial(pos) => *pos + bounds.origin,
                    CroppingMousePosition::Moved(pos) => *pos,
                };

                (
                    point(
                        (mouse_initial_pos.x + bounds.origin.x)
                            .max(bounds.origin.x + px(bounds_padding_x as f32))
                            .min(
                                bounds.origin.x
                                    + px((bounds_padding_x + image_visible_width) as f32),
                            ),
                        (mouse_initial_pos.y + bounds.origin.y)
                            .max(bounds.origin.y + px(bounds_padding_y as f32))
                            .min(
                                bounds.origin.y
                                    + px((bounds_padding_y + image_visible_height) as f32),
                            ),
                    ),
                    point(
                        mouse_pos
                            .x
                            .max(bounds.origin.x + px(bounds_padding_x as f32))
                            .min(
                                bounds.origin.x
                                    + px((bounds_padding_x + image_visible_width) as f32),
                            ),
                        mouse_pos
                            .y
                            .max(bounds.origin.y + px(bounds_padding_y as f32))
                            .min(
                                bounds.origin.y
                                    + px((bounds_padding_y + image_visible_height) as f32),
                            ),
                    ),
                )
            } else {
                let image_visible_scale = image_visible_scale as f32;
                let bounds_padding_x = px(bounds_padding_x as f32);
                let bounds_padding_y = px(bounds_padding_y as f32);
                match *image_crop.read(cx) {
                    ImageCrop::Cropped {
                        crop_x,
                        crop_y,
                        width,
                        height,
                    } => (
                        point(
                            bounds.origin.x + crop_x * image_visible_scale + bounds_padding_x,
                            bounds.origin.y + crop_y * image_visible_scale + bounds_padding_y,
                        ),
                        point(
                            bounds.origin.x
                                + (crop_x + width) * image_visible_scale
                                + bounds_padding_x,
                            bounds.origin.y
                                + (crop_y + height) * image_visible_scale
                                + bounds_padding_y,
                        ),
                    ),
                    _ => (point(px(0.), px(0.)), point(px(0.), px(0.))),
                }
            };

            let origin = point(
                mouse_initial.x.min(mouse_cur.x),
                mouse_initial.y.min(mouse_cur.y),
            );

            let se_corner = point(
                mouse_initial.x.max(mouse_cur.x),
                mouse_initial.y.max(mouse_cur.y),
            );

            let image_crop_x_value =
                (origin.x.to_f64() - bounds.origin.x.to_f64() - bounds_padding_x)
                    * image_visible_scale_inverse;
            let image_crop_y_value =
                (origin.y.to_f64() - bounds.origin.y.to_f64() - bounds_padding_y)
                    * image_visible_scale_inverse;
            let image_width_value =
                (se_corner.x.to_f64() - origin.x.to_f64()) * image_visible_scale_inverse;
            let image_height_value =
                (se_corner.y.to_f64() - origin.y.to_f64()) * image_visible_scale_inverse;

            // let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            // println!("[{}] in canvas", time.as_millis());

            if *is_selecting.read(cx) {
                let size = Size {
                    width: se_corner.x - origin.x,
                    height: se_corner.y - origin.y,
                };
                let quad_bounds = Bounds::new(origin, size);
                window.paint_quad(quad(
                    quad_bounds,
                    px(0.0),
                    rgba(0x709ebe7f),
                    px(1.0),
                    rgba(0x709ebeaf),
                    BorderStyle::default(),
                ));

                let new_image_crop = ImageCrop::Cropped {
                    crop_x: (image_crop_x_value as f32).into(),
                    crop_y: (image_crop_y_value as f32).into(),
                    width: (image_width_value as f32).into(),
                    height: (image_height_value as f32).into(),
                };
                if &new_image_crop != image_crop.read(cx) {
                    // println!("Updated crop: {:?}", new_image_crop);
                    image_crop.write(cx, new_image_crop);
                }
            } else {
                let occlusion_bounds = gpui::bounds(
                    point(
                        px((bounds.origin.x.to_f64() + bounds_padding_x) as f32),
                        px((bounds.origin.y.to_f64() + bounds_padding_y) as f32),
                    ),
                    size(
                        px((bounds.size.width.to_f64() - bounds_padding_x * 2.0) as f32),
                        px((bounds.size.height.to_f64() - bounds_padding_y * 2.0) as f32),
                    ),
                );
                let mut builder = PathBuilder::fill();
                builder.move_to(occlusion_bounds.origin);
                builder.line_to(occlusion_bounds.top_right());
                builder.line_to(occlusion_bounds.bottom_right());
                builder.line_to(occlusion_bounds.bottom_left());
                builder.close();
                builder.move_to(origin);
                builder.line_to(point(se_corner.x, origin.y));
                builder.line_to(point(se_corner.x, se_corner.y));
                builder.line_to(point(origin.x, se_corner.y));
                builder.close();
                let path = builder.build().unwrap();
                window.paint_path(path, rgba(0x000000c8));
            }

            if *is_selecting.read(cx) {
                crop_x.update(cx, |input, cx| {
                    input.get_state().update(cx, |input, cx| {
                        let value = (image_crop_x_value as u64).to_string();
                        input.set_value(value, window, cx);
                    });
                });
                crop_y.update(cx, |input, cx| {
                    input.get_state().update(cx, |input, cx| {
                        let value = (image_crop_y_value as u64).to_string();
                        input.set_value(value, window, cx);
                    });
                });
                width.update(cx, |input, cx| {
                    input.get_state().update(cx, |input, cx| {
                        let value = (image_width_value as u64).to_string();
                        input.set_value(value, window, cx);
                    });
                });
                height.update(cx, |input, cx| {
                    input.get_state().update(cx, |input, cx| {
                        let value = (image_height_value as u64).to_string();
                        input.set_value(value, window, cx);
                    });
                });
            }
        },
    )
}
