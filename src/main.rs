use eframe::egui;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;

struct CatClickerApp {
    texture: Option<egui::TextureHandle>,
    click_count: u64,
    click_time: Option<Instant>,
    audio_sinks: Arc<Mutex<Vec<Sink>>>,
    _stream: Option<OutputStream>,
    stream_handle: Option<OutputStreamHandle>,
}

impl Default for CatClickerApp {
    fn default() -> Self {
        let mut stream_out = None;
        let mut handle_out = None;

        if let Ok((stream, stream_handle)) = OutputStream::try_default() {
            stream_out = Some(stream);
            handle_out = Some(stream_handle);
        }

        Self {
            texture: None,
            click_count: 0,
            click_time: None,
            audio_sinks: Arc::new(Mutex::new(Vec::new())),
            _stream: stream_out,
            stream_handle: handle_out,
        }
    }
}

impl CatClickerApp {
    fn play_sound(&self) {
        if let Some(ref handle) = self.stream_handle {
            if let Ok(sink) = Sink::try_new(handle) {
                if let Ok(file) = File::open("src/meow.mp3") {
                    let reader = BufReader::new(file);
                    if let Ok(source) = Decoder::new(reader) {
                        sink.append(source);
                        if let Ok(mut sinks) = self.audio_sinks.lock() {
                            sinks.retain(|s| !s.empty());
                            sinks.push(sink);
                        }
                    }
                }
            }
        }
    }
}

impl eframe::App for CatClickerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.texture.is_none() {
            let img_path = Path::new("src/cat.png");
            if let Ok(dynamic_img) = image::open(img_path) {
                let size = [dynamic_img.width() as usize, dynamic_img.height() as usize];
                let rgba = dynamic_img.to_rgba8();
                let pixels = rgba.into_raw();
                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
                self.texture =
                    Some(ctx.load_texture("cat_picture", color_image, egui::TextureOptions::LINEAR));
            }
        }

        let mut scale = 1.0;
        if let Some(start_time) = self.click_time {
            let elapsed = start_time.elapsed().as_secs_f32();
            let duration = 0.12;
            if elapsed < duration {
                let progress = elapsed / duration;
                if progress < 0.5 {
                    scale = 1.0 - (progress * 2.0) * 0.12;
                } else {
                    scale = 0.88 + ((progress - 0.5) * 2.0) * 0.12;
                }
                ctx.request_repaint();
            } else {
                self.click_time = None;
            }
        }

        let frame = egui::Frame::none()
            .fill(egui::Color32::from_rgb(18, 18, 18))
            .inner_margin(20.0);

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new("cat clicker")
                        .size(36.0)
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new(format!("clicks: {}", self.click_count))
                        .size(28.0)
                        .strong()
                        .color(egui::Color32::from_rgb(255, 215, 0)),
                );
                ui.add_space(40.0);

                if let Some(ref texture) = self.texture {
                    let base_size = 500.0;
                    let current_size = base_size * scale;
                    let (rect, response) =
                        ui.allocate_exact_size(egui::vec2(base_size, base_size), egui::Sense::click());

                    let center = rect.center();
                    let scaled_min = egui::pos2(
                        center.x - current_size / 2.0,
                        center.y - current_size / 2.0,
                    );
                    let scaled_max = egui::pos2(
                        center.x + current_size / 2.0,
                        center.y + current_size / 2.0,
                    );
                    let scaled_rect = egui::Rect::from_min_max(scaled_min, scaled_max);

                    if ui.is_rect_visible(rect) {
                        let painter = ui.painter();
                        painter.image(
                            texture.id(),
                            scaled_rect,
                            egui::Rect::from_min_max(
                                egui::pos2(0.0, 0.0),
                                egui::pos2(1.0, 1.0),
                            ),
                            egui::Color32::WHITE,
                        );
                    }

                    if response.clicked() {
                        self.click_count += 1;
                        self.click_time = Some(Instant::now());
                        self.play_sound();
                    }
                } else {
                    ui.label(
                        egui::RichText::new("failed to load src/cat.png")
                            .color(egui::Color32::RED)
                            .size(18.0),
                    );
                }
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 800.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "cat clicker",
        options,
        Box::new(|_cc| Ok(Box::new(CatClickerApp::default()))),
    )
}
