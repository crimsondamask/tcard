//use rusqlite::Connection;

use anyhow::{anyhow, Result};
//use anyhow::Result;
use crossbeam_channel::{Receiver, Sender};
use mysql::prelude::*;
use mysql::*;
//use sqlx::mysql::MySqlPool;
use std::{collections::HashSet, io::Write, sync::Arc};

use chrono::DateTime;
use egui::{
    style::Selection, Button, Color32, CornerRadius, Label, RichText, Stroke, TextEdit, Vec2,
    Visuals,
};
use egui_extras::{Column, TableBuilder};

#[derive(serde::Deserialize, serde::Serialize)]
struct CheckError {
    is_error: bool,
    err_msg: String,
}
#[derive(Debug, Hash, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
struct Employee {
    id: String,
    name: String,
    department: String,
    title: String,
    expro_id: String,
    field: String,
    category: String,
    in_base: usize,
    last_timestamp: usize,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct Emergency {
    on_base_total: usize,
    //on_base_list: Vec<Employee>,
    //count_list: Vec<Employee>,
    #[serde(skip)] // This how you opt-out of serialization of a field
    missing_list: Vec<Employee>,
    #[serde(skip)] // This how you opt-out of serialization of a field
    all_employees_hash: HashSet<Employee>,
    #[serde(skip)] // This how you opt-out of serialization of a field
    present_employees_hash: HashSet<Employee>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    #[serde(skip)] // This how you opt-out of serialization of a field
    last_log_dump: i64,
    #[serde(skip)] // This how you opt-out of serialization of a field
    log_dump_debounced: bool,
    #[serde(skip)] // This how you opt-out of serialization of a field
    scanned_employee_name: String,
    #[serde(skip)] // This how you opt-out of serialization of a field
    count_pressed: bool,
    #[serde(skip)] // This how you opt-out of serialization of a field
    reset_pressed: bool,
    emergency: Emergency,
    #[serde(skip)] // This how you opt-out of serialization of a field
    emergency_base_count: usize,
    #[serde(skip)] // This how you opt-out of serialization of a field
    about_show: bool,
    #[serde(skip)] // This how you opt-out of serialization of a field
    locked: bool,
    #[serde(skip)] // This how you opt-out of serialization of a field
    send_channel: Option<Sender<Option<String>>>,
    #[serde(skip)] // This how you opt-out of serialization of a field
    receive_channel: Option<Receiver<Option<Employee>>>,

    db_url: String,
    #[serde(skip)] // This how you opt-out of serialization of a field
    first_frame: bool,
    #[serde(skip)] // This how you opt-out of serialization of a field
    id_input: String,
    #[serde(skip)] // This how you opt-out of serialization of a field
    input_result: String,

    is_emergency: bool,
    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    employee_buffer: Vec<Employee>,
    emergency_buffer: Vec<Employee>,
    #[serde(skip)] // This how you opt-out of serialization of a field
    id_check: CheckError,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            last_log_dump: 0,
            log_dump_debounced: false,
            // Example stuff:
            scanned_employee_name: "".to_owned(),
            count_pressed: false,
            reset_pressed: false,
            emergency: Emergency {
                on_base_total: 0,
                //on_base_list: Vec::new(),
                //count_list: Vec::new(),
                missing_list: Vec::new(),
                all_employees_hash: HashSet::new(),
                present_employees_hash: HashSet::new(),
            },
            emergency_base_count: 0,
            about_show: false,
            locked: true,
            send_channel: None,
            receive_channel: None,
            db_url: "mysql://root:admin@localhost:3306/expro".to_owned(),
            first_frame: true,
            id_input: "".to_owned(),
            input_result: "".to_owned(),
            is_emergency: false,
            value: 2.7,
            employee_buffer: Vec::new(),
            emergency_buffer: Vec::new(),
            id_check: CheckError {
                is_error: false,
                err_msg: "".to_owned(),
            },
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
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(200, 200, 200);
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
        // Request repaint to keep updating the app even when it is not in view.
        ctx.request_repaint();
        let now = chrono::Local::now().naive_local().and_utc().timestamp();

        //
        if self.first_frame {
            self.last_log_dump = now;
            self.first_frame = false;
        }

        // Check if the period is reached
        // We use debounce here to avoid calling the log dump function multiple times within
        // 1 second (60 fps)
        if (now - self.last_log_dump) == 86400 && !self.log_dump_debounced {
            self.log_dump_debounced = true;
            self.last_log_dump = now;

            match dump_24h_log_file(self, now) {
                Ok(_) => {}
                Err(_) => {}
            }
        }

        // Remove debounce after 1 second has passed
        if (now - self.last_log_dump) > 1 {
            self.log_dump_debounced = false;
        }

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
                    ui.menu_button("Edit", |ui| {
                        ui.checkbox(&mut self.locked, "Lock input");
                    });
                    ui.add_space(16.0);
                    ui.menu_button("Help", |ui| {
                        if ui.button("About").clicked() {
                            self.about_show = !self.about_show;
                        }
                    });
                }

