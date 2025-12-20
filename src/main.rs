mod actions;
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

const USAGE: &str = r#"USAGE
    basicrop [-h|--help]
             source-image [output-image]

DESCRIPTION
    basicrop is a basic program to crop images. It will open
    the source-image in a window that allows cropping by
    clicking and dragging anywhere on the image. After clicking
    the "Ok" button it will save the cropped image to
    output-image if provided, or to the same path as
    source-image with .cropped appended to the file name before
    the file extension.

    Supported image formats:
      AVIF  BMP      Farbfeld
      GIF   HDR      ICO
      JPEG  OpenEXR  PNG
      PNM   QOI      TGA
      TIFF  WebP
"#;

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).take(2).collect();

    if args.is_empty() {
        eprintln!("error: missing source-image\n");
        eprint!("{USAGE}");
        std::process::exit(1);
    }

    if args[0] == "-h" || args[0] == "--help" {
        eprint!("{USAGE}");
        std::process::exit(0);
    }

    let image_path = PathBuf::from(args.remove(0));
    let dest_image_path: PathBuf = match args.into_iter().next() {
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

        let bounds = Bounds::centered(None, size(px(500.), px(500.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                is_resizable: true,
                is_minimizable: true,
                window_decorations: Some(WindowDecorations::Server),
                window_min_size: Some(Size {
                    width: px(750.),
                    height: px(500.),
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
