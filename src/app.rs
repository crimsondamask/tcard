//use rusqlite::Connection;
use anyhow::{anyhow, Result};
use rodio::*;
//use anyhow::Result;
use crossbeam_channel::{Receiver, Sender};
use mysql::prelude::*;
use mysql::*;
//use sqlx::mysql::MySqlPool;
use std::{collections::HashSet, fmt::Display, io::Write, sync::Arc, time::Duration};

use chrono::DateTime;
use egui::{
    style::Selection, Button, Color32, CornerRadius, Label, RichText, Stroke, TextEdit, Vec2,
    Visuals,
};
use egui_extras::{Column, TableBuilder};

#[derive(PartialEq, Eq, serde::Deserialize, serde::Serialize)]
enum Base {
    MainBase,
    IkramBase,
}

impl Display for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Base::MainBase => {
                write!(f, "Main Base")
            }
            &Base::IkramBase => {
                write!(f, "Ikram Base")
            }
        }
    }
}

struct DepartmentCount {
    breakfast: usize,
    lunch: usize,
    dinner: usize,
}
struct ScannedDetails {
    employee_name: String,
    status: String,
}
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
    last_timestamp: usize,
    breakfast: usize,
    lunch: usize,
    dinner: usize,
}
#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
enum Meal {
    Breakfast,
    Lunch,
    Dinner,
}

