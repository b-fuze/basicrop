use crate::counter_input::number_field;
use crate::misc::LoadingImage;
use crate::selection_canvas::selection_canvas;
use crate::{basicrop_state::BasicropState, misc::CroppingMousePosition};
use gpui::{Context, Edges, IntoElement, ObjectFit, Styled, div, img, prelude::*, px, rgb};
use gpui_component::IconName;
use gpui_component::{StyledExt, button::Button};

pub fn render_main_view<T>(
    state: &mut BasicropState,
    image_asset: LoadingImage,
    cx: &mut Context<T>,
) -> impl IntoElement {
    let fields = div()
        .flex()
        .flex_row()
        .w_full()
        .gap_3()
        .items_center()
        .justify_start()
        .paddings(Edges {
            top: px(16.),
            bottom: px(16.),
            left: px(0.),
            right: px(0.),
        })
        .child(number_field("X:", state.crop_x.read(cx).get_state()))
        .child(number_field("Y:", state.crop_y.read(cx).get_state()))
        .child(number_field("Width:", state.width.read(cx).get_state()))
        .child(number_field("Height:", state.height.read(cx).get_state()));

    cx.bind_keys([gpui::KeyBinding::new(
        "enter",
        crate::actions::CropImage,
        None,
    )]);
    cx.bind_keys([gpui::KeyBinding::new(
        "escape",
        crate::actions::CancelCrop,
        None,
    )]);

    // Main window root element
    div()
        .id("main-window-root-element")
        .focusable()
        .flex()
        .flex_col()
        .bg(rgb(0xfafafa))
        .justify_start()
        .items_center()
        .size_full()
        .content_stretch()
        .relative()
        .shadow_lg()
        .text_sm()
        .text_color(rgb(0x202020))
        .child(
            div()
                .flex()
                .flex_row()
                .justify_center()
                .items_center()
                .w_full()
                // .gap_full()
                .paddings(Edges {
                    top: px(0.),
                    bottom: px(0.),
                    left: px(16.),
                    right: px(16.),
                })
                .border_b(px(1.))
                .border_color(rgb(0xd0d0d0))
                .child(fields)
                .child(
                    div().flex().flex_row().justify_end().w(px(100.)).child(
                        Button::new("reset-btn")
                            .label("Reset")
                            .border_1()
                            .border_color(rgb(0xd0d0d0))
                            .on_click({
                                let fields = [
                                    state.crop_x.clone(),
                                    state.crop_y.clone(),
                                    state.width.clone(),
                                    state.height.clone(),
                                ];
                                let new_values = match &image_asset {
                                    LoadingImage::Image(image) => {
                                        let size = image.size(0);
                                        [0u32, 0u32, size.width.into(), size.height.into()]
                                    }
                                    _ => [0u32, 0u32, 0u32, 0u32],
                                };
                                move |_, window, cx| {
                                    for (field, value) in fields.iter().zip(new_values) {
                                        field.update(cx, |input, cx| {
                                            input.get_state().update(cx, |input, cx| {
                                                input.set_value(value.to_string(), window, cx);
                                            });
                                        });
                                    }
                                }
                            }),
                    ),
                ),
        )
        .child(
            div()
                .id("image_viewport")
                .flex()
                .flex_col()
                .justify_center()
                .items_center()
                .relative()
                .h_full()
                .w_full()
                .map({
                    let image_asset = image_asset.clone();
                    move |this| match image_asset.clone() {
                        LoadingImage::Image(image) => this.child(
                            img(image)
                                .absolute()
                                .size_full()
                                .object_fit(ObjectFit::Contain),
                        ),
                        LoadingImage::Failed => this.child("Failed to load image"),
                        LoadingImage::Loading => this.child("Loading image..."),
                    }
                })
                .child(
                    selection_canvas(
                        state.crop_x.clone(),
                        state.crop_y.clone(),
                        state.width.clone(),
                        state.height.clone(),
                        image_asset.clone(),
                        state.image_crop.clone(),
                        state.is_selecting.clone(),
                        state.mouse_pos.clone(),
                        state.mouse_initial_pos.clone(),
                    )
                    .absolute()
                    .size_full(),
                )
                .on_drag((), {
                    let is_selecting = state.is_selecting.clone();
                    let mouse_pos = state.mouse_pos.clone();
                    let mouse_initial_pos = state.mouse_initial_pos.clone();
                    move |_, point, _window, cx| {
                        // `point` is relative to this element's bounds
                        mouse_pos.write(cx, CroppingMousePosition::Initial(point));
                        mouse_initial_pos.write(cx, point);
                        is_selecting.write(cx, true);

                        cx.new(|_| gpui::Empty)
                    }
                })
                .on_drag_move::<()>({
                    let mouse_pos = state.mouse_pos.clone();
                    move |evt, _window, cx| {
                        let position = evt.event.position;
                        mouse_pos.write(cx, CroppingMousePosition::Moved(position));
                    }
                })
                .on_drop::<()>({
                    let is_selecting = state.is_selecting.clone();
                    move |_, _window, cx| {
                        is_selecting.write(cx, false);
                    }
                })
                .on_mouse_up_out(gpui::MouseButton::Left, {
                    let is_selecting = state.is_selecting.clone();
                    move |_, _, cx| is_selecting.write(cx, false)
                }),
        )
        .child(
            div()
                .flex()
                .w_full()
                .h_16()
                .justify_end()
                .paddings(Edges {
                    top: px(12.),
                    bottom: px(12.),
                    left: px(16.),
                    right: px(16.),
                })
                .gap_4()
                .child(
                    Button::new("cancel-btn")
                        .icon(IconName::Close)
                        .label("Cancel")
                        .border_1()
                        .border_color(rgb(0xd0d0d0))
                        .on_click(|_, _, cx| {
                            println!("info: image crop canceled");
                            cx.shutdown();
                        }),
                )
                .child(
                    Button::new("confirm-btn")
                        .icon(IconName::Check)
                        .label("Ok")
                        .border_1()
                        .border_color(rgb(0xd0d0d0))
                        .on_click({
                            let image_asset = image_asset.clone();
                            let state = state.clone();
                            move |_, _, cx| finalize_crop(cx, &state, &image_asset)
                        }),
                ),
        )
        .on_action({
            let image_asset = image_asset.clone();
            let state = state.clone();
            move |_: &crate::actions::CropImage, _, cx| finalize_crop(cx, &state, &image_asset)
        })
        .on_action(|_: &crate::actions::CancelCrop, _, cx| {
            println!("info: image crop canceled via Escape");
            cx.shutdown();
        })
}

