use std::collections::HashMap;

use eframe::{
    egui::{self, Color32, RichText},
    epi,
};

#[derive(Default)]
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct AppState {
    // Example stuff:
    link_input: String,
    label_input: String,
    links: HashMap<String, Link>,
    selected: Option<String>,
}

#[derive(Default)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] //
pub struct Link {
    pub url: String,
    pub labels: Vec<String>,
}

impl Link {
    pub fn new(url: String) -> Self {
        Self {
            url,
            labels: Default::default(),
        }
    }

    pub fn add_label(&mut self, label: String) {
        self.labels.push(label);
    }
}

impl epi::App for AppState {
    fn name(&self) -> &str {
        "Linkser"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        let Self {
            link_input,
            links,
            selected,
            label_input,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Load").clicked() {
                        // Load from disk
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Add Link");
            ui.spacing();
            let mut add_link = None;
            ui.horizontal(|ui| {
                let response = ui.text_edit_singleline(link_input);
                if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                    add_link = Some(link_input.clone());
                    response.request_focus();
                }
                ui.add_enabled_ui(link_input.len() > 0, |ui| {
                    if ui
                        .button("+")
                        .on_disabled_hover_text("Type link before adding to list")
                        .on_hover_text("Save link to link list")
                        .clicked()
                    {
                        add_link = Some(link_input.clone());
                    }
                });
            });

            if let Some(new_link) = add_link {
                let new_link = new_link.trim().to_string();
                if new_link.len() > 0 && !links.contains_key(&new_link) {
                    links.insert(new_link.clone(), Link::new(new_link));
                    link_input.clear();
                }
            }

            ui.separator();
            if !links.is_empty() {
                ui.heading(format!("Links ({}):", links.len()));
                ui.spacing();
                for (url, link) in links.iter() {
                    if ui
                        .button(RichText::new(url).color(
                            if selected.clone().map(|s| &s == url).unwrap_or(false) {
                                Color32::RED
                            } else {
                                Color32::WHITE
                            },
                        ))
                        .on_hover_text(format!("Labels: {}", link.labels.join(", ")))
                        .clicked()
                    {
                        // Open link
                        *selected = Some(url.clone());
                    }
                }
            } else {
                ui.label(RichText::new("Add a link to the list.").color(Color32::GRAY));
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if selected.is_some() {
                if let Some(link) = links.get_mut(&selected.clone().unwrap()) {
                    let mut add_label = None;
                    let mut remove_link = None;
                    ui.horizontal(|ui| {
                        if ui.button("Copy").clicked() {
                            // Copy link to clipboard
                        }
                        if ui.button("Edit").clicked() {
                            // Copy link to clipboard
                        }
                        ui.separator();
                        if ui
                            .button(RichText::new("Delete").color(Color32::RED))
                            .clicked()
                        {
                            remove_link = Some(link.url.clone());
                        }
                    });
                    ui.separator();
                    ui.add(egui::Hyperlink::from_label_and_url(
                        RichText::new(link.url.clone()).heading(),
                        link.url.clone(),
                    ));
                    ui.label(RichText::new(link.labels.join(", ")).color(Color32::GOLD));

                    ui.separator();
                    ui.heading("Add Label:");
                    ui.spacing();
                    ui.horizontal(|ui| {
                        let response = ui.text_edit_singleline(label_input);
                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            add_label = Some(label_input.clone());
                            response.request_focus();
                        }
                        if ui.button("+").on_hover_text("Add label").clicked() {
                            add_label = Some(label_input.clone());
                        }
                    });

                    if let Some(new_label) = add_label {
                        let new_label = new_label.trim().to_string();
                        if new_label.len() > 0 && !link.labels.contains(&new_label) {
                            link.add_label(new_label.clone());
                            label_input.clear();
                        }
                    }

                    if let Some(to_remove_link) = remove_link {
                        links.remove(&to_remove_link);
                        *selected = None;
                    }
                } else {
                    ui.label(RichText::new("Link not found.").color(Color32::GRAY));
                }
            } else {
                ui.label(RichText::new("Select a link to display details.").color(Color32::GRAY));
            }
            // The central panel the region left after adding TopPanel's and SidePanel's
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