impl Display for Meal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Meal::Breakfast => write!(f, "Breakfast"),
            Meal::Lunch => write!(f, "Lunch"),
            Meal::Dinner => write!(f, "Dinner"),
        }
    }
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
    option_menu_open: bool,
    meal: Meal,
    #[serde(skip)] // This how you opt-out of serialization of a field
    scroll_up: bool,
    #[serde(skip)] // This how you opt-out of serialization of a field
    last_log_dump: i64,
    current_base: Base,
    #[serde(skip)] // This how you opt-out of serialization of a field
    log_dump_debounced: bool,
    #[serde(skip)] // This how you opt-out of serialization of a field
    scanned_employee: ScannedDetails,
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
            meal: Meal::Breakfast,
            option_menu_open: false,
            scroll_up: false,
            last_log_dump: 0,
            current_base: Base::MainBase,
            log_dump_debounced: false,
            // Example stuff:
            scanned_employee: ScannedDetails {
                employee_name: "".to_string(),
                status: "".to_string(),
            },
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
                        if ui.button("Options").clicked() {
                            self.option_menu_open = !self.option_menu_open;
                        }
                    });
                    ui.add_space(16.0);
                    ui.menu_button("Help", |ui| {
                        if ui.button("About").clicked() {
                            self.about_show = !self.about_show;
                        }
                    });
                    ui.with_layout(
                        egui::Layout::right_to_left(egui::Align::RIGHT),
                        |ui| match self.meal {
                            Meal::Breakfast => {
                                ui.strong("Breakfast");
                            }
                            Meal::Lunch => {
                                ui.strong("Lunch");
                            }
                            Meal::Dinner => {
                                ui.strong("Dinner");
                            }
                        },
                    );
                }
            });
        });

        egui::Window::new("About")
            .open(&mut self.about_show)
            .fade_out(true)
            .show(ctx, |ui| {
                ui.label("Developed by Abdelkader Madoui <abdelkader.madoui@expro.com>.");
                ui.label("All rights reserved 2025.");
            });

        egui::Window::new("Options")
            .open(&mut self.option_menu_open)
            .fade_out(true)
            .show(ctx, |ui| {
                egui::Grid::new("meal_selection")
                    .num_columns(2)
                    .striped(true)
                    .min_col_width(200.)
                    .show(ui, |ui| {
                        ui.label("Select meal:");
                        egui::ComboBox::from_label("Meal")
                            .selected_text(format!("{}", self.meal))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.meal, Meal::Breakfast, "Breakfast");
                                ui.selectable_value(&mut self.meal, Meal::Lunch, "Lunch");
                                ui.selectable_value(&mut self.meal, Meal::Dinner, "Dinner");
                            });
                        ui.end_row();
                        if ui.button("Start").clicked() {
                            self.employee_buffer.clear();
                        }
                        ui.end_row();
                        if ui.button("Generate Day Report").clicked() {
                            match generate_day_report(self.db_url.clone()) {
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
                    });
                ui.collapsing("Reset", |ui| {
                    if ui
                        .add(egui::Button::new("RESET DAY COUNT").fill(Color32::RED))
                        .clicked()
                    {
                        match generate_day_report(self.db_url.clone()) {
                            Ok(_) => {
                                self.id_check.is_error = false;
                                self.id_check.err_msg = "".to_owned();
                                match reset_day(self.db_url.clone()) {
                                    Ok(_) => {
                                        self.id_check.is_error = false;
                                        self.id_check.err_msg = "".to_owned();
                                        self.employee_buffer.clear();
                                    }
                                    Err(e) => {
                                        self.id_check.is_error = true;
                                        self.id_check.err_msg = format!("{}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                self.id_check.is_error = true;
                                self.id_check.err_msg = format!("{}", e);
                            }
                        }
                    }
                });
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
                    } else if self.scanned_employee.employee_name.len() > 6 {
                        ui.label(
                            RichText::new(format!("  {}  ", self.scanned_employee.employee_name))
                                .background_color(Color32::from_rgb(51, 204, 51))
                                .color(Color32::BLACK)
                                .size(20.),
                        );
                    }

                    // if self.scanned_employee.status == "  IN  ".to_string() {
                    //     ui.label(
                    //         RichText::new(format!("{}", self.scanned_employee.status))
                    //             .background_color(Color32::from_rgb(51, 204, 51))
                    //             .color(Color32::BLACK)
                    //             .size(20.),
                    //     );
                    // } else {
                    //     ui.label(
                    //         RichText::new(format!("{}", self.scanned_employee.status))
                    //             .background_color(Color32::RED)
                    //             .color(Color32::BLACK)
                    //             .size(20.),
                    //     );
                    // }
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
                let mut table = TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                    .vscroll(true)
                    .min_scrolled_height(0.0)
                    .max_scroll_height(available_height);
                if self.scroll_up {
                    table = table.scroll_to_row(1, Some(egui::Align::Center));
                    self.scroll_up = false;
                }

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
                                ui.add(
                                    Button::new("  MISSING  ")
                                        .fill(Color32::RED)
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
            } else if self.is_emergency {
                ui.heading("Employee Count List");
                let available_height = ui.available_height();
                let mut table = TableBuilder::new(ui)
                    .striped(true)
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .resizable(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .vscroll(true)
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                    .min_scrolled_height(0.0)
                    .max_scroll_height(available_height);

                if self.scroll_up {
                    table = table.scroll_to_row(1, Some(egui::Align::Center));
                    self.scroll_up = false;
                }
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
                let mut table = TableBuilder::new(ui)
                    .striped(true)
                    //.stick_to_bottom(true)
                    //.scroll_to_row(self.employee_buffer.len(), Some(egui::Align::BOTTOM))
                    .resizable(true)
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .vscroll(true)
                    .min_scrolled_height(0.0)
                    .max_scroll_height(available_height);
                if self.scroll_up {
                    table = table.scroll_to_row(1, Some(egui::Align::Center));
                    self.scroll_up = false;
                }

                table
                    .header(40.0, |mut header| {
                        header.col(|ui| {
                            if ui.small_button("UP").clicked() {
                                self.scroll_up = true;
                            }
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
                        // header.col(|ui| {
                        //     ui.strong("STATUS");
                        // });
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
                ui.vertical_centered(|ui| {});
            });
    }
}
fn generate_report(app: &mut TemplateApp) -> Result<()> {
    let datetime = chrono::Local::now();
    let date_str = datetime.naive_local().and_utc().format("%d-%m-%Y %H:%M");
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


                                                                          #set text(font: "Arial", size: 8pt)

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
            missing.id,
            missing
                .name
                .replace("'", "")
                .replace("#", "")
                .replace("%", "")
                .replace("$", "")
                .replace("\\", ""),
            missing
                .department
                .replace("'", "")
                .replace("#", "")
                .replace("%", "")
                .replace("$", "")
                .replace("\\", ""),
            missing
                .title
                .replace("'", "")
                .replace("#", "")
                .replace("%", "")
                .replace("$", "")
                .replace("\\", ""),
        );
        template_str.push_str(&typst_string);
    }
    template_str.push_str(
        r#"
          )
        "#,
    );

    template_str.push_str(format!("Report date: {}", date_str).as_str());

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

fn generate_day_report(db_url: String) -> Result<()> {
    let datetime = chrono::Local::now();
    let date_str = datetime.naive_local().and_utc().format("%d-%m-%Y %H:%M");
    let mut template_str = r#"
 
                            #set page(paper: "a4", margin: (
                              top: 3cm,
                                bottom: 3cm,
                                  left: 2cm, 
                                right: 2cm,
                                              x: 1cm,
                                                  ), header: context {
                                                        [

                                                                _Expro Canteen Report_
                                                                    #h(1fr)
                                                                        #counter(page).display()
                                                                          ]
                                                                          }, )


                                                                          #set text(font: "Arial", size: 8pt)

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
                                                                                                    columns: (1fr, 1fr, 1fr, 0.5fr, 0.5fr, 0.5fr),

                                                                                                      table.header[ID][Name][Department][Breakfast][Lunch][Dinner],
                                "#.to_owned();
    let pool = Pool::new(db_url.as_str())?;
    let mut conn = pool.get_conn()?;

    let res = conn.query_map(
            format!(
            r#"
                SELECT id, name, department, title, expro_id, field, category, breakfast, lunch, dinner, last_timestamp FROM expro_employees
                WHERE breakfast=1 OR lunch=1 OR dinner=1;
            "#,
            ),
            |(id, name, department, title, expro_id, field, category, breakfast, lunch, dinner, last_timestamp)| {
                Employee {
                    id,
                    name,
                    department,
                    title,
                    expro_id,
                    field,
                    category,
                    breakfast,
                    lunch,
                    dinner,
                    last_timestamp
                }
            }
    )?;

    let mut csv_string = String::from("ID,NAME,DEPARTMENT,BREAKFAST,LUNCH,DINNER\n");
    let mut breakfast_total = 0;
    let mut lunch_total = 0;
    let mut dinner_total = 0;

    let mut production_count = DepartmentCount {
        breakfast: 0,
        lunch: 0,
        dinner: 0,
    };
    let mut daq_count = DepartmentCount {
        breakfast: 0,
        lunch: 0,
        dinner: 0,
    };
    let mut welltest_count = DepartmentCount {
        breakfast: 0,
        lunch: 0,
        dinner: 0,
    };
    let mut meters_count = DepartmentCount {
        breakfast: 0,
        lunch: 0,
        dinner: 0,
    };
    let mut mpp_count = DepartmentCount {
        breakfast: 0,
        lunch: 0,
        dinner: 0,
    };
    let mut wireline_count = DepartmentCount {
        breakfast: 0,
        lunch: 0,
        dinner: 0,
    };
    let mut dst_count = DepartmentCount {
        breakfast: 0,
        lunch: 0,
        dinner: 0,
    };
    let mut lab_count = DepartmentCount {
        breakfast: 0,
        lunch: 0,
        dinner: 0,
    };
    let mut it_count = DepartmentCount {
        breakfast: 0,
        lunch: 0,
        dinner: 0,
    };
    let mut finance_count = DepartmentCount {
        breakfast: 0,
        lunch: 0,
        dinner: 0,
    };
    let mut admin_count = DepartmentCount {
        breakfast: 0,
        lunch: 0,
        dinner: 0,
    };
    let mut bd_count = DepartmentCount {
        breakfast: 0,
        lunch: 0,
        dinner: 0,
    };
    let mut facilities_count = DepartmentCount {
        breakfast: 0,
        lunch: 0,
        dinner: 0,
    };
    let mut hr_count = DepartmentCount {
        breakfast: 0,
        lunch: 0,
        dinner: 0,
    };
    let mut hseq_count = DepartmentCount {
        breakfast: 0,
        lunch: 0,
        dinner: 0,
    };
    let mut learning_and_dev = DepartmentCount {
        breakfast: 0,
        lunch: 0,
        dinner: 0,
    };
    let mut ops_count = DepartmentCount {
        breakfast: 0,
        lunch: 0,
        dinner: 0,
    };
    let mut supply_chain_count = DepartmentCount {
        breakfast: 0,
        lunch: 0,
        dinner: 0,
    };
    let mut trs_count = DepartmentCount {
        breakfast: 0,
        lunch: 0,
        dinner: 0,
    };

    for entry in res {
        if entry.breakfast == 1 {
            breakfast_total += 1;
            match entry.department.to_lowercase().trim() {
                "area support" => {
                    admin_count.breakfast += 1;
                }
                "bd" => {
                    bd_count.breakfast += 1;
                }
                "daq" => {
                    daq_count.breakfast += 1;
                }
                "dst" => {
                    dst_count.breakfast += 1;
                }
                "facilities" => {
                    facilities_count.breakfast += 1;
                }
                "finance" => {
                    finance_count.breakfast += 1;
                }
                "fluids" => {
                    lab_count.breakfast += 1;
                }
                "hr" => {
                    hr_count.breakfast += 1;
                }
                "hseq" => {
                    hseq_count.breakfast += 1;
                }
                "qhse" => {
                    hseq_count.breakfast += 1;
                }
                "it" => {
                    it_count.breakfast += 1;
                }
                "l&d" => {
                    learning_and_dev.breakfast += 1;
                }
                "logging" => {
                    wireline_count.breakfast += 1;
                }
                "meters" => {
                    meters_count.breakfast += 1;
                }
                "production" => {
                    production_count.breakfast += 1;
                }
                "sampling" => {
                    daq_count.breakfast += 1;
                }
                "production operations" => {
                    ops_count.breakfast += 1;
                }
                "supply chain" => {
                    supply_chain_count.breakfast += 1;
                }
                "trs / wc" => {
                    trs_count.breakfast += 1;
                }
                "welltest" => {
                    welltest_count.breakfast += 1;
                }
                "wireline" => {
                    wireline_count.breakfast += 1;
                }
                _ => {}
            }
        }
        if entry.lunch == 1 {
            lunch_total += 1;

            match entry.department.to_lowercase().trim() {
                "area support" => {
                    admin_count.lunch += 1;
                }
                "bd" => {
                    bd_count.lunch += 1;
                }
                "daq" => {
                    daq_count.lunch += 1;
                }
                "dst" => {
                    dst_count.lunch += 1;
                }
                "facilities" => {
                    facilities_count.lunch += 1;
                }
                "finance" => {
                    finance_count.lunch += 1;
                }
                "fluids" => {
                    lab_count.lunch += 1;
                }
                "hr" => {
                    hr_count.lunch += 1;
                }
                "hseq" => {
                    hseq_count.lunch += 1;
                }
                "qhse" => {
                    hseq_count.lunch += 1;
                }
                "it" => {
                    it_count.lunch += 1;
                }
                "l&d" => {
                    learning_and_dev.lunch += 1;
                }
                "logging" => {
                    wireline_count.lunch += 1;
                }
                "meters" => {
                    meters_count.lunch += 1;
                }
                "production" => {
                    production_count.lunch += 1;
                }
                "sampling" => {
                    daq_count.lunch += 1;
                }
                "production operations" => {
                    ops_count.lunch += 1;
                }
                "supply chain" => {
                    supply_chain_count.lunch += 1;
                }
                "trs / wc" => {
                    trs_count.lunch += 1;
                }
                "welltest" => {
                    welltest_count.lunch += 1;
                }
                "wireline" => {
                    wireline_count.lunch += 1;
                }
                _ => {}
            }
        }
        if entry.dinner == 1 {
            dinner_total += 1;

            match entry.department.to_lowercase().trim() {
                "area support" => {
                    admin_count.dinner += 1;
                }
                "bd" => {
                    bd_count.dinner += 1;
                }
                "daq" => {
                    daq_count.dinner += 1;
                }
                "dst" => {
                    dst_count.dinner += 1;
                }
                "facilities" => {
                    facilities_count.dinner += 1;
                }
                "finance" => {
                    finance_count.dinner += 1;
                }
                "fluids" => {
                    lab_count.dinner += 1;
                }
                "hr" => {
                    hr_count.dinner += 1;
                }
                "hseq" => {
                    hseq_count.dinner += 1;
                }
                "qhse" => {
                    hseq_count.dinner += 1;
                }
                "it" => {
                    it_count.dinner += 1;
                }
                "l&d" => {
                    learning_and_dev.dinner += 1;
                }
                "logging" => {
                    wireline_count.dinner += 1;
                }
                "meters" => {
                    meters_count.dinner += 1;
                }
                "production" => {
                    production_count.dinner += 1;
                }
                "sampling" => {
                    daq_count.dinner += 1;
                }
                "production operations" => {
                    ops_count.dinner += 1;
                }
                "supply chain" => {
                    supply_chain_count.dinner += 1;
                }
                "trs / wc" => {
                    trs_count.dinner += 1;
                }
                "welltest" => {
                    welltest_count.dinner += 1;
                }
                "wireline" => {
                    wireline_count.dinner += 1;
                }
                _ => {}
            }
        }
        let typst_string = format!(
            "\n[{}], [{}], [{}], [{}], [{}], [{}],",
            entry.id,
            entry
                .name
                .replace("'", "")
                .replace("#", "")
                .replace("%", "")
                .replace("$", "")
                .replace("\\", ""),
            entry
                .department
                .replace("'", "")
                .replace("#", "")
                .replace("%", "")
                .replace("$", "")
                .replace("\\", ""),
            entry.breakfast,
            entry.lunch,
            entry.dinner,
        );
        template_str.push_str(&typst_string);

        let csv_str = format!(
            "{},{},{},{},{},{},\n",
            &entry.id,
            &entry.name,
            &entry.department,
            &entry.breakfast,
            &entry.lunch,
            &entry.dinner
        );
        csv_string.push_str(&csv_str);
    }
    template_str.push_str(
        r#"
          )
        "#,
    );

    template_str.push_str(
        r#"
            
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
                                    columns: (1fr, 1fr, 1fr, 1fr),

                                      table.header[Department][Breakfast][Lunch][Dinner],
        "#,
    );

    template_str.push_str(
        format!(
            "[Area Support],[{}],[{}],[{}],\n",
            admin_count.breakfast, admin_count.lunch, admin_count.dinner
        )
        .as_str(),
    );
    template_str.push_str(
        format!(
            "[BD],[{}],[{}],[{}],\n",
            bd_count.breakfast, bd_count.lunch, bd_count.dinner
        )
        .as_str(),
    );
    template_str.push_str(
        format!(
            "[DAQ],[{}],[{}],[{}],\n",
            daq_count.breakfast, daq_count.lunch, daq_count.dinner
        )
        .as_str(),
    );
    template_str.push_str(
        format!(
            "[DST],[{}],[{}],[{}],\n",
            dst_count.breakfast, dst_count.lunch, dst_count.dinner
        )
        .as_str(),
    );
    template_str.push_str(
        format!(
            "[Facilities],[{}],[{}],[{}],\n",
            facilities_count.breakfast, facilities_count.lunch, facilities_count.dinner
        )
        .as_str(),
    );
    template_str.push_str(
        format!(
            "[Finance],[{}],[{}],[{}],\n",
            finance_count.breakfast, finance_count.lunch, finance_count.dinner
        )
        .as_str(),
    );
    template_str.push_str(
        format!(
            "[Fluids],[{}],[{}],[{}],\n",
            lab_count.breakfast, lab_count.lunch, lab_count.dinner
        )
        .as_str(),
    );
    template_str.push_str(
        format!(
            "[HR],[{}],[{}],[{}],\n",
            hr_count.breakfast, hr_count.lunch, hr_count.dinner
        )
        .as_str(),
    );
    template_str.push_str(
        format!(
            "[QHSE],[{}],[{}],[{}],\n",
            hseq_count.breakfast, hseq_count.lunch, hseq_count.dinner
        )
        .as_str(),
    );
    template_str.push_str(
        format!(
            "[IT],[{}],[{}],[{}],\n",
            it_count.breakfast, it_count.lunch, it_count.dinner
        )
        .as_str(),
    );
    template_str.push_str(
        format!(
            "[L&D],[{}],[{}],[{}],\n",
            learning_and_dev.breakfast, learning_and_dev.lunch, learning_and_dev.dinner
        )
        .as_str(),
    );
    template_str.push_str(
        format!(
            "[Wireline],[{}],[{}],[{}],\n",
            wireline_count.breakfast, wireline_count.lunch, wireline_count.dinner
        )
        .as_str(),
    );
    template_str.push_str(
        format!(
            "[Meters],[{}],[{}],[{}],\n",
            meters_count.breakfast, meters_count.lunch, meters_count.dinner
        )
        .as_str(),
    );
    template_str.push_str(
        format!(
            "[Production],[{}],[{}],[{}],\n",
            meters_count.breakfast, meters_count.lunch, meters_count.dinner
        )
        .as_str(),
    );
    template_str.push_str(
        format!(
            "[OPS],[{}],[{}],[{}],\n",
            ops_count.breakfast, ops_count.lunch, ops_count.dinner
        )
        .as_str(),
    );
    template_str.push_str(
        format!(
            "[Supply Chain],[{}],[{}],[{}],\n",
            supply_chain_count.breakfast, supply_chain_count.lunch, supply_chain_count.dinner
        )
        .as_str(),
    );
    template_str.push_str(
        format!(
            "[TRS],[{}],[{}],[{}],\n",
            trs_count.breakfast, trs_count.lunch, trs_count.dinner
        )
        .as_str(),
    );
    template_str.push_str(
        format!(
            "[Welltest],[{}],[{}],[{}],\n",
            welltest_count.breakfast, welltest_count.lunch, welltest_count.dinner
        )
        .as_str(),
    );
    template_str.push_str(
        format!(
            "[Wireline],[{}],[{}],[{}],\n",
            wireline_count.breakfast, wireline_count.lunch, wireline_count.dinner
        )
        .as_str(),
    );
    template_str.push_str(
        r#"
          )
        "#,
    );

    template_str.push_str(format!("Report date: {}\n\n", date_str).as_str());
    template_str.push_str(format!("Breakfast total: {}\n\n", breakfast_total).as_str());
    template_str.push_str(format!("Lunch total: {}\n\n", lunch_total).as_str());
    template_str.push_str(format!("Dinner total: {}\n\n", dinner_total).as_str());
    template_str.push_str(format!("Dinner total: {}\n\n", dinner_total).as_str());

    let mut file = std::fs::File::options()
        .write(true)
        .truncate(true)
        .open("template.typ")?;
    file.write_all(template_str.as_bytes())?;

    let csv_date_str = datetime.naive_local().and_utc().format("%d_%m_%Y_%H_%M");
    let report_file_name = format!("report_{}.csv", csv_date_str);
    let mut csv_file = std::fs::File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(report_file_name)?;
    csv_file.write_all(csv_string.as_bytes())?;
    let _compile_cmd = std::process::Command::new("cmd")
        .args([
            "/C",
            format!("typst.exe compile template.typ {}.pdf", csv_date_str).as_str(),
        ])
        .output()?;
    let _pdf_open_cmd = std::process::Command::new("cmd")
        .args(["/C", format!("start {}.pdf", csv_date_str).as_str()])
        .output()?;

    Ok(())
}

