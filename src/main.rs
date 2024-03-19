// Disable terminal
#![windows_subsystem = "windows"]

use eframe::egui::{self, Color32, RichText, Slider, TextStyle::Body, Visuals};
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU32, Ordering};
use std::{fs::File, io::BufReader, io::BufWriter};

// Generate unique IDs for each todo item to ensure each item's uniqueness
static NEXT_ID: AtomicU32 = AtomicU32::new(1);

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TodoItem {
    id: u32,
    description: String,
    completed: bool,
    edit: bool,
}

struct TodoApp {
    items: Vec<TodoItem>,
    input: String,
    text_size: f32,    
    dark_mode: bool,  
    showing_add_item_input: bool,
}

impl Default for TodoApp {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            input: String::new(),
            text_size: 14.0,   
            dark_mode: false,
            showing_add_item_input: false,
        }
    }
}

impl TodoApp {
    fn save_to_file_dialog(&self) {
        // Filter for .json files
        if let Some(path) = FileDialog::new()
            .add_filter("JSON files", &["json"])
            .set_file_name("todo_list_save.json")
            .save_file() 
        {
            match File::create(path) {
                Ok(file) => {
                    let writer = BufWriter::new(file);
                    // Use to_writer_pretty for indented JSON
                    if serde_json::to_writer_pretty(writer, &self.items).is_err() {
                        eprintln!("Failed to write items to file.");
                    }
                }
                Err(e) => eprintln!("Failed to create file: {:?}", e),
            }
        }
    }

    fn load_from_file_dialog(&mut self) {
        // Filter for .json files
        if let Some(path) = FileDialog::new()
            .add_filter("JSON files", &["json"])
            .pick_file() 
        {
            match File::open(path) {
                Ok(file) => {
                    let reader = BufReader::new(file);
                    match serde_json::from_reader(reader) {
                        Ok(items) => {
                            self.items = items;
                            let max_id = self.items.iter().max_by_key(|item| item.id).map_or(0, |item| item.id);
                            NEXT_ID.store(max_id + 1, Ordering::SeqCst);
                        }
                        Err(e) => eprintln!("Failed to deserialize items: {:?}", e),
                    }
                }
                Err(e) => eprintln!("Failed to open file: {:?}", e),
            }
        }
    }
}


impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply the selected theme
        ctx.set_visuals(if self.dark_mode { Visuals::dark() } else { Visuals::light() });

        // Set text style based on the chosen text size
        let mut style: egui::Style = (*ctx.style()).clone();
        // Calculate scale factor based on default text size and chosen text size to scale the UI too
        let scale_factor = self.text_size / style.text_styles.get(&Body).unwrap().size;
        style.text_styles.iter_mut().for_each(|(_style, data)| {
            data.size *= scale_factor;
        });
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Todo List");

                // Load and Save buttons
                ui.horizontal(|ui| {
                    if ui.button("Load").clicked() {
                        self.load_from_file_dialog();
                    }
                    ui.add_space(20.0);
                    if ui.button("Save").clicked() {
                        self.save_to_file_dialog();
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("UI size:");
                    ui.add(Slider::new(&mut self.text_size, 6.0..=32.0).text(""));
                });

                ui.horizontal(|ui| {
                    ui.label("Theme:");
                    ui.checkbox(&mut self.dark_mode, "Dark");
                });

                ui.separator();

                let mut to_remove: Vec<usize> = Vec::new(); // Prepare a list to track items to remove
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (index, item) in self.items.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            
                            // Checkbox for completion status
                            ui.checkbox(&mut item.completed, "");

                            if !item.edit {
                                // If not in edit mode, show the description and the edit button
                                let text = if item.completed {
                                    // Apply strikethrough style if item is completed
                                    egui::RichText::new(&item.description).strikethrough()
                                } else {
                                    egui::RichText::new(&item.description)
                                };
                
                                ui.label(text);
                            } else {
                                // If in edit mode, show a text edit field
                                ui.text_edit_multiline(&mut item.description);
                            }
                            
                            // Right-align the edit and delete buttons
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                if item.edit {
                                    // If in edit mode, show a save button
                                    if ui.button(egui::RichText::new("✔").color(egui::Color32::DARK_GREEN)).clicked() {
                                        item.edit = false; // Disable edit mode after saving
                                    }
                                } else {
                                    // Show the edit button if not in edit mode
                                    if ui.button("Edit").clicked() {
                                        item.edit = true; // Enable edit mode
                                    }
                                }
                                
                                // Button for deletion
                                if ui.add(egui::Button::new(egui::RichText::new("❌").color(egui::Color32::RED))).clicked() {
                                    to_remove.push(index); // Mark this index for removal
                                }
                            });
                        });
                    }
                });

                // Remove items that were marked for deletion
                for &index in to_remove.iter().rev() { // Reverse iterate to avoid index shift
                    self.items.remove(index);
                }

                // Toggle the visibility of the add item input
                if !self.showing_add_item_input {
                    if ui.button("➕").clicked() {
                        self.showing_add_item_input = true;
                    }
                }

                // Conditionally show the add item input and button
                if self.showing_add_item_input {
                    ui.horizontal(|ui| {
                        ui.text_edit_multiline(&mut self.input);
                        if ui.button(egui::RichText::new("✔").color(egui::Color32::DARK_GREEN)).clicked() && !self.input.trim().is_empty() {
                            self.items.push(TodoItem {
                                id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
                                description: self.input.trim().to_string(),
                                completed: false,
                                edit: false,
                            });
                            self.input.clear();
                            self.showing_add_item_input = false;
                        }
                    });
                }
            });
        });
    }
}

fn main() {

    let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([400.0, 600.0])
                .with_drag_and_drop(true),
             ..Default::default()
        };

    if let Err(e) = eframe::run_native(
        "Todo List App",
        options,
        Box::new(|_cc| Box::new(TodoApp::default())),
    ) {
        eprintln!("Application error: {}", e);
    }
}
