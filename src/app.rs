use egui::widget_text;

use crate::oscillator;

// the app state
#[derive(serde::Deserialize, serde::Serialize)] // use serialize and deserialize to save and load state
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    // Example stuff:
    label: String,
    value: f32,

    #[serde(skip)] // we don't need to save this
    playing: bool,
}

// default app state
impl Default for App {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            playing: false,
        }
    }
}

impl App {
    // called once before the first frame
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // load previous app state if any
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn play_pause_button(&mut self, ui: &mut egui::Ui) {
        let (button_text, hover_text) = if self.playing {
            ("⏸", "Pause (Space)")
        } else {
            ("▶", "Play (Space)")
        };

        let widget_text = egui::RichText::new(button_text)
            .text_style(egui::TextStyle::Heading)
            .color(egui::Color32::from_rgb(255, 255, 255));

        let button = egui::Button::new(widget_text)
            .min_size([50.0, 50.0].into())
            .fill(egui::Color32::from_rgb(0, 64, 0));

        let clicked = ui.add(button).on_hover_text(hover_text).clicked();
        let space_pressed = ui.input(|i| i.key_pressed(egui::Key::Space));

        if clicked || space_pressed {
            self.playing = !self.playing;
        }
    }
}

impl eframe::App for App {
    // save app state
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            label,
            value,
            playing,
        } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // the menu bar
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |_ui| {
                    // quit button (only available when compiling natively)
                    #[cfg(not(target_arch = "wasm32"))]
                    if _ui.button("Quit").clicked() {
                        _frame.close();
                    }

                    // // save/load buttons
                    // if ui.button("Save").clicked() {
                    //     frame
                    //         .open_native_file_selector_for_write("Save Settings", ".json", None);
                    // }

                    // if ui.button("Load").clicked() {
                    //     frame.open_native_file_selector_for_read("Load Settings", ".json");
                    // }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // the central panel, after adding the top and left panels

            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);

            // bottom panel for player controls
            // side by side and centered player buttons for play/pause, stop, and reset, with margins on the sides

            egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
                // the middle 1/3 of the bottom panel, for the player buttons
                ui.vertical_centered(|ui| {
                    self.play_pause_button(ui);
                });
            });
        });

        if true {
            oscillator::ui(ctx);
        }
    }
}
