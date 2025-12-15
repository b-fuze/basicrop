use gpui::{Context, Entity, Subscription, Window, div, prelude::*, px};
use gpui_component::Sizable;
use gpui_component::input::{InputEvent, InputState, NumberInput, NumberInputEvent, StepAction};

use crate::image_crop::{ImageCrop, InitializedImageCrop};

pub struct CounterView {
    pub counter_input: Entity<InputState>,
    counter_value: i32,
    _subscriptions: Vec<Subscription>,
}

impl CounterView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>, default_value: i32) -> Self {
        let counter_input = cx.new(
            |cx| {
                InputState::new(window, cx)
                    .placeholder("Count")
                    .default_value(default_value.to_string())
            }, // .pattern(Regex::new(r"^-?\d+$").unwrap()) // Allow negative integers
        );

        let _subscriptions = vec![
            cx.subscribe_in(&counter_input, window, Self::on_number_event),
            cx.subscribe_in(&counter_input, window, Self::on_input_event),
        ];

        Self {
            counter_input,
            counter_value: default_value,
            _subscriptions,
        }
    }

    pub fn get_state(&self) -> Entity<InputState> {
        self.counter_input.clone()
    }

    /// Subscribes to the number input and auto-detaches the subscription. Should not be
    /// used if inputs are created on-demand
    pub fn subscribe<T: 'static>(
        &mut self,
        window: &mut Window,
        cx: &mut Context<T>,
        image_crop: Entity<ImageCrop>,
        mut on_event: impl FnMut(u32, InitializedImageCrop, &mut Context<T>) + 'static,
    ) {
        cx.subscribe_in(
            &self.counter_input,
            window,
            move |_, input, evt: &InputEvent, _, cx| match evt {
                InputEvent::Change => {
                    let value = input.read(cx).value().parse::<u32>();
                    if let (Ok(value), Some(initialized_image_crop)) =
                        (value, image_crop.read(cx).to_initialized())
                    {
                        on_event(value, initialized_image_crop, cx);
                    }
                }
                _ => {}
            },
        )
        .detach();
    }

    fn on_input_event(
        &mut self,
        state: &Entity<InputState>,
        event: &InputEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            InputEvent::Change => {
                let text = state.read(cx).value();
                if let Ok(value) = text.parse::<i32>() {
                    self.counter_value = value;
                }
            }
            _ => {}
        }
    }

    fn on_number_event(
        &mut self,
        state: &Entity<InputState>,
        event: &NumberInputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            NumberInputEvent::Step(StepAction::Increment) => {
                self.counter_value += 1;
                state.update(cx, |input, cx| {
                    input.set_value(self.counter_value.to_string(), window, cx);
                });
            }
            NumberInputEvent::Step(StepAction::Decrement) => {
                self.counter_value -= 1;
                state.update(cx, |input, cx| {
                    input.set_value(self.counter_value.to_string(), window, cx);
                });
            }
        }
    }
}

pub fn number_field(label: &str, state: Entity<InputState>) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .gap_2()
        .items_center()
        .child(label.to_string())
        .child(
            div()
                .flex()
                .flex_row()
                .w(px(100.))
                .child(NumberInput::new(&state).small()),
        )
}
