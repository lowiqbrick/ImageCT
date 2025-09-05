use eframe::{App, CreationContext, Frame};
use egui::{CentralPanel, Context, SidePanel};

pub struct ImageProcessor {
    _image_path: String,
    total_width: Option<f32>,
}

impl ImageProcessor {
    pub fn new(_: &CreationContext<'_>) -> Self {
        ImageProcessor {
            _image_path: "".to_string(),
            total_width: None,
        }
    }
}

impl App for ImageProcessor {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        let working_width = self.total_width.unwrap_or(400.0);
        SidePanel::left("left panel")
            .exact_width(working_width / 2.0)
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("TODO\nadd options");
                });
            });
        SidePanel::right("right panel")
            .exact_width(working_width / 2.0)
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.label("TODO\nadd preview images");
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
