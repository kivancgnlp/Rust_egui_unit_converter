#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Temperature Unit Converter",
        options,
        Box::new(|cc| {
            // This gives us image support:
            //egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    temperature_in_celsius: i32,
    height_in_cm : usize,
    val_str : String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            temperature_in_celsius: 42,
            height_in_cm: 172,
            val_str: "Empty".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Temperature Converter");
            ui.separator();
            ui.horizontal(|ui| {
                let name_label = ui.label("Temp in celsius : ");
                ui.add(egui::Slider::new(&mut self.temperature_in_celsius, -50..=150).text("C"));

            });
            
            ui.horizontal(|ui| {
                let name_label = ui.label("Temp in fahrenheit : ");
                let fahrenheit = self.temperature_in_celsius as f64 * 1.8 + 32.0;
                ui.label(format!("Fahrenheit : {:.2}", fahrenheit));

            });

            ui.heading("Height Converter");
            ui.separator();
            ui.horizontal(|ui| {
                let name_label = ui.label("Height in cm : ");
                ui.add(egui::Slider::new(&mut self.height_in_cm, 0..=250).text("cm"));

            });

            ui.horizontal(|ui| {
                let name_label = ui.label("Height in US standard : ");
                let inch = self.height_in_cm as f64 / 2.54;

                let feet = (inch / 12.0) as u32;
                let inch_remainder = inch - (feet as f64 * 12.0);
                ui.label(format!("{} foot {:.2}", feet,inch_remainder));

            });


        });
    }
}