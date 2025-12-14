mod basicrop;
mod basicrop_state;
mod counter_input;
mod image_crop;
mod main_view;
mod misc;
mod selection_canvas;

use basicrop::Basicrop;
use std::path::PathBuf;
// use std::time::{SystemTime, UNIX_EPOCH};
use gpui::{
    App, Application, Bounds, Size, TitlebarOptions, WindowBounds, WindowDecorations,
    WindowOptions, hsla, prelude::*, px, size,
};
use gpui_component::*;

fn main() {
    let mut args = std::env::args();
    let image_path: PathBuf = match args.nth(1) {
        Some(path) => path.into(),
        None => {
            eprintln!("error: missing input image");
            std::process::exit(1);
        }
    };
    let dest_image_path: PathBuf = match args.next() {
        Some(path) => path.into(),
        None => {
            let mut orig_path = image_path.to_str().unwrap().to_owned();
            let ext_index = orig_path.rfind('.').unwrap_or(orig_path.len());
            orig_path.insert_str(ext_index, ".cropped");
            PathBuf::from(orig_path)
        }
    };

    let app = Application::new().with_assets(gpui_component_assets::Assets);

    app.run(|cx: &mut App| {
        gpui_component::init(cx);
        Theme::global_mut(cx).window_border = hsla(0., 0., 0., 0.6);

        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                is_resizable: true,
                is_minimizable: true,
                window_decorations: Some(WindowDecorations::Server),
                window_min_size: Some(Size {
                    width: px(750.0),
                    height: px(500.0),
                }),
                titlebar: Some(TitlebarOptions {
                    title: Some("Basicrop".into()),
                    ..Default::default()
                }),
                app_id: Some("Basicrop".into()),
                ..Default::default()
            },
            |window, cx| {
                let view = cx.new(|cx| Basicrop::new(window, cx, image_path, dest_image_path));
                cx.new(|cx| Root::new(view, window, cx))
            },
        )
        .unwrap();
    });
}
