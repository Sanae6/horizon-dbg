use eframe::{App, CreationContext};
use egui::{CentralPanel, SidePanel};

fn main() {
  let native_options = eframe::NativeOptions::default();
  eframe::run_native(
    "My egui App",
    native_options,
    Box::new(|cc| Ok(Box::new(DebuggerApp::new(cc)))),
  )
  .unwrap();
}

struct DebuggerApp {}

impl App for DebuggerApp {
  fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
  }
}

impl DebuggerApp {
  pub fn new(cc: &CreationContext<'_>) -> DebuggerApp {
    DebuggerApp {}
  }
}

