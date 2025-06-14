#![warn(clippy::all, rust_2018_idioms)]

pub mod modules; // Добавляем модуль modules
mod app;
pub use app::TemplateApp;
