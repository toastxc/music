use eframe::{HardwareAcceleration, Renderer};
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::runtime::Runtime;
mod music;
pub mod process;
pub mod view;

static MIN_WIDTH: f32 = 400.0;
static DEFAULT_WIDTH: f32 = 480.0;
static MIN_HEIGHT: f32 = 480.0;
static DEFAULT_HEIGHT: f32 = 480.0;

fn main() {
    let _guard = flame::start_guard("main");
    // env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([DEFAULT_WIDTH, DEFAULT_HEIGHT])
            .with_min_inner_size([MIN_WIDTH, MIN_HEIGHT])
            .with_transparent(true),
        vsync: false,
        hardware_acceleration: HardwareAcceleration::Preferred,
        renderer: Renderer::Glow,
        follow_system_theme: true,
        centered: false,
        ..Default::default()
    };

    eframe::run_native("App", options, Box::new(|_cc| Box::from(App::default()))).unwrap();
}

pub type Guard<T> = Arc<RwLock<T>>;

#[derive(Debug, Clone)]
struct App {
    pub runtime: Arc<Runtime>,

    pub ui_top: UiTop,

    pub ui_song_list: Vec<SongData>,

    pub ui_bottom: UiBottom,
}
#[derive(Debug, Clone)]
pub struct UiTop {
    // album
    pub album_art: Vec<u8>,
    pub album_name: String,
    pub artists: Vec<String>,
    // graph
    pub amp_list: Guard<Vec<f32>>,
    pub fre: Guard<f32>,
    // info
    pub play_length: Duration,
}

#[derive(Debug, Clone, Default)]
pub struct SongData {
    pub name: String,
    pub icon: Vec<u8>,
    pub artist: String,
    pub length: Duration,
    pub album: String,
}
#[derive(Debug, Clone)]
pub struct UiBottom {
    pub progress: Guard<f32>,
    pub paused: Guard<bool>,
    pub progress_request: Option<f32>,
    pub current_song_index: usize,
}

impl Default for UiBottom {
    fn default() -> Self {
        Self {
            progress: Arc::new(Default::default()),
            paused: Arc::new(RwLock::new(true)),
            progress_request: None,
            current_song_index: 0,
        }
    }
}
impl Default for UiTop {
    fn default() -> Self {
        Self {
            album_art: vec![],
            album_name: "Something for Everybody".to_string(),
            artists: vec!["Womp", "Womp"].into_iter().map(String::from).collect(),
            amp_list: Arc::new(Default::default()),
            fre: Arc::new(Default::default()),

            play_length: Duration::from_secs(400),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            runtime: Arc::new(Runtime::new().unwrap()),
            ui_top: UiTop::default(),
            ui_song_list: vec![
                SongData {
                    name: "Fresh".to_string(),
                    icon: vec![],
                    artist: "Devo".to_string(),
                    length: Duration::from_secs(400),
                    album: "Something for Everybody".to_string(),
                },
                SongData {
                    name: "What We Do".to_string(),
                    icon: vec![],
                    artist: "Devo".to_string(),
                    length: Duration::from_secs(290),
                    album: "Something for Everybody".to_string(),
                },
            ],

            ui_bottom: Default::default(),
        }
    }
}
