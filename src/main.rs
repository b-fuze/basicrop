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

argyle::argue! {
    /// basicrop is a basic program to crop images. It will open
    /// the source-image in a window that allows cropping by
    /// clicking and dragging anywhere on the image. After clicking
    /// the "Ok" button it will save the cropped image to
    /// output-image if provided, or to the same path as
    /// source-image with .cropped appended to the file name before
    /// the file extension.
    ///
    /// Supported image formats:
    ///   AVIF  BMP      Farbfeld
    ///   GIF   HDR      ICO
    ///   JPEG  OpenEXR  PNG
    ///   PNM   QOI      TGA
    ///   TIFF  WebP
    Argument,
    ArgumentIter,
    Help  "-h" "--help",
    Force "-f" "--force",
}

fn main() {
    let mut force = false;
    let mut image_path: Option<PathBuf> = None;
    let mut dest_image_path: Option<PathBuf> = None;

    for arg in Argument::args_os() {
        match arg {
            Argument::Help => print_help(),
            Argument::Force => {
                force = true;
            }
            Argument::Other(v) if image_path.is_none() => {
                image_path = Some(v.into());
            }
            Argument::Other(v) if dest_image_path.is_none() => {
                dest_image_path = Some(v.into());
            }
            Argument::Other(v) => {
                panic!("Encountered extra argument: {}", v);
            }
            Argument::OtherOs(v) => {
                panic!("Encountered invalid UTF-8 string: {}", v.to_string_lossy());
            }
        }
    }

    let Some(image_path) = image_path else {
        eprintln!("No image path specified");
        panic!("{USAGE}");
    };
    let dest_image_path = dest_image_path.unwrap_or_else(|| {
        let mut orig_path = image_path.to_str().unwrap().to_owned();
        let ext_index = orig_path.rfind('.').unwrap_or(orig_path.len());
        orig_path.insert_str(ext_index, ".cropped");
        PathBuf::from(orig_path)
    });

    let app = Application::new().with_assets(gpui_component_assets::Assets);

    app.run(move |cx: &mut App| {
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
                let view =
                    cx.new(|cx| Basicrop::new(window, cx, image_path, dest_image_path, force));
                cx.new(|cx| Root::new(view, window, cx))
            },
        )
        .unwrap();
    });
}

fn print_help() {
    eprintln!("{USAGE}");
    std::process::exit(0);
}