fn finalize_crop(cx: &mut gpui::App, state: &BasicropState, image_asset: &LoadingImage) {
    let image_crop = state.image_crop.clone();
    let image_crop_initial = state.image_crop_initial.clone();
    let image_asset = image_asset.clone();
    let dest_image_path = state.dest_image_path.clone();
    let image_saved_notification = state.image_saved_notification.clone();

    if image_crop.read(cx) == image_crop_initial.read(cx) {
        println!("info: image not cropped");
        image_saved_notification.write(cx, ());
        return;
    }

    let image_crop_logged = image_crop.read(cx).clone().to_final().unwrap();
    println!(
        "info: cropping image with inputs: x: {}, y: {}, dimensions: {}x{}",
        image_crop_logged.crop_x,
        image_crop_logged.crop_y,
        image_crop_logged.width,
        image_crop_logged.height,
    );

    if let (Some(final_crop), LoadingImage::Image(image)) =
        (image_crop.read(cx).to_final(), &image_asset)
    {
        let image_size = image.size(0);
        let cropped_image_buf: Option<image::ImageBuffer<image::Rgba<_>, Vec<_>>> =
            image::ImageBuffer::from_raw(
                image_size.width.into(),
                image_size.height.into(),
                image.as_bytes(0).unwrap().to_vec(),
            );

        if let Some(mut cropped_image_buf) = cropped_image_buf {
            let dest_path = dest_image_path.read(cx).clone();
            let image_saved_notification = image_saved_notification.clone();
            cx.spawn(async move |cx: &mut gpui::AsyncApp| {
                cx.background_spawn(async move {
                    let mut cropped_image_buf = image::imageops::crop(
                        &mut cropped_image_buf,
                        final_crop.crop_x,
                        final_crop.crop_y,
                        final_crop.width,
                        final_crop.height,
                    )
                    .to_image();

                    // Convert from RGBA to BGRA.
                    for pixel in cropped_image_buf.as_chunks_mut::<4>().0 {
                        pixel.swap(0, 2);
                    }

                    let image_type = dest_path
                        .components()
                        .map(|component| component.as_os_str().to_str().unwrap().to_string())
                        .next_back()
                        .unwrap()
                        .rsplit('.')
                        .next_back()
                        .unwrap()
                        .to_lowercase();
                    let saved_image = match image_type.as_str() {
                        "png" | "webp" => image::save_buffer(
                            &dest_path,
                            cropped_image_buf.into_raw().as_slice(),
                            final_crop.width,
                            final_crop.height,
                            image::ExtendedColorType::Rgba8,
                        ),
                        _ => image::save_buffer(
                            &dest_path,
                            image::DynamicImage::ImageRgba8(cropped_image_buf)
                                .to_rgb8()
                                .into_raw()
                                .as_slice(),
                            final_crop.width,
                            final_crop.height,
                            image::ExtendedColorType::Rgb8,
                        ),
                    };

                    match saved_image {
                        Ok(_) => {
                            println!(
                                "info: cropped and saved image successfully to: {}",
                                dest_path.to_str().unwrap_or("[invalid_str]")
                            );
                        }
                        Err(error) => {
                            eprintln!("error: failed to save cropped image: {:?}", error);
                        }
                    };
                })
                .await;

                let _ = image_saved_notification.write(cx, ());
            })
            .detach();
        } else {
            eprintln!("error: can't retrieve image buffer for cropping");
            cx.shutdown();
        }
    } else {
        eprintln!("warn: can't save file due to uninitialized image");
        cx.shutdown();
    }
}
