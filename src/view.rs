use crate::App;
use egui::{Align, Context, Layout, ScrollArea, Slider, Style, Ui, Vec2};
use std::ops::RangeInclusive;

use egui::style::HandleShape;
use std::time::Duration;

use crate::music::keter3;

// calling the trait for eframe
impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);

        egui::CentralPanel::default().show(ctx, |ui| update_fn(self, ui, ctx));
    }
}

fn update_fn(value: &mut App, ui: &mut Ui, ctx: &egui::Context) {
    top_ui(ui, value, ctx);
    song_list(ui, value, ctx);
    ui.add_space(20.);
    bottom_ui(ui, value, ctx);
}

fn bottom_ui(ui: &mut Ui, value: &mut App, ctx: &Context) {
    // this is not a great way of doin things todo
    let space = ui.available_width() / 20.;
    ui.with_layout(Layout::left_to_right(Align::LEFT), |ui| {
        let mut style = Style::default();
        for (_text_style, font_id) in style.text_styles.iter_mut() {
            font_id.size = 40. // whatever size you want here
        }
        ui.set_style(style);
        ui.add_space(space);
        ui.button("⏮");
        ui.add_space(space);
        ui.button("🔀");
        ui.add_space(space);
        let paused = *value.ui_bottom.paused.read().unwrap();

        if ui
            .button(match paused {
                false => "⏸",
                true => "⏵",
            })
            .clicked()
        {
            *value.ui_bottom.paused.write().unwrap() = !paused;
        };
        ui.add_space(space);
        ui.button("🔁");
        ui.add_space(space);
        ui.button("⏭");
        ui.add_space(space);
        ui.set_style(Style::default());
    });

    ui.style_mut().spacing.slider_width = ui.available_width();
    ui.add(
        Slider::new(
            &mut *value.ui_bottom.progress.write().unwrap(),
            RangeInclusive::new(0., 100.),
        )
        .show_value(false)
        .handle_shape(HandleShape::Rect { aspect_ratio: 0.2 })
        .trailing_fill(true),
    );
}

fn song_list(ui: &mut Ui, value: &mut App, ctx: &Context) {
    ScrollArea::vertical().show(ui, |ui| {
        for item in value.ui_song_list.clone().into_iter() {
            ui.horizontal(|ui| {
                ui.allocate_ui_with_layout(
                    Vec2::new(ui.available_width() / 2., 10.0),
                    Layout::left_to_right(Align::Center),
                    |ui| {
                        ui.add_sized(
                            Vec2::new(IMG_SIZE / 2., IMG_SIZE / 2.),
                            egui::Image::new(egui::include_image!("../img.png")).rounding(5.),
                        );
                        ui.vertical(|ui| {
                            text_big(ui, &item.name, 20.);
                            ui.label(&item.artist);
                        });
                    },
                );

                ui.allocate_ui_with_layout(
                    Vec2::new(ui.available_width(), 10.0),
                    Layout::right_to_left(Align::RIGHT),
                    |ui| {
                        text_big(ui, time_display(item.length), 20.);
                        text_big(ui, &item.album, 20.);
                    },
                );
            });
            ui.separator();
        }
    });
}

fn text_big(ui: &mut Ui, label: impl Into<String>, font_size: f32) {
    let mut style = Style::default();
    for (_text_style, font_id) in style.text_styles.iter_mut() {
        font_id.size = font_size // whatever size you want here
    }
    ui.set_style(style);
    ui.label(label.into());
    ui.set_style(Style::default());
}

const IMG_SIZE: f32 = 100.;

fn top_ui(ui: &mut Ui, value: &mut App, _ctx: &egui::Context) {
    if ui.button("moosic").clicked() {
        let _ = value.audio();
    }

    ui.horizontal(|ui| {
        ui.add_sized(
            Vec2::new(IMG_SIZE, IMG_SIZE),
            egui::Image::new(egui::include_image!("../img.png")).rounding(5.),
        );
        ui.vertical(|ui| {
            text_big(ui, &value.ui_top.album_name, 24.0);

            ui.label(value.ui_top.artists.len().to_string() + " Artists");

            ui.label(value.ui_song_list.len().to_string() + " Songs");

            ui.label(time_display(value.ui_top.play_length));
        });
    });
    // weird barely functional visualiser
    keter3(ui, &value.ui_top.amp_list.read().unwrap().clone());
}

fn time_display(duration: Duration) -> String {
    let mut secs = duration.as_secs();
    let mut mins = 0;
    let mut hours = 0;

    loop {
        if secs >= 3600 {
            secs -= 3600;
            hours += 1;
        } else if secs >= 60 {
            secs -= 60;
            mins += 1;
        } else {
            break;
        }
    }
    format!("{hours}:{mins}:{secs}")
}
