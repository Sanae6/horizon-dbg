mod engine;
pub mod ui;

use eframe::{App, CreationContext};
use egui::{CentralPanel, SidePanel};
use egui_dock::{DockArea, DockState, TabViewer};

use crate::ui::DebuggerApp;

fn main() {
  let native_options = eframe::NativeOptions::default();
  eframe::run_native(
    "My egui App",
    native_options,
    Box::new(|cc| Ok(Box::new(DebuggerApp::new(cc)))),
  )
  .unwrap();
}
