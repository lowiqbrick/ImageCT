mod app;

use app::ImageProcessor;

fn main() {
    let native_options = eframe::NativeOptions::default();
    if let Err(error) = eframe::run_native(
        "ImageCT",
        native_options,
        Box::new(|cc| Ok(Box::new(ImageProcessor::new(cc)))),
    ) {
        println!("{error}")
    }
}
