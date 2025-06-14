use crate::modules::json_io::JsonData;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    pub json_data: Option<JsonData>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self { json_data: None }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::widgets::global_theme_preference_buttons(ui);
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            if ui.button("Загрузить JSON").clicked() {
                match crate::modules::json_io::read_json("test.json") {
                    Ok(data) => {
                        self.json_data = Some(data);
                        ui.label("JSON успешно загружен!");
                    }
                    Err(err) => {
                        self.json_data = None;
                        ui.label(format!("Ошибка загрузки: {}", err));
                    }
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Данные из JSON:");
            if let Some(ref data) = self.json_data {
                ui.label(format!("{:?}", data));
            } else {
                ui.label("Нет данных");
            }

            ui.separator();
            if ui.button("Показать видеопоток").clicked() {
                try_show_video_player("http://localhost:8080/playlist.m3u8", "my-video-player");
            }
            if ui.button("Показать поток: Смешарики").clicked() {
                try_show_video_player("/streams/smeshariki/playlist.m3u8", "my-video-player-smeshariki");
            }
            if ui.button("Показать поток: Пони").clicked() {
                try_show_video_player("/streams/poni/playlist.m3u8", "my-video-player-poni");
            }
        });
    }
}

fn try_show_video_player(url: &str, id: &str) {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        show_video_player_with_id(url, id);
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = showVideoPlayer)]
    fn show_video_player_with_id(url: &str, id: &str);
}