                //egui::widgets::global_theme_preference_buttons(ui);
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
                });
            });
        });

        egui::Window::new("About")
            .open(&mut self.about_show)
            .fade_out(true)
            .show(ctx, |ui| {
                ui.label("Developed by Abdelkader Madoui <abdelkader.madoui@expro.com>.");
                ui.label("All rights reserved 2025.");
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            egui::Grid::new("id_grid")
                .num_columns(4)
                .min_col_width(240.0)
                .show(ui, |ui| {
                    ui.heading(format!("{}", self.input_result));
                    if self.id_check.is_error {
                        // ui.heading(format!("Error: {}", self.id_check.err_msg));
                        ui.label(
                            RichText::new(format!("ERROR: {}", self.id_check.err_msg))
                                .size(16.)
                                .background_color(Color32::RED),
                        );
                    } else if self.scanned_employee_name.len() > 6 {
                        ui.label(
                            RichText::new(format!("  {}  ", self.scanned_employee_name))
                                .background_color(Color32::from_rgb(51, 204, 51))
                                .color(Color32::BLACK)
                                .size(20.),
                        );
                    }
                    ui.end_row();
                    ui.label("DB URL:");
                    ui.text_edit_singleline(&mut self.db_url);
                    ui.end_row();
                    ui.label("Type your ID: ");
                    if self.locked {
                        let edit = TextEdit::singleline(&mut self.id_input).lock_focus(true);
                        ui.add(edit).request_focus();
                    } else {
                        let edit = TextEdit::singleline(&mut self.id_input).lock_focus(true);
                        ui.add(edit);
                        ui.end_row();
                    }
                });

            ui.horizontal(|_ui| {
                if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                    if !self.id_input.is_empty() {
                        self.input_result = self.id_input.clone();
                        self.id_input = "".to_owned();
                        //let url = "mysql://root:admin@localhost:3306/employees";
                        match process_id(self) {
                            Ok(_) => {
                                self.id_check.is_error = false;
                                self.id_check.err_msg = "".to_owned();
                            }
                            Err(e) => {
                                self.id_check.is_error = true;
                                self.id_check.err_msg = format!("{}", e);
                            }
                        }
                    }
                }
            });

            ui.separator();

            if self.is_emergency && (self.emergency.missing_list.len() > 0) && self.count_pressed {
                ui.heading("Missing List");
                let available_height = ui.available_height();
                let table = TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::exact(30.0))
                    .column(Column::exact(200.0))
                    .column(Column::exact(200.0))
                    .column(Column::exact(100.0))
                    .column(Column::exact(200.0))
                    .column(Column::exact(80.0))
                    .column(Column::exact(100.0))
                    .column(Column::exact(100.0))
                    .column(Column::remainder())
                    .min_scrolled_height(0.0)
                    .max_scroll_height(available_height);

                table
                    .header(40.0, |mut header| {
                        header.col(|ui| {
                            ui.strong("INDEX");
                        });
                        header.col(|ui| {
                            ui.strong("ID");
                        });
                        header.col(|ui| {
                            ui.strong("NAME");
                        });
                        header.col(|ui| {
                            ui.strong("DEPARTMENT");
                        });
                        header.col(|ui| {
                            ui.strong("TITLE");
                        });
                        header.col(|ui| {
                            ui.strong("EXPRO ID");
                        });
                        header.col(|ui| {
                            ui.strong("FIELD");
                        });
                        header.col(|ui| {
                            ui.strong("STATUS");
                        });
                        header.col(|ui| {
                            ui.strong("TIMESTAMP");
                        });
                    })
                    .body(|body| {
                        let row_height = 20.0;
                        let num_rows = self.emergency.missing_list.len();

                        body.rows(row_height, num_rows, |mut row| {
                            let index = num_rows - 1 - row.index();
                            let employee = &self.emergency.missing_list[index];
                            row.col(|ui| {
                                ui.label(format!("{index}"));
                            });
                            row.col(|ui| {
                                let id = &employee.id;
                                ui.label(format!("{id}"));
                            });
                            row.col(|ui| {
                                let name = &employee.name;
                                ui.label(format!("{name}"));
                            });
                            row.col(|ui| {
                                let department = &employee.department;
                                ui.label(format!("{department}"));
                            });
                            row.col(|ui| {
                                let title = &employee.title;
                                ui.label(format!("{title}"));
                            });
                            row.col(|ui| {
                                let expro_id = &employee.expro_id;
                                ui.label(format!("{expro_id}"));
                            });
                            row.col(|ui| {
                                let field = &employee.field;
                                ui.label(format!("{field}"));
                            });
                            row.col(|ui| {
                                //ui.label("IN");
                                let in_base = &employee.in_base;
                                if *in_base == 1 {
                                    ui.add(
                                        Button::new("  MISSING  ")
                                            .fill(Color32::RED)
                                            .corner_radius(0.0)
                                            .min_size(Vec2::new(100.0, 10.0))
                                            .frame(false),
                                    );
                                } else {
                                    ui.add(
                                        Button::new("  OUT  ")
                                            .fill(Color32::RED)
                                            .corner_radius(0.0)
                                            .min_size(Vec2::new(100.0, 10.0))
                                            .frame(false),
                                    );
                                }
                            });
                            row.col(|ui| {
                                let timestamp = &employee.last_timestamp;
                                let time_str = DateTime::from_timestamp(*timestamp as i64, 0)
                                    .unwrap()
                                    .format("%d-%m-%y %H:%M:%S");
                                ui.label(format!("{time_str}"));
                            });
                        })
                    });
            } else if self.is_emergency {
                ui.heading("Employee Count List");
                let available_height = ui.available_height();
                let table = TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::exact(30.0))
                    .column(Column::exact(200.0))
                    .column(Column::exact(200.0))
                    .column(Column::exact(100.0))
                    .column(Column::exact(200.0))
                    .column(Column::exact(80.0))
                    .column(Column::exact(100.0))
                    .column(Column::exact(100.0))
                    .column(Column::remainder())
                    .min_scrolled_height(0.0)
                    .max_scroll_height(available_height);

                table
                    .header(40.0, |mut header| {
                        header.col(|ui| {
                            ui.strong("INDEX");
                        });
                        header.col(|ui| {
                            ui.strong("ID");
                        });
                        header.col(|ui| {
                            ui.strong("NAME");
                        });
                        header.col(|ui| {
                            ui.strong("DEPARTMENT");
                        });
                        header.col(|ui| {
                            ui.strong("TITLE");
                        });
                        header.col(|ui| {
                            ui.strong("EXPRO ID");
                        });
                        header.col(|ui| {
                            ui.strong("FIELD");
                        });
                        header.col(|ui| {
                            ui.strong("STATUS");
                        });
                        header.col(|ui| {
                            ui.strong("TIMESTAMP");
                        });
                    })
                    .body(|body| {
                        let row_height = 20.0;
                        let num_rows = self.emergency.present_employees_hash.len();
                        let present_employees_vec: Vec<&Employee> =
                            self.emergency.present_employees_hash.iter().collect();

                        body.rows(row_height, num_rows, |mut row| {
                            let index = num_rows - 1 - row.index();
                            let employee = present_employees_vec[index];
                            row.col(|ui| {
                                ui.label(format!("{index}"));
                            });
                            row.col(|ui| {
                                let id = &employee.id;
                                ui.label(format!("{id}"));
                            });
                            row.col(|ui| {
                                let name = &employee.name;
                                ui.label(format!("{name}"));
                            });
                            row.col(|ui| {
                                let department = &employee.department;
                                ui.label(format!("{department}"));
                            });
                            row.col(|ui| {
                                let title = &employee.title;
                                ui.label(format!("{title}"));
                            });
                            row.col(|ui| {
                                let expro_id = &employee.expro_id;
                                ui.label(format!("{expro_id}"));
                            });
                            row.col(|ui| {
                                let field = &employee.field;
                                ui.label(format!("{field}"));
                            });
                            row.col(|ui| {
                                //ui.label("IN");
                                ui.add(
                                    Button::new("  PRESENT  ")
                                        .fill(Color32::from_rgb(51, 204, 51))
                                        .corner_radius(0.0)
                                        .min_size(Vec2::new(100.0, 10.0))
                                        .frame(false),
                                );
                            });
                            row.col(|ui| {
                                let timestamp = &employee.last_timestamp;
                                let time_str = DateTime::from_timestamp(*timestamp as i64, 0)
                                    .unwrap()
                                    .format("%d-%m-%y %H:%M:%S");
                                ui.label(format!("{time_str}"));
                            });
                        })
                    });
            } else {
                let available_height = ui.available_height();
                let table = TableBuilder::new(ui)
                    .striped(true)
                    .stick_to_bottom(true)
                    //.scroll_to_row(self.employee_buffer.len(), Some(egui::Align::BOTTOM))
                    .resizable(false)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::exact(30.0))
                    .column(Column::exact(200.0))
                    .column(Column::exact(200.0))
                    .column(Column::exact(100.0))
                    .column(Column::exact(200.0))
                    .column(Column::exact(80.0))
                    .column(Column::exact(100.0))
                    .column(Column::exact(100.0))
                    .column(Column::remainder())
                    .min_scrolled_height(0.0)
                    .max_scroll_height(available_height);

                table
                    .header(40.0, |mut header| {
                        header.col(|ui| {
                            ui.strong("INDEX");
                        });
                        header.col(|ui| {
                            ui.strong("ID");
                        });
                        header.col(|ui| {
                            ui.strong("NAME");
                        });
                        header.col(|ui| {
                            ui.strong("DEPARTMENT");
                        });
                        header.col(|ui| {
                            ui.strong("TITLE");
                        });
                        header.col(|ui| {
                            ui.strong("EXPRO ID");
                        });
                        header.col(|ui| {
                            ui.strong("FIELD");
                        });
                        header.col(|ui| {
                            ui.strong("STATUS");
                        });
                        header.col(|ui| {
                            ui.strong("TIMESTAMP");
                        });
                    })
                    .body(|body| {
                        let row_height = 20.0;
                        let num_rows = self.employee_buffer.len();

                        body.rows(row_height, num_rows, |mut row| {
                            let index = num_rows - 1 - row.index();

                            let employee = &self.employee_buffer[index];
                            row.col(|ui| {
                                ui.label(format!("{index}"));
                            });
                            row.col(|ui| {
                                let id = &employee.id;
                                ui.label(format!("{id}"));
                            });
                            row.col(|ui| {
                                let name = &employee.name;
                                ui.label(format!("{name}"));
                            });
                            row.col(|ui| {
                                let department = &employee.department;
                                ui.label(format!("{department}"));
                            });
                            row.col(|ui| {
                                let title = &employee.title;
                                ui.label(format!("{title}"));
                            });
                            row.col(|ui| {
                                let expro_id = &employee.expro_id;
                                ui.label(format!("{expro_id}"));
                            });
                            row.col(|ui| {
                                let field = &employee.field;
                                ui.label(format!("{field}"));
                            });
                            row.col(|ui| {
                                //ui.label("IN");
                                let in_base = &employee.in_base;
                                if *in_base == 1 {
                                    ui.add(
                                        Button::new("  IN  ")
                                            .fill(Color32::from_rgb(51, 204, 51))
                                            .corner_radius(0.0)
                                            .min_size(Vec2::new(100.0, 10.0))
                                            .frame(false),
                                    );
                                } else {
                                    ui.add(
                                        Button::new("  OUT  ")
                                            .fill(Color32::from_rgb(242, 13, 13))
                                            .corner_radius(0.0)
                                            .min_size(Vec2::new(100.0, 10.0))
                                            .frame(false),
                                    );
                                }
                            });
                            row.col(|ui| {
                                let timestamp = &employee.last_timestamp;
                                let time_str = DateTime::from_timestamp(*timestamp as i64, 0)
                                    .unwrap()
                                    .format("%d-%m-%y %H:%M:%S");
                                ui.label(format!("{time_str}"));
                            });
                        })
                    });
            }
        });
        egui::SidePanel::right("right")
            .min_width(200.0)
            .max_width(200.0)
            .default_width(200.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.add(egui::Image::new(egui::include_image!("../assets/logo.png")));
                ui.vertical_centered(|ui| {
                    if ui
                        .add(Button::new("EMERGENCY").min_size(Vec2::new(184., 40.)))
                        .clicked()
                    {
                        self.is_emergency = true;

                        match emergency_get_employee_list(self) {
                            Ok(_) => {
                                self.id_check.is_error = false;
                                self.id_check.err_msg = "".to_owned();
                            }
                            Err(e) => {
                                self.id_check.is_error = true;
                                self.id_check.err_msg = format!("{}", e);
                            }
                        }
                    }

                    if self.is_emergency {
                        ui.heading(format!("ON BASE TOTAL:"));
                        ui.add(Label::new(
                            RichText::new(format!(
                                "   {}   ",
                                self.emergency.all_employees_hash.len()
                            ))
                            .size(24.)
                            .strong()
                            .background_color(Color32::BLACK)
                            .color(Color32::WHITE),
                        ));
                        ui.heading(format!("CURRENT COUNT:"));
                        ui.add(Label::new(
                            RichText::new(format!(
                                "   {}   ",
                                self.emergency.present_employees_hash.len()
                            ))
                            .size(24.)
                            .strong()
                            .background_color(Color32::BLACK)
                            .color(Color32::WHITE),
                        ));
                        ui.heading(format!("MISSING:"));
                        ui.add(Label::new(
                            RichText::new(format!("   {}   ", self.emergency.missing_list.len()))
                                .size(24.)
                                .strong()
                                .background_color(Color32::BLACK)
                                .color(Color32::WHITE),
                        ));
                        if ui
                            .add(
                                Button::new("COUNT")
                                    .min_size(Vec2::new(184., 40.))
                                    .fill(Color32::from_rgb(51, 204, 51)),
                            )
                            .clicked()
                        {
                            self.count_pressed = true;
                            let diff: Vec<_> = self
                                .emergency
                                .all_employees_hash
                                .difference(&self.emergency.present_employees_hash)
                                .map(|employee| employee.clone())
                                .collect();
                            self.emergency.missing_list = diff;
                        }
                        if ui
                            .add(
                                Button::new("RESET")
                                    .min_size(Vec2::new(184., 40.))
                                    .fill(Color32::from_rgb(255, 204, 0)),
                            )
                            .clicked()
                        {
                            self.reset_pressed = true;
                        }
                        if self.reset_pressed {
                            if ui
                                .add(
                                    Button::new("CONFIRM RESET")
                                        .min_size(Vec2::new(184., 40.))
                                        .fill(Color32::from_rgb(242, 13, 13)),
                                )
                                .clicked()
                            {
                                self.emergency.on_base_total = 0;
                                //self.emergency.on_base_list = Vec::new();
                                //self.emergency.count_list = Vec::new();
                                self.emergency.all_employees_hash.clear();
                                self.emergency.present_employees_hash.clear();
                                self.emergency.missing_list.clear();
                                self.reset_pressed = false;
                                self.count_pressed = false;
                                self.is_emergency = false;
                            }
                        }
                        if self.count_pressed {
                            if ui
                                .add(Button::new("GENERATE REPORT").min_size(Vec2::new(184., 40.)))
                                .clicked()
                            {
                                // TODO
                                match generate_report(self) {
                                    Err(e) => {
                                        self.id_check.is_error = true;
                                        self.id_check.err_msg = format!("{}", e)
                                    }
                                    Ok(_) => {}
                                }
                            }
                        }
                    }
                });
            });
    }
}
fn generate_report(app: &mut TemplateApp) -> Result<()> {
    let mut template_str = r#"
 
                            #set page(paper: "a4", margin: (
                              top: 3cm,
                                bottom: 3cm,
                                  left: 2cm, 
                                right: 2cm,
                                              x: 1cm,
                                                  ), header: context {
                                                        [

                                                                _Expro Emergency Access Report_
                                                                    #h(1fr)
                                                                        #counter(page).display()
                                                                          ]
                                                                          }, )


                                                                          #set text(font: "Arial", size: 6pt)

                                                                          // Medium bold table header.
                                                                          #show table.cell.where(y: 0): set text(weight: "medium")

                                                                          // Bold titles.

                                                                          // See the strokes section for details on this!
                                                                          #let frame(stroke) = (x, y) => (
                                                                                left: if x > 0 { 0pt } else { stroke },
                                                                                  right: stroke,
                                                                                    top: if y < 2 { stroke } else { 0pt },
                                                                                      bottom: stroke,
                                                                                      )

                                                                                      #set table(
                                                                                            fill: (_, y) => if calc.odd(y) { rgb("EAF2F5") },
                                                                                              stroke: frame(rgb("21222C")),
                                                                                              )

                                                                                              #table(
                                                                                                    columns: (1fr, 1fr, 1fr, 1fr, 0.5fr),

                                                                                                      table.header[ID][Name][Department][Function][Status],
                                "#.to_owned();
    for missing in app.emergency.missing_list.clone() {
        let typst_string = format!(
            "\n[{}], [{}], [{}], [{}], [MISSING],",
            missing.id, missing.name, missing.department, missing.title
        );
        template_str.push_str(&typst_string);
    }
    template_str.push_str(
        r#"
          )
        "#,
    );

    let mut file = std::fs::File::options()
        .write(true)
        .truncate(true)
        .open("template.typ")?;
    file.write_all(template_str.as_bytes())?;

    let _compile_cmd = std::process::Command::new("cmd")
        .args(["/C", "typst compile template.typ"])
        .output()?;
    let _pdf_open_cmd = std::process::Command::new("cmd")
        .args(["/C", "start template.pdf"])
        .output()?;

    Ok(())
}
// Query a list of the employees who are currently inside the base.
fn emergency_get_employee_list(app: &mut TemplateApp) -> Result<()> {
    let pool = Pool::new(app.db_url.as_str())?;
    let mut conn = pool.get_conn()?;

    let res = conn.query_map(
            format!(
                r#"
                    SELECT id, name, department, title, expro_id, field, category, in_base, last_timestamp FROM expro_employees
                    WHERE in_base={};
                "#,
                    1
            ),
            |(id, name, department, title, expro_id, field, category, in_base, last_timestamp)| {
                Employee {
                    id,
                    name,
                    department,
                    title,
                    expro_id,
                    field,
                    category,
                    in_base,
                    last_timestamp
                }
            }
    )?;
    app.emergency.on_base_total = res.len();
    //app.emergency.on_base_list = res.clone();

    let hash: HashSet<Employee> = HashSet::from_iter(res);
    app.emergency.all_employees_hash = hash;

    Ok(())
}

