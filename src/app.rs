use std::sync::Arc;

use egui::{
    style::Selection, Color32, CornerRadius, Label, Pos2, Rect, RichText, Rounding, Stroke,
    TextEdit, Visuals,
};
use egui_extras::{Column, TableBuilder};

#[derive(serde::Deserialize, serde::Serialize)]
struct Employee {
    id: usize,
    code: String,
    first_name: String,
    last_name: String,
    department: String,
    in_base: bool,
    last_timestamp: usize,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    #[serde(skip)] // This how you opt-out of serialization of a field
    id_input: String,
    #[serde(skip)] // This how you opt-out of serialization of a field
    input_result: String,

    is_emergency: bool,
    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    employee_buffer: Vec<Employee>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            id_input: "".to_owned(),
            input_result: "".to_owned(),
            is_emergency: false,
            value: 2.7,
            employee_buffer: Vec::new(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "custom_font".to_owned(),
            Arc::new(egui::FontData::from_static(include_bytes!(
                "../assets/plex.ttf"
            ))),
            //egui::FontData::from_static(include_bytes!("../assets/dejavu.ttf")),
        );
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "custom_font".to_owned());

        //egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::variants::Variant::Regular);

        egui_extras::install_image_loaders(&cc.egui_ctx);
        cc.egui_ctx.set_fonts(fonts);

        // Configuring visuals.

        let mut visuals = Visuals::light();
        visuals.selection = Selection {
            bg_fill: Color32::from_rgb(81, 129, 154),
            stroke: Stroke::new(1.0, Color32::WHITE),
        };

        visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(180, 180, 180);
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(180, 180, 180);
        visuals.widgets.inactive.corner_radius = CornerRadius::ZERO;
        visuals.widgets.noninteractive.corner_radius = CornerRadius::ZERO;
        visuals.widgets.active.corner_radius = CornerRadius::ZERO;
        visuals.widgets.hovered.corner_radius = CornerRadius::ZERO;
        visuals.window_corner_radius = CornerRadius::ZERO;
        visuals.window_fill = Color32::from_rgb(197, 197, 197);
        visuals.menu_corner_radius = CornerRadius::ZERO;
        visuals.panel_fill = Color32::from_rgb(200, 200, 200);
        visuals.striped = true;
        visuals.slider_trailing_fill = true;

        cc.egui_ctx.set_visuals(visuals);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    if self.is_emergency {
                        ui.add(Label::new(
                            RichText::new(format!("    EMERGENCY    "))
                                .size(14.)
                                .strong()
                                .background_color(Color32::RED)
                                .italics(),
                        ));
                    }
                    // let rect = Rect::from_min_max(Pos2::new(0.0, 20.0), Pos2::new(200.0, 100.0));
                    // ui.horizontal(|ui| {});
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading(format!("{}", self.input_result));

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                    if !self.id_input.is_empty() {
                        self.input_result = self.id_input.clone();
                    }
                }
                let edit = TextEdit::singleline(&mut self.id_input).lock_focus(true);
                ui.add(edit).request_focus();

                //ui.text_edit_singleline(&mut self.label).request_focus();
            });

            ui.separator();

            let available_height = ui.available_height();
            let table = TableBuilder::new(ui)
                .striped(true)
                .stick_to_bottom(true)
                .scroll_to_row(self.employee_buffer.len(), Some(egui::Align::BOTTOM))
                .resizable(false)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::exact(100.0))
                .column(Column::exact(200.0))
                .column(Column::exact(200.0))
                .column(Column::exact(200.0))
                .column(Column::remainder())
                .min_scrolled_height(0.0)
                .max_scroll_height(available_height);

            table
                .header(40.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("ID");
                    });
                    header.col(|ui| {
                        ui.strong("FIRST NAME");
                    });
                    header.col(|ui| {
                        ui.strong("LAST NAME");
                    });
                    header.col(|ui| {
                        ui.strong("DEPARTMENT");
                    });
                    header.col(|ui| {
                        ui.strong("STATUS");
                    });
                })
                .body(|mut body| {
                    let row_height = 20.0;
                    let num_rows = 100;
                    body.rows(row_height, num_rows, |mut row| {
                        let index = row.index();
                        row.col(|ui| {
                            ui.label(format!("{index}"));
                        });
                        row.col(|ui| {
                            ui.label(format!("ABDELKADER"));
                        });
                        row.col(|ui| {
                            ui.label(format!("MADOUI"));
                        });
                        row.col(|ui| {
                            ui.label(format!("DAQ"));
                        });
                        row.col(|ui| {
                            ui.label(format!("DAQ"));
                        });
                    });
                });
            // ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            //     powered_by_egui_and_eframe(ui);
            //     egui::warn_if_debug_build(ui);
            // });
        });
        egui::SidePanel::right("right")
            .min_width(200.0)
            .max_width(200.0)
            .default_width(200.0)
            .resizable(false)
            .show(ctx, |ui| {
                // egui::Image::new(egui::include_image!("../assets/logo.png"))
                //     .paint_at(ui, ctx.available_rect());
                ui.add(egui::Image::new(egui::include_image!("../assets/logo.png")));
                ui.vertical_centered(|ui| {});
            });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
