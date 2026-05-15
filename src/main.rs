// fn main() {
//     println!("Hello, world!");
// }

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 600.0])
            .with_min_inner_size([800.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Lable - image library tagger",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    // name: String,
    // age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            // name: "Arthur".to_owned(),
            // age: 42,
        }
    }
}

fn wide_button(ui: &mut egui::Ui, text: impl Into<egui::WidgetText>, size: egui::Vec2) -> egui::Response {
    ui.add_sized(size, egui::Button::new(text))
}

impl eframe::App for MyApp {

    

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::Panel::left("left_panel")
                .min_size(250.0)
                .default_size(250.0)
                .show_inside(ui, |ui| {
                    ui.label("Filter");
                    egui::Panel::bottom("bottom_left_panel").min_size(360.0).show_inside(ui, |ui| {
                        let button_size = egui::vec2(ui.available_width(), 35.0);
                        ui.label("Utilities");

                        //TODO: backend calls
                        if  wide_button(ui, "Clear filters", button_size).clicked() { }
                        if wide_button(ui, "Detect duplicates", button_size).clicked() { 
                            // apply sys:duplicate:x or something
                        }
                        if wide_button(ui, "Infer tags", button_size).clicked() {
                            // read sidecars, create tags, delete sidecars
                         }
                        if wide_button(ui, "Move to subolder", button_size).clicked() { 
                            //subfolder name popup, mkdir, update db, do it
                        }

                    });
                    egui::CentralPanel::default().show_inside(ui, |ui| {
                        ui.label("Filter tags");

                    });
                });

            egui::Panel::right("right_panel")
                .min_size(250.0)
                .default_size(250.0)
                .show_inside(ui, |ui| {
                    ui.label("Tags");
                    egui::Panel::bottom("bottom_right_panel").min_size(360.0).show_inside(ui, |ui| {
                        let button_size = egui::vec2(ui.available_width(), 35.0);

                        ui.label("Image actions");
                          //TODO: backend calls
                        if wide_button(ui, "Open in default app", button_size).clicked() { }
                        if wide_button(ui, "Open containing folder", button_size).clicked() { }
                        if wide_button(ui, "Rename", button_size).clicked() { }
                        if wide_button(ui, "Delete", button_size).clicked() {
                            // Move to trash folder
                         }
                    });
                    egui::CentralPanel::default().show_inside(ui, |ui| {
                        ui.label("Image tags");

                      

                    });
                });

            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    // let side = ui.available_height();
                    // let (rect, _) =
                    //     ui.allocate_at_least(egui::vec2(side, side), egui::Sense::hover());

                    // ui.painter().rect_filled(rect, 0.0, egui::Color32::GRAY);
                    ui.add(
                        egui::Image::new(egui::include_image!("../images/test.png"))
                            .corner_radius(5),
                    );
                });
            });

            // ui.horizontal(|ui| {
            //     ui.vertical(|ui| {
            //         ui.label("over");
            //         ui.label("under");
            //     });
            //     ui.vertical(|ui| {
            //         ui.label("over");
            //         ui.label("under");
            //     });
            //     ui.vertical(|ui| {
            //         ui.label("over");
            //         ui.label("under");
            //     });
            // })
            // ui.horizontal(|ui| {
            //     let name_label = ui.label("Your name: ");
            //     ui.text_edit_singleline(&mut self.name)
            //         .labelled_by(name_label.id);
            // });
            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            // if ui.button("Increment").clicked() {
            //     self.age += 1;
            // }
            // ui.label(format!("Hello '{}', age {}", self.name, self.age));

            // ui.image(egui::include_image!(
            //     "../images/test.png"
            // ));
        });
    }
}
