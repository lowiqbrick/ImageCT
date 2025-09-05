use eframe::{App, CreationContext, Frame};
use egui::{CentralPanel, Context};

#[derive(Default)]
struct MyApp {}

impl MyApp {
    fn new(_: &CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello egui");
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    if let Err(error) = eframe::run_native(
        "ImageCT",
        native_options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    ) {
        println!("{error}")
    }
}