fn reset_day(db_url: String) -> Result<()> {
    let pool = Pool::new(db_url.as_str())?;
    let mut conn = pool.get_conn()?;

    let _update_res = conn.exec_drop(
        format!(
            "UPDATE expro_employees
                                SET breakfast=0, lunch=0, dinner=0;
                                ",
        ),
        (),
    )?;
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
                SELECT id, name, department, title, expro_id, field, category, breakfast, lunch, dinner, last_timestamp FROM expro_employees
                WHERE id={};
            "#,
            app.input_result
            ),
            |(id, name, department, title, expro_id, field, category, breakfast, lunch, dinner, last_timestamp)| {
                Employee {
                    id,
                    name,
                    department,
                    title,
                    expro_id,
                    field,
                    category,
                    breakfast,
                    lunch,
                    dinner,
                    last_timestamp
                }
            }
    )?;

    if res.len() == 1 {
        let timestamp = chrono::Local::now().naive_local().and_utc().timestamp();
        let mut employee_query_result = res[0].clone();

        app.scanned_employee.employee_name = employee_query_result.name.clone();
        // We set the employee_is_scanned field to make the table scroll up.
        app.scroll_up = true;
        // During emergencies we no longer update the employee status
        // but we push the employee id into the emergency hash for counting.

        let duration_since = timestamp - employee_query_result.last_timestamp as i64;

        if duration_since >= 10 {
            match app.meal {
                Meal::Breakfast => {
                    if employee_query_result.breakfast == 0 {
                        let _update_res = conn.exec_drop(
                            format!(
                                "UPDATE expro_employees
                                SET breakfast=1, last_timestamp={}
                                WHERE id={}",
                                timestamp, employee_query_result.id
                            ),
                            (),
                        )?;

                        employee_query_result.breakfast = 1;
                        employee_query_result.last_timestamp = timestamp as usize;
                        app.employee_buffer.push(employee_query_result);
                    } else {
                        play_error_sound()?;
                        return Err(anyhow!("ID has already been scanned."));
                    }
                }
                Meal::Lunch => {
                    if employee_query_result.lunch == 0 {
                        let _update_res = conn.exec_drop(
                            format!(
                                "UPDATE expro_employees
                                SET lunch=1, last_timestamp={}
                                WHERE id={}",
                                timestamp, employee_query_result.id
                            ),
                            (),
                        )?;

                        employee_query_result.lunch = 1;
                        employee_query_result.last_timestamp = timestamp as usize;
                        app.employee_buffer.push(employee_query_result);
                    } else {
                        play_error_sound()?;
                        return Err(anyhow!("ID has already been scanned."));
                    }
                }
                Meal::Dinner => {
                    if employee_query_result.dinner == 0 {
                        let _update_res = conn.exec_drop(
                            format!(
                                "UPDATE expro_employees
                                SET dinner=1, last_timestamp={}
                                WHERE id={}",
                                timestamp, employee_query_result.id
                            ),
                            (),
                        )?;

                        employee_query_result.dinner = 1;
                        employee_query_result.last_timestamp = timestamp as usize;
                        app.employee_buffer.push(employee_query_result);
                    } else {
                        play_error_sound()?;
                        return Err(anyhow!("ID has already been scanned."));
                    }
                }
            }

            play_ok_sound()?;
        }
    } else if res.len() == 0 {
        play_error_sound()?;
        return Err(anyhow!("Could not find ID in the database."));
    } else {
        play_error_sound()?;
        return Err(anyhow!("More than one ID found in the database."));
    }
    Ok(())
}

