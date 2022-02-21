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
    links: Vec<String>,
    selected: Option<usize>,
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
            ui.horizontal(|ui| {
                let response = ui.text_edit_singleline(link_input);
                if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                    if link_input.len() > 0 {
                        links.push(link_input.clone());
                        link_input.clear();
                    }
                    response.request_focus();
                }
                ui.add_enabled_ui(link_input.len() > 0, |ui| {
                    if ui
                        .button("+")
                        .on_disabled_hover_text("Type link before adding to list")
                        .on_hover_text("Save link to link list")
                        .clicked()
                    {
                        // Add a link
                        if link_input.len() > 0 {
                            links.push(link_input.clone());
                            link_input.clear();
                        }
                    }
                });
            });

            ui.separator();
            if !links.is_empty() {
                ui.heading(format!("Links ({}):", links.len()));
                ui.spacing();
                for (i, link) in links.iter().enumerate() {
                    if ui
                        .button(RichText::new(link).color(
                            if selected.map(|s| s == i).unwrap_or(false) {
                                Color32::RED
                            } else {
                                Color32::WHITE
                            },
                        ))
                        .clicked()
                    {
                        // Open link
                        *selected = Some(i);
                    }
                }
            } else {
                ui.label(RichText::new("Add a link to the list.").color(Color32::GRAY));
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if selected.is_some() {
                if let Some(link) = links.get(selected.unwrap()) {
                    ui.hyperlink(link);
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                        ui.horizontal(|ui| {
                            if ui.button("Copy").clicked() {
                                // Copy link to clipboard
                            }
                            if ui.button("Delete").clicked() {
                                // Delete link
                                links.remove(selected.unwrap());
                                *selected = None;
                            }
                        });
                    });
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
