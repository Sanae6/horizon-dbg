mod engine;
pub mod ui;

use crate::ui::DebuggerApp;

fn main() {
  let native_options = eframe::NativeOptions::default();
  eframe::run_native(
    "horizon-dbg",
    native_options,
    Box::new(|cc| Ok(Box::new(DebuggerApp::new(cc)))),
  )
  .unwrap();
}
