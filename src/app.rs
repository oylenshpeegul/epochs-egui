// use chrono::{NaiveDate, NaiveDateTime};
use chrono::NaiveDate;
use eframe::{egui, epi};
// use epochs;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum Epoch {
    Apfs,
    Java,
    Mozilla,
    Unix,
}

// Since Epoch is a simple C-like enum, we can iterate over its associated constant.
impl Epoch {
    const VALUES: [Self; 4] = [Self::Apfs, Self::Java, Self::Mozilla, Self::Unix];
}

impl fmt::Display for Epoch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Epoch::Apfs => write!(f, "{:30}", "APFS (nanoseconds)"),
            Epoch::Java => write!(f, "{:30}", "Java (milliseconds)"),
            Epoch::Mozilla => write!(f, "{:30}", "Mozilla (microseconds)"),
            Epoch::Unix => write!(f, "{:30}", "Unix (seconds)"),
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    datetime: String,
    number: i64,
    selected: Epoch,
    // this how you opt-out of serialization of a member
    // #[cfg_attr(feature = "persistence", serde(skip))]
    // value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            datetime: "1970-01-01 00:00:00".to_string(),
            number: 0,
            selected: Epoch::Unix,
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "epochs egui"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
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
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self {
            datetime,
            number,
            selected,
        } = self;

        let default_ndt = NaiveDate::from_ymd(1970, 1, 1).and_hms(0, 0, 0);
        *datetime = match selected {
            Epoch::Apfs => format!("{}", epochs::apfs(*number).unwrap_or(default_ndt)),
            Epoch::Java => format!("{}", epochs::java(*number).unwrap_or(default_ndt)),
            Epoch::Mozilla => format!("{}", epochs::mozilla(*number).unwrap_or(default_ndt)),
            Epoch::Unix => format!("{}", epochs::unix(*number).unwrap_or(default_ndt)),
            // _ => todo!(),
        };

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel is the region left after adding top and side panels.

            ui.heading("epochs egui");
            ui.hyperlink("https://github.com/oylenshpeegul/epochs-egui");
            egui::warn_if_debug_build(ui);

            // ui.add(egui::Slider::new(number, std::i64::MIN..=std::i64::MAX).text("number"));
            // ui.add(egui::DragValue::new(number).speed(1));
            ui.horizontal(|ui| {
                ui.label("Epoch value: ");
                ui.add(egui::DragValue::new(number).speed(1));
            });

            ui.horizontal(|ui| {
                ui.label("Epoch type: ");
                egui::ComboBox::from_id_source("")
                    .selected_text(format!("{}", selected))
                    .show_ui(ui, |ui| {
                        for e in Epoch::VALUES.iter().copied() {
                            ui.selectable_value(selected, e, format!("{}", e));
                        }
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Datetime: ");
                ui.text_edit_singleline(datetime);
            });
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
