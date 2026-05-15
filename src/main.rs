// fn main() {
//     println!("Hello, world!");
// }

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, vec2};
use std::path::Path;

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
    selected_index: Option<usize>,
    is_preview_open: bool, // TODO: make switchable with Space and also load selected image; Also arrow navigation
    image_paths: Vec<String>,
}
impl Default for MyApp {
    fn default() -> Self {
        Self {
            selected_index: None,
            is_preview_open: false,
            image_paths: (0..8)
                .map(|i| format!("images/test{}.png", i))
                .collect::<Vec<_>>(),
        }
    }
}

fn wide_button(
    ui: &mut egui::Ui,
    text: impl Into<egui::WidgetText>,
    size: egui::Vec2,
) -> egui::Response {
    ui.add_sized(size, egui::Button::new(text))
}

enum MediaType {
    ImageGif,
    Video,
    Unknown,
}

fn get_media_filetype(path_in: &str) -> MediaType {
    let path = Path::new(path_in);
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase());
    match ext.as_deref() {
        Some("jpg") | Some("jpeg") | Some("png") | Some("gif") | Some("webp") | Some("bmp") => {
            MediaType::ImageGif
        }
        Some("mp4") | Some("mkv") | Some("mov") | Some("avi") | Some("webm") => MediaType::Video,
        _ => MediaType::Unknown,
    }
}

fn draw_media(ui: &mut egui::Ui, path_in: &str, tile_size: egui::Vec2) {
    let filetype = get_media_filetype(path_in);

    let library_abs_path = "/home/kf/Files/Programming/lable";
    let uri = format!("file://{}/{}", library_abs_path, path_in);
    // TODO: in the end will actually have absolute paths, but the path still set at runtime and not like that
    match filetype {
        MediaType::ImageGif => {
            ui.add(
                egui::Image::new(uri)
                    .fit_to_exact_size(tile_size)
                    .maintain_aspect_ratio(true), // .max_size(egui::vec2(150.0, 150.0))
            );
        }
        _ => {
            ui.label("Not an image");
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // let audio_device = egui_video::AudioDevice::new()?;

        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::Panel::left("left_panel")
                .min_size(250.0)
                .default_size(250.0)
                .show_inside(ui, |ui| {
                    ui.label("Filter");
                    egui::Panel::bottom("bottom_left_panel")
                        .min_size(360.0)
                        .show_inside(ui, |ui| {
                            let button_size = egui::vec2(ui.available_width(), 35.0);
                            ui.label("Utilities");

                            //TODO: backend calls
                            if wide_button(ui, "Clear filters", button_size).clicked() {}
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
                    egui::Panel::bottom("bottom_right_panel")
                        .min_size(360.0)
                        .show_inside(ui, |ui| {
                            let button_size = egui::vec2(ui.available_width(), 35.0);

                            ui.label("Image actions");
                            //TODO: backend calls
                            if wide_button(ui, "Open in default app", button_size).clicked() {}
                            if wide_button(ui, "Open containing folder", button_size).clicked() {}
                            if wide_button(ui, "Rename", button_size).clicked() {}
                            if wide_button(ui, "Delete", button_size).clicked() {
                                // Move to trash folder
                            }
                        });
                    egui::CentralPanel::default().show_inside(ui, |ui| {
                        ui.label("Image tags");
                    });
                });

            egui::CentralPanel::default().show_inside(ui, |ui| {
                let preview_size = ui.available_size();

                // TODO: make go OVER grid while still keeping it
                if self.is_preview_open {
                    draw_media(ui, "images/test1.gif", preview_size);
                }

                ui.vertical_centered(|ui| {
                    // let side = ui.available_height();
                    // let (rect, _) =
                    //     ui.allocate_at_least(egui::vec2(side, side), egui::Sense::hover());

                    // ui.painter().rect_filled(rect, 0.0, egui::Color32::GRAY);
                    let tile_size = egui::vec2(221.0, 221.0); // TODO: make ajustable
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            // Adjust spacing between tiles
                            ui.spacing_mut().item_spacing = egui::vec2(24.0, 24.0);
                            // TODO: make focusable
                            draw_media(ui, "images/test0.png", tile_size);
                            draw_media(ui, "images/test1.gif", tile_size);
                            draw_media(ui, "images/test2.jpg", tile_size);
                            draw_media(ui, "images/test3.png", tile_size);
                            draw_media(ui, "images/test4.jpg", tile_size);
                            draw_media(ui, "images/test5.gif", tile_size);
                        });
                    });
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