// fn dump_24h_log_file(app: &mut TemplateApp, timestamp: i64) -> Result<()> {
//     if let Some(date_time) = DateTime::from_timestamp(timestamp, 0) {
//         let mut buffer = String::new();
//         let date = date_time.naive_local().and_utc().format("%d%m%Y_%H%M");
//         let file_name = format!("LOG_{}.txt", date);

//         for record in app.employee_buffer.iter() {
//             let mut line = String::new();
//             let time = DateTime::from_timestamp(record.last_timestamp as i64, 0)
//                 .unwrap()
//                 .format("%d-%m-%Y\t%H:%M:%S");
//             if record.in_base == 0 {
//                 line = format!(
//                     "{}\t{}\t{}\t{}\t{}\n",
//                     &record.name, &record.department, &record.title, "OUT", &time
//                 );
//             } else {
//                 line = format!(
//                     "{}\t{}\t{}\t{}\t{}\n",
//                     &record.name, &record.department, &record.title, "IN", &time
//                 );
//             }

//             buffer.push_str(line.as_str());
//         }

//         let mut file = std::fs::File::options()
//             .write(true)
//             .create(true)
//             .truncate(true)
//             .open(file_name)?;
//         file.write_all(buffer.as_bytes())?;
//         app.employee_buffer.clear();
//     }
//     Ok(())
// }

fn play_ok_sound() -> Result<()> {
    std::thread::spawn(move || {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

        let file = std::fs::File::open("assets/ok.mp3").unwrap();
        let beep1 = stream_handle
            .play_once(std::io::BufReader::new(file))
            .unwrap();
        beep1.set_volume(0.9);
        println!("Started beep1");
        std::thread::sleep(Duration::from_millis(1000));
    });

    Ok(())
}
fn play_error_sound() -> Result<()> {
    std::thread::spawn(move || {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

        let file = std::fs::File::open("assets/error.mp3").unwrap();
        let beep1 = stream_handle
            .play_once(std::io::BufReader::new(file))
            .unwrap();
        beep1.set_volume(0.9);
        println!("Started beep1");
        std::thread::sleep(Duration::from_millis(1000));
    });

    Ok(())
}
