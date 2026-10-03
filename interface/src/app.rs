use eframe::{App, CreationContext, Frame};
use egui::{CentralPanel, ColorImage, Context, TextureOptions};
use image::{DynamicImage, ImageReader};

pub struct ImageProcessor {
    image_path: String,
    image: Option<DynamicImage>,
}

impl ImageProcessor {
    pub fn new(_: &CreationContext<'_>) -> Self {
        ImageProcessor {
            image_path: "".to_string(),
            image: None,
        }
    }
}

impl App for ImageProcessor {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        // draw window
        CentralPanel::default().show(ctx, |ui| {
            ui.label("enter filepath:");
            ui.vertical(|ui| ui.text_edit_singleline(&mut self.image_path));
            // load image
            self.image = match ImageReader::open(self.image_path.clone()) {
                Ok(opened_file) => match opened_file.decode() {
                    Ok(image) => Some(image),
                    Err(error) => {
                        ui.label(format!("couldn't decode image; {error}"));
                        None
                    }
                },
                Err(error) => {
                    ui.label(format!("image path leads nowhere; {error}"));
                    None
                }
            };
            // display image
            if let Some(ref saved_image) = self.image {
                // display image
                let rgba_image = saved_image.to_rgba8();
                let size = [rgba_image.width() as _, rgba_image.height() as _];
                let color_image = ColorImage::from_rgba_unmultiplied(size, &rgba_image);
                let texture_handle = ctx.load_texture("image", color_image, TextureOptions::LINEAR);
                ui.image(&texture_handle);
            }
        });
    }
}
