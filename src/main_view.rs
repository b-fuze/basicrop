use crate::accept_crop::{AcceptCrop, CancelCrop};
use crate::counter_input::number_field;
use crate::image_crop::CroppingState;
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

    // Main window root element
    div()
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
                .focusable()
                .flex()
                .flex_col()
                .justify_center()
                .items_center()
                .relative()
                .h_full()
                .w_full()
                .map({
                    let image_asset = image_asset.clone();
                    move |this| {
                        let this = match image_asset.clone() {
                            LoadingImage::Image(image) => this.child(
                                img(image)
                                    .absolute()
                                    .size_full()
                                    .object_fit(ObjectFit::Contain),
                            ),
                            LoadingImage::Failed => this.child("Failed to load image"),
                            LoadingImage::Loading => this.child("Loading image..."),
                        };

                        this
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
                    let image_crop = state.image_crop.clone();
                    move |_, _window, cx| {
                        is_selecting.write(cx, false);
                        cx.global_mut::<CroppingState>().image_crop =
                            Some(image_crop.read(&cx).clone());
                    }
                })
                .on_mouse_up_out(gpui::MouseButton::Left, {
                    let is_selecting = state.is_selecting.clone();
                    let image_crop = state.image_crop.clone();
                    move |_, _, cx| {
                        is_selecting.write(cx, false);
                        cx.global_mut::<CroppingState>().image_crop =
                            Some(image_crop.read(&cx).clone());
                    }
                })
                .on_key_up({
                    |evt, _window, cx| {
                        if evt.keystroke.key.as_str() == "enter" {
                            cx.defer(|cx| cx.dispatch_action(&AcceptCrop));
                        };
                    }
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
                            cx.defer(|cx| cx.dispatch_action(&CancelCrop));
                        }),
                )
                .child(
                    Button::new("confirm-btn")
                        .icon(IconName::Check)
                        .label("Ok")
                        .border_1()
                        .border_color(rgb(0xd0d0d0))
                        .on_click(move |_, _, cx| {
                            cx.defer(|cx| cx.dispatch_action(&AcceptCrop));
                        }),
                ),
        )
}
