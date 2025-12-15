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
            let image_width = i32::from(image_size.width) as f32;
            let image_height = i32::from(image_size.height) as f32;
            let viewport_aspect_ratio = bounds.size.width / bounds.size.height;
            let image_aspect_ratio = image_width / image_height;
            let image_visible_scale = if viewport_aspect_ratio > image_aspect_ratio {
                f32::from(bounds.size.height / image_height)
            } else {
                f32::from(bounds.size.width / image_width)
            };
            let image_visible_scale_inverse = 1. / image_visible_scale;
            let image_visible_width = px(image_width * image_visible_scale);
            let image_visible_height = px(image_height * image_visible_scale);

            // We need this because only the canvas' height might not match the
            // image's height, but the width will always match
            let bounds_padding_x = (image_visible_width.max(bounds.size.width) - image_visible_width.min(bounds.size.width)) / 2.;
            let bounds_padding_y = (image_visible_height.max(bounds.size.height) - image_visible_height.min(bounds.size.height)) / 2.;

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
                            .max(bounds.origin.x + bounds_padding_x)
                            .min(bounds.origin.x + bounds_padding_x + image_visible_width),
                        (mouse_initial_pos.y + bounds.origin.y)
                            .max(bounds.origin.y + bounds_padding_y)
                            .min(bounds.origin.y + bounds_padding_y + image_visible_height),
                    ),
                    point(
                        mouse_pos
                            .x
                            .max(bounds.origin.x + bounds_padding_x)
                            .min(bounds.origin.x + bounds_padding_x + image_visible_width),
                        mouse_pos
                            .y
                            .max(bounds.origin.y + bounds_padding_y)
                            .min(bounds.origin.y + bounds_padding_y + image_visible_height),
                    ),
                )
            } else {
                let image_visible_scale = image_visible_scale;
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

            let image_crop_x_value = (origin.x - bounds.origin.x - bounds_padding_x) * image_visible_scale_inverse;
            let image_crop_y_value = (origin.y - bounds.origin.y - bounds_padding_y) * image_visible_scale_inverse;
            let image_width_value = (se_corner.x - origin.x) * image_visible_scale_inverse;
            let image_height_value = (se_corner.y - origin.y) * image_visible_scale_inverse;

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
                    px(0.),
                    rgba(0x709ebe7f),
                    px(1.),
                    rgba(0x709ebeaf),
                    BorderStyle::default(),
                ));

                let new_image_crop = ImageCrop::Cropped {
                    crop_x: image_crop_x_value,
                    crop_y: image_crop_y_value,
                    width: image_width_value,
                    height: image_height_value,
                };
                if &new_image_crop != image_crop.read(cx) {
                    // println!("Updated crop: {:?}", new_image_crop);
                    image_crop.write(cx, new_image_crop);
                }
            } else {
                let occlusion_bounds = gpui::bounds(
                    point(
                        bounds.origin.x + bounds_padding_x,
                        bounds.origin.y + bounds_padding_y,
                    ),
                    size(
                        bounds.size.width - bounds_padding_x * 2.,
                        bounds.size.height - bounds_padding_y * 2.,
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
                        let value = u32::from(image_crop_x_value).to_string();
                        input.set_value(value, window, cx);
                    });
                });
                crop_y.update(cx, |input, cx| {
                    input.get_state().update(cx, |input, cx| {
                        let value = u32::from(image_crop_y_value).to_string();
                        input.set_value(value, window, cx);
                    });
                });
                width.update(cx, |input, cx| {
                    input.get_state().update(cx, |input, cx| {
                        let value = u32::from(image_width_value).to_string();
                        input.set_value(value, window, cx);
                    });
                });
                height.update(cx, |input, cx| {
                    input.get_state().update(cx, |input, cx| {
                        let value = u32::from(image_height_value).to_string();
                        input.set_value(value, window, cx);
                    });
                });
            }
        },
    )
}
