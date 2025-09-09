use eframe::{App, CreationContext, Frame};
use egui::{CentralPanel, Context, Image, SidePanel, TextureOptions, load::SizedTexture};
use image::{DynamicImage, ImageReader};

pub struct ImageProcessor {
    image_path: String,
    image: Option<DynamicImage>,
    total_width: Option<f32>,
}

impl ImageProcessor {
    pub fn new(_: &CreationContext<'_>) -> Self {
        ImageProcessor {
            image_path: "".to_string(),
            image: None,
            total_width: None,
        }
    }
}

impl App for ImageProcessor {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        // draw window
        let working_width = self.total_width.unwrap_or(400.0);
        SidePanel::left("left panel")
            .exact_width(working_width / 2.0)
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.label("enter filepath:");
                ui.vertical(|ui| ui.text_edit_singleline(&mut self.image_path));
                // open image
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
            });

        SidePanel::right("right panel")
            .exact_width(working_width / 2.0)
            .show_separator_line(false)
            .show(ctx, |ui| {
                if self.image.is_some() {
                    let color_image = egui::ColorImage::from_rgb(
                        [
                            self.image.as_ref().unwrap().width() as usize,
                            self.image.as_ref().unwrap().height() as usize,
                        ],
                        self.image.as_ref().unwrap().as_bytes(),
                    );
                    let handle = ctx.load_texture(
                        self.image_path.clone(),
                        color_image.clone(),
                        TextureOptions::default(),
                    );
                    let sized_image = SizedTexture::new(
                        handle.id(),
                        egui::vec2(color_image.size[0] as f32, color_image.size[1] as f32),
                    );
                    let texture = Image::from_texture(sized_image);
                    ui.add(texture);
                }
            });
        // the central panel is not used to display anything
        // it's purpose is to give the total current width of the window
        let central_panel = CentralPanel::default().show(ctx, |_| {
            let rect = ctx.screen_rect();
            rect.max.x - rect.min.x
        });
        self.total_width = Some(central_panel.inner);
    }
}