// Process the employee ID and update the employee status (IN or OUT)
// Errors are bubbled up and dealt with by the caller.
fn process_id(app: &mut TemplateApp) -> Result<()> {
    let pool = Pool::new(app.db_url.as_str())?;
    let mut conn = pool.get_conn()?;

    let res = conn.query_map(
            format!(
            r#"
                SELECT id, name, department, title, expro_id, field, category, in_base, last_timestamp FROM expro_employees
                WHERE id={};
            "#,
            app.input_result
            ),
            |(id, name, department, title, expro_id, field, category, in_base, last_timestamp)| {
                Employee {
                    id,
                    name,
                    department,
                    title,
                    expro_id,
                    field,
                    category,
                    in_base,
                    last_timestamp
                }
            }
    )?;

    if res.len() == 1 {
        let timestamp = chrono::Local::now().naive_local().and_utc().timestamp();
        let mut employee_query_result = res[0].clone();

        app.scanned_employee_name = employee_query_result.name.clone();
        // During emergencies we no longer update the employee status
        // but we push the employee id into the emergency hash for counting.
        if app.is_emergency {
            if employee_query_result.in_base == 0 {
                return Err(anyhow!("The employee is not inside the base."));
            }
            // We check if the employee has already been counted in the drill.
            let mut exists = false;
            for employee in app.emergency.present_employees_hash.iter() {
                if employee.id == employee_query_result.id {
                    exists = true;
                }
            }
            if !exists {
                // If the employee has not been counted we push them to the present list.
                app.emergency
                    .present_employees_hash
                    .insert(employee_query_result.clone());
            } else {
                return Err(anyhow!("The employee has already been counted."));
            }
        } else {
            let duration_since = timestamp - employee_query_result.last_timestamp as i64;

            if duration_since >= 30 {
                if employee_query_result.in_base == 0 {
                    let _update_res = conn.exec_drop(
                        format!(
                            "UPDATE expro_employees
                            SET in_base=1, last_timestamp={}
                            WHERE id={}",
                            timestamp, employee_query_result.id
                        ),
                        (),
                    )?;

                    employee_query_result.in_base = 1;
                    employee_query_result.last_timestamp = timestamp as usize;
                    app.employee_buffer.push(employee_query_result);
                } else {
                    let _update_res = conn.exec_drop(
                        format!(
                            "UPDATE expro_employees
                            SET in_base=0, last_timestamp={}
                            WHERE id={}",
                            timestamp, employee_query_result.id
                        ),
                        (),
                    )?;

                    employee_query_result.in_base = 0;
                    employee_query_result.last_timestamp = timestamp as usize;
                    app.employee_buffer.push(employee_query_result);
                }
            }
        }
    } else if res.len() == 0 {
        return Err(anyhow!("Could not find ID in the database."));
    } else {
        return Err(anyhow!("More than one ID found in the database."));
    }
    Ok(())
}

fn dump_24h_log_file(app: &mut TemplateApp, timestamp: i64) -> Result<()> {
    if let Some(date_time) = DateTime::from_timestamp(timestamp, 0) {
        let mut buffer = String::new();
        let date = date_time.naive_local().and_utc().format("%d%m%Y_%H%M");
        let file_name = format!("LOG_{}.txt", date);

        for record in app.employee_buffer.iter() {
            let mut line = String::new();
            let time = DateTime::from_timestamp(record.last_timestamp as i64, 0)
                .unwrap()
                .format("%d-%m-%Y\t%H:%M:%S");
            if record.in_base == 0 {
                line = format!(
                    "{}\t{}\t{}\t{}\t{}\n",
                    &record.name, &record.department, &record.title, "OUT", &time
                );
            } else {
                line = format!(
                    "{}\t{}\t{}\t{}\t{}\n",
                    &record.name, &record.department, &record.title, "IN", &time
                );
            }

            buffer.push_str(line.as_str());
        }

        let mut file = std::fs::File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_name)?;
        file.write_all(buffer.as_bytes())?;
        app.employee_buffer.clear();
    }
    Ok(())
}
