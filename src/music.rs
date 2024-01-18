use crate::App;
use claxon::FlacReader;
use eframe::emath::{vec2, Pos2, Rect};
use eframe::epaint::{Color32, Stroke};
use egui::Ui;
use std::time::{Duration, Instant};

impl App {
    // audio processing from a FLAC file
    pub fn audio(&mut self) -> Result<(), anyhow::Error> {
        let fre = self.ui_top.amp_list.clone();
        let slider = self.ui_bottom.progress.clone();
        let paused = self.ui_bottom.paused.clone();

        let file_path = "bau.flac";

        self.runtime.spawn(async move {
            *paused.write().unwrap() = false;
            let file = std::fs::File::open(file_path).unwrap();
            let mut reader = FlacReader::new(file).unwrap();

            let display_rate = 60;
            let sample_rate = reader.streaminfo().sample_rate;
            let new_vec = reader
                .samples()
                .filter_map(Result::ok)
                .step_by((sample_rate / display_rate) as usize)
                .collect::<Vec<i32>>();

            let mut interval_duration = Duration::from_secs_f64(1.0 / display_rate as f64);

            interval_duration -= Duration::from_secs_f64(0.00835);
            loop {
                if *slider.read().unwrap() > 100. && !*paused.read().unwrap() {
                    *paused.write().unwrap() = true;
                    *slider.write().unwrap() = 0.;
                };
                if *paused.read().unwrap() {
                    tokio::time::sleep(Duration::from_secs_f64(1. / 120.)).await;
                    continue;
                };

                tokio::time::sleep(interval_duration).await;
                let start_time = Instant::now();

                let old_percentage = *slider.read().unwrap();

                *slider.write().unwrap() = old_percentage + (1. / new_vec.len() as f32) * 100.;

                if *slider.read().unwrap() >= 99.4999 {
                    continue;
                }
                let iter = *slider.read().unwrap() * new_vec.len() as f32 / 100.;

                // unsafe call
                let item = new_vec[iter as usize];

                let item = (item / 200) as f32;

                fre.write().unwrap().push(item);

                let time = start_time.elapsed().as_millis();

                if time > 20 {
                    println!("WARN {time}ms")
                }
            }
        });
        Ok(())
    }
}

// graphing data based on an array of Y coords
pub fn keter3(ui: &mut Ui, entries: &[f32]) {
    egui::Frame::canvas(ui.style())
        .fill(Color32::from_rgba_premultiplied(0, 0, 0, 0))
        .stroke(Stroke::new(
            0.0,
            Color32::from_rgba_premultiplied(0, 0, 0, 0),
        ))
        .show(ui, |ui| {
            ui.ctx().request_repaint();

            let desired_size = ui.available_width() * vec2(1.0, 0.35);
            let (_id, rect) = ui.allocate_space(desired_size);

            let mut lines = Vec::with_capacity(entries.len() + 2);

            let x_middle = rect.max.x;
            for y in entries.iter() {
                let y = rect.max.y + *y;

                lines.push(Pos2::new(x_middle, y));

                lines.iter_mut().for_each(|pos| pos.x -= 1.0);
            }

            lines = limiter(lines, rect);

            ui.painter().add(egui::Shape::line(
                lines,
                Stroke::new(2.0, Color32::from_gray(255)),
            ));
        });
}

fn limiter(values: Vec<Pos2>, rect: Rect) -> Vec<Pos2> {
    values
        .into_iter()
        .filter(|&Pos2 { x, y: _ }| x > rect.min.x)
        .map(|Pos2 { x, y }| Pos2 {
            x: x.clamp(rect.min.x, rect.max.x),
            y: y.clamp(rect.min.y, rect.max.y),
        })
        .collect()
}
