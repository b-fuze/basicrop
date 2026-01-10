use gpui::{App, AppContext, actions};

use crate::image_crop::CroppingState;

actions!([AcceptCrop, CancelCrop, Quit]);

pub fn on_accept_crop(_: &AcceptCrop, cx: &mut App) {
    let state: &CroppingState = cx.global();
    let dest_image_path = state.dest_path.clone();

    if let (Some(image_crop), Some(image_crop_initial), Some(image)) = (
        state.image_crop.clone(),
        state.image_initial.clone(),
        state.image.clone(),
    ) {
        if image_crop == image_crop_initial {
            cx.defer(|cx| cx.dispatch_action(&Quit));
            return;
        }

        let image_crop_logged = image_crop.to_final().unwrap();
        println!(
            "info: cropping image with inputs: x: {}, y: {}, dimensions: {}x{}",
            image_crop_logged.crop_x,
            image_crop_logged.crop_y,
            image_crop_logged.width,
            image_crop_logged.height,
        );

        if let Some(final_crop) = image_crop.to_final() {
            let image_size = image.size(0);
            let cropped_image_buf: Option<image::ImageBuffer<image::Rgba<_>, Vec<_>>> =
                image::ImageBuffer::from_raw(
                    image_size.width.into(),
                    image_size.height.into(),
                    image.as_bytes(0).unwrap().to_vec(),
                );

            if let Some(mut cropped_image_buf) = cropped_image_buf {
                let dest_path = dest_image_path.clone();
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
                            .last()
                            .unwrap()
                            .rsplit('.')
                            .last()
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

                    let _ = cx.update(|cx| {
                        cx.shutdown();
                    });
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
}
