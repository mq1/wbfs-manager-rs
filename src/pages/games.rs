// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-2.0-only

use eframe::egui::{self, FontId, RichText};
use poll_promise::Promise;

use crate::app::App;

pub fn view(ctx: &egui::Context, app: &mut App) {
    let drive = app.current_drive.clone().unwrap();

    let drive_cloned = drive.clone();
    let promise = app.games.get_or_insert_with(|| {
        Promise::spawn_thread("get_games", move || drive_cloned.get_games())
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading(&drive.name);

        ui.add_space(10.0);

        match promise.ready_mut() {
            None => {
                ui.spinner();
            }
            Some(Err(err)) => {
                ui.label(&format!("Error: {}", err));
            }
            Some(Ok(games)) => {
                ui.horizontal(|ui| {
                    if ui.button("🗑 Delete selected").clicked() {}

                    if ui.button("➕ Add games").clicked() {}

                    if ui.button("✅ Select all").clicked() {
                        for game in games.iter_mut() {
                            game.checked = true;
                        }
                    }

                    if ui.button("❌ Deselect all").clicked() {
                        for game in games.iter_mut() {
                            game.checked = false;
                        }
                    }
                });

                ui.separator();

                egui_extras::TableBuilder::new(ui)
                    .striped(true)
                    .column(
                        egui_extras::Column::auto_with_initial_suggestion(1000.).resizable(true),
                    )
                    .column(egui_extras::Column::remainder())
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.label(RichText::new("🎮 Game").font(FontId::proportional(16.0)));
                        });
                        header.col(|ui| {
                            ui.label(RichText::new("📁 Size").font(FontId::proportional(16.0)));
                        });
                    })
                    .body(|mut body| {
                        for game in games.iter_mut() {
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.checkbox(&mut game.checked, game.display_title.clone());
                                });
                                row.col(|ui| {
                                    ui.label(format!(
                                        "{:.2} GiB",
                                        game.size as f32 / 1073741824.
                                    ));
                                });
                            });
                        }
                    });
            }
        }
    });
}
